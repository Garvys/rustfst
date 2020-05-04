use std::fmt::Debug;

use crate::semirings::Semiring;
use crate::Tr;
use crate::EPS_LABEL;

/// Base trait to restrict which arcs are traversed in an FST.
pub trait TrFilter<S: Semiring>: Clone + Debug + PartialEq {
    /// If true, Tr should be kept, else Tr should be ignored.
    fn keep(&self, arc: &Tr<S>) -> bool;
}

/// True for all arcs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnyTrFilter {}

impl<S: Semiring> TrFilter<S> for AnyTrFilter {
    fn keep(&self, _tr: &Tr<S>) -> bool {
        true
    }
}

/// True for (input/output) epsilon arcs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpsilonTrFilter {}

impl<S: Semiring> TrFilter<S> for EpsilonTrFilter {
    fn keep(&self, arc: &Tr<S>) -> bool {
        arc.ilabel == EPS_LABEL && arc.olabel == EPS_LABEL
    }
}

/// True for input epsilon arcs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputEpsilonTrFilter {}

impl<S: Semiring> TrFilter<S> for InputEpsilonTrFilter {
    fn keep(&self, arc: &Tr<S>) -> bool {
        arc.ilabel == EPS_LABEL
    }
}

/// True for output epsilon arcs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputEpsilonTrFilter {}

impl<S: Semiring> TrFilter<S> for OutputEpsilonTrFilter {
    fn keep(&self, arc: &Tr<S>) -> bool {
        arc.olabel == EPS_LABEL
    }
}
