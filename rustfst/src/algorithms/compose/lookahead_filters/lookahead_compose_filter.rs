use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use failure::Fallible;

use crate::algorithms::compose::compose_filters::ComposeFilter;
use crate::algorithms::compose::filter_states::FilterState;
use crate::algorithms::compose::lookahead_filters::lookahead_selector::{
    selector, MatchTypeTrait, Selector,
};
use crate::algorithms::compose::lookahead_filters::{
    lookahead_match_type, LookAheadComposeFilterTrait,
};
use crate::algorithms::compose::lookahead_matchers::LookaheadMatcher;
use crate::algorithms::compose::matchers::MatcherFlags;
use crate::algorithms::compose::matchers::{MatchType, Matcher};
use crate::semirings::Semiring;
use crate::{Arc, EPS_LABEL};

#[derive(Debug)]
pub struct LookAheadComposeFilter<
    'fst1,
    'fst2,
    W: Semiring + 'fst1 + 'fst2,
    CF: LookAheadComposeFilterTrait<'fst1, 'fst2, W>,
    SMT: MatchTypeTrait,
> where
    CF::M1: LookaheadMatcher<'fst1, W>,
    CF::M2: LookaheadMatcher<'fst2, W>,
{
    filter: CF,
    lookahead_type: MatchType,
    flags: MatcherFlags,
    lookahead_arc: bool,
    smt: PhantomData<&'fst1 SMT>,
    w: PhantomData<&'fst2 W>,
    selector: Selector<'fst1, 'fst2, W, CF::M1, CF::M2>,
}

impl<
        'fst1,
        'fst2,
        W: Semiring + 'fst1 + 'fst2,
        CF: LookAheadComposeFilterTrait<'fst1, 'fst2, W>,
        SMT: MatchTypeTrait,
    > LookAheadComposeFilter<'fst1, 'fst2, W, CF, SMT>
where
    CF::M1: LookaheadMatcher<'fst1, W>,
    CF::M2: LookaheadMatcher<'fst2, W>,
{
    fn lookahead_filter_arc(
        &mut self,
        arca: &mut Arc<W>,
        arcb: &mut Arc<W>,
        fs: &CF::FS,
    ) -> Fallible<CF::FS> {
        println!("Arc a : {:?}", arca);
        println!("Arc b : {:?}", arcb);

        let labela = if self.lookahead_output() {
            arca.olabel
        } else {
            arca.ilabel
        };
        if labela != EPS_LABEL && !self.flags.contains(MatcherFlags::LOOKAHEAD_NON_EPSILONS) {
            println!("Early exit 1");
            return Ok(fs.clone());
        }
        if labela == EPS_LABEL && !self.flags.contains(MatcherFlags::LOOKAHEAD_EPSILONS) {
            println!("Early exit 2");
            return Ok(fs.clone());
        }
        self.lookahead_arc = true;

        let res = match self.selector() {
            Selector::MatchInput(s) => {
                s.matcher
                    .borrow_mut()
                    .lookahead_fst(arca.nextstate, s.fst, arcb.nextstate)?
            }
            Selector::MatchOutput(s) => {
                s.matcher
                    .borrow_mut()
                    .lookahead_fst(arca.nextstate, s.fst, arcb.nextstate)?
            }
        };

        if res {
            Ok(fs.clone())
        } else {
            Ok(CF::FS::new_no_state())
        }
    }
}

impl<
        'fst1,
        'fst2,
        W: Semiring + 'fst1 + 'fst2,
        CF: LookAheadComposeFilterTrait<'fst1, 'fst2, W>,
        SMT: MatchTypeTrait,
    > ComposeFilter<'fst1, 'fst2, W> for LookAheadComposeFilter<'fst1, 'fst2, W, CF, SMT>
where
    CF::M1: LookaheadMatcher<'fst1, W>,
    CF::M2: LookaheadMatcher<'fst2, W>,
{
    type M1 = CF::M1;
    type M2 = CF::M2;
    type FS = CF::FS;

    fn new<IM1: Into<Option<Rc<RefCell<Self::M1>>>>, IM2: Into<Option<Rc<RefCell<Self::M2>>>>>(
        fst1: &'fst1 <Self::M1 as Matcher<'fst1, W>>::F,
        fst2: &'fst2 <Self::M2 as Matcher<'fst2, W>>::F,
        m1: IM1,
        m2: IM2,
    ) -> Fallible<Self> {
        let filter = CF::new(fst1, fst2, m1, m2)?;
        let lookahead_type = if SMT::match_type() == MatchType::MatchBoth {
            lookahead_match_type(filter.matcher1(), filter.matcher2())
        } else {
            SMT::match_type()
        };

        let flags = if std::dbg!(lookahead_type == MatchType::MatchOutput) {
            filter.matcher1().borrow().flags()
        } else {
            filter.matcher2().borrow().flags()
        };
        std::dbg!(flags);

        if lookahead_type == MatchType::MatchNone {
            bail!(
                "LookAheadComposeFilter: 1st argument cannot match/look-ahead on output \
            labels and 2nd argument cannot match/look-ahead on input labels"
            )
        }

        let mut selector = selector(
            filter.matcher1(),
            filter.matcher2(),
            SMT::match_type(),
            lookahead_type,
        );

        match &mut selector {
            Selector::MatchInput(l) => l.matcher.borrow_mut().init_lookahead_fst(l.fst)?,
            Selector::MatchOutput(l) => l.matcher.borrow_mut().init_lookahead_fst(l.fst)?,
        };

        Ok(Self {
            lookahead_type,
            flags,
            smt: PhantomData,
            lookahead_arc: false,
            w: PhantomData,
            selector,
            filter,
        })
    }

    fn start(&self) -> Self::FS {
        self.filter.start()
    }

    fn set_state(&mut self, s1: usize, s2: usize, filter_state: &Self::FS) -> Fallible<()> {
        self.filter.set_state(s1, s2, filter_state)
    }

    fn filter_arc(&mut self, arc1: &mut Arc<W>, arc2: &mut Arc<W>) -> Fallible<Self::FS> {
        self.lookahead_arc = false;
        let fs = std::dbg!(self.filter.filter_arc(arc1, arc2)?);
        if fs == CF::FS::new_no_state() {
            return Ok(CF::FS::new_no_state());
        }
        if std::dbg!(self.lookahead_output()) {
            self.lookahead_filter_arc(arc1, arc2, &fs)
        } else {
            self.lookahead_filter_arc(arc2, arc1, &fs)
        }
    }

    fn filter_final(&self, w1: &mut W, w2: &mut W) -> Fallible<()> {
        self.filter.filter_final(w1, w2)
    }

    fn matcher1(&self) -> Rc<RefCell<Self::M1>> {
        self.filter.matcher1()
    }

    fn matcher2(&self) -> Rc<RefCell<Self::M2>> {
        self.filter.matcher2()
    }
}

impl<
        'fst1,
        'fst2,
        W: Semiring + 'fst1 + 'fst2,
        CF: LookAheadComposeFilterTrait<'fst1, 'fst2, W>,
        SMT: MatchTypeTrait,
    > LookAheadComposeFilterTrait<'fst1, 'fst2, W>
    for LookAheadComposeFilter<'fst1, 'fst2, W, CF, SMT>
where
    CF::M1: LookaheadMatcher<'fst1, W>,
    CF::M2: LookaheadMatcher<'fst2, W>,
{
    fn lookahead_flags(&self) -> MatcherFlags {
        self.flags
    }

    fn lookahead_arc(&self) -> bool {
        self.lookahead_arc
    }

    fn lookahead_type(&self) -> MatchType {
        self.lookahead_type
    }

    fn lookahead_output(&self) -> bool {
        if SMT::match_type() == MatchType::MatchOutput {
            true
        } else if SMT::match_type() == MatchType::MatchInput {
            false
        } else if self.lookahead_type == MatchType::MatchOutput {
            true
        } else {
            false
        }
    }

    fn selector(&self) -> &Selector<'fst1, 'fst2, W, Self::M1, Self::M2> {
        &self.selector
    }
}