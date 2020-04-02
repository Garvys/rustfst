use failure::Fallible;

use crate::algorithms::compose::lookahead_matchers::LookaheadMatcher;
use crate::algorithms::compose::matchers::{MatchType, Matcher, MatcherFlags};
use crate::fst_traits::{ExpandedFst, Fst};
use crate::semirings::Semiring;
use crate::{Arc, Label, StateId, NO_STATE_ID};
use failure::_core::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TrivialLookAheadMatcher<W, M> {
    matcher: M,
    lookahead_weight: W,
    prefix_arc: Arc<W>,
}

impl<'fst, W: Semiring + 'fst, M: Matcher<'fst, W>> Matcher<'fst, W>
    for TrivialLookAheadMatcher<W, M>
{
    type F = M::F;
    type Iter = M::Iter;

    fn new(fst: &'fst Self::F, match_type: MatchType) -> Fallible<Self> {
        Ok(Self {
            matcher: M::new(fst, match_type)?,
            prefix_arc: Arc::new(0, 0, W::one(), NO_STATE_ID),
            lookahead_weight: W::one(),
        })
    }

    fn iter(&self, state: usize, label: usize) -> Fallible<Self::Iter> {
        self.matcher.iter(state, label)
    }

    fn final_weight(&self, state: usize) -> Fallible<Option<&'fst W>> {
        self.matcher.final_weight(state)
    }

    fn match_type(&self) -> MatchType {
        self.matcher.match_type()
    }

    fn flags(&self) -> MatcherFlags {
        self.matcher.flags()
            | MatcherFlags::INPUT_LOOKAHEAD_MATCHER
            | MatcherFlags::OUTPUT_LOOKAHEAD_MATCHER
    }

    fn priority(&self, state: usize) -> Fallible<usize> {
        self.matcher.priority(state)
    }

    fn fst(&self) -> &'fst Self::F {
        self.matcher.fst()
    }
}

impl<'fst, W: Semiring + 'fst, M: Matcher<'fst, W>> LookaheadMatcher<'fst, W>
    for TrivialLookAheadMatcher<W, M>
{
    type MatcherData = ();

    fn data(&self) -> Option<&Rc<RefCell<Self::MatcherData>>> {
        None
    }

    fn new_with_data(
        fst: &'fst Self::F,
        match_type: MatchType,
        _data: Option<Rc<RefCell<Self::MatcherData>>>,
    ) -> Fallible<Self> {
        Self::new(fst, match_type)
    }

    fn create_data(
        _fst: &Self::F,
        _match_type: MatchType,
    ) -> Fallible<Option<Rc<RefCell<Self::MatcherData>>>> {
        Ok(None)
    }

    fn init_lookahead_fst<LF: ExpandedFst<W = W>>(&mut self, _lfst: &LF) -> Fallible<()> {
        Ok(())
    }

    fn lookahead_fst<LF: Fst<W = W>>(
        &mut self,
        _matcher_state: StateId,
        _lfst: &LF,
        _s: StateId,
    ) -> Fallible<bool> {
        Ok(true)
    }

    fn lookahead_label(&self, _state: StateId, _label: Label) -> Fallible<bool> {
        Ok(true)
    }

    fn lookahead_prefix(&self, _arc: &mut Arc<W>) -> bool {
        false
    }

    fn lookahead_weight(&self) -> &W {
        &self.lookahead_weight
    }

    fn prefix_arc(&self) -> &Arc<W> {
        &self.prefix_arc
    }

    fn prefix_arc_mut(&mut self) -> &mut Arc<W> {
        &mut self.prefix_arc
    }

    fn lookahead_weight_mut(&mut self) -> &mut W {
        &mut self.lookahead_weight
    }
}