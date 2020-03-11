use failure::Fallible;

use crate::algorithms::compose_filters::ComposeFilter;
use crate::algorithms::filter_states::FilterState;
use crate::algorithms::filter_states::TrivialFilterState;
use crate::algorithms::matchers::{MatchType, Matcher};
use crate::semirings::Semiring;
use crate::{Arc, NO_LABEL};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct NullComposeFilter<M1, M2> {
    matcher1: Rc<RefCell<M1>>,
    matcher2: Rc<RefCell<M2>>,
}

impl<'fst, W: Semiring + 'fst, M1: Matcher<'fst, W>, M2: Matcher<'fst, W>> ComposeFilter<'fst, W>
    for NullComposeFilter<M1, M2>
{
    type M1 = M1;
    type M2 = M2;
    type FS = TrivialFilterState;

    fn new<IM1: Into<Option<Self::M1>>, IM2: Into<Option<Self::M2>>>(
        fst1: &'fst <Self::M1 as Matcher<'fst, W>>::F,
        fst2: &'fst <Self::M2 as Matcher<'fst, W>>::F,
        m1: IM1,
        m2: IM2,
    ) -> Fallible<Self> {
        Ok(Self {
            matcher1: Rc::new(RefCell::new(
                m1.into()
                    .unwrap_or_else(|| M1::new(fst1, MatchType::MatchOutput).unwrap()),
            )),
            matcher2: Rc::new(RefCell::new(
                m2.into()
                    .unwrap_or_else(|| M2::new(fst2, MatchType::MatchInput).unwrap()),
            )),
        })
    }

    fn start(&self) -> Self::FS {
        Self::FS::new(true)
    }

    fn set_state(&mut self, _s1: usize, _s2: usize, _filter_state: &Self::FS) {}

    fn filter_arc(&self, arc1: &mut Arc<W>, arc2: &mut Arc<W>) -> Option<Self::FS> {
        if arc1.olabel == NO_LABEL || arc2.ilabel == NO_LABEL {
            None
        } else {
            Some(Self::FS::new(true))
        }
    }

    fn filter_final(&self, _w1: &mut W, _w2: &mut W) {}

    fn matcher1(&self) -> Rc<RefCell<Self::M1>> {
        Rc::clone(&self.matcher1)
    }

    fn matcher2(&self) -> Rc<RefCell<Self::M2>> {
        Rc::clone(&self.matcher2)
    }
}