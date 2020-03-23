use crate::{Arc, StateId};
use crate::algorithms::dfs_visit::Visitor;
use crate::algorithms::lookahead_matchers::interval_set::{IntervalSet, IntInterval};
use crate::fst_traits::{CoreFst, Fst};

static UNASSIGNED: usize = std::usize::MAX;

pub struct IntervalReachVisitor<'a, F> {
    fst: &'a F,
    pub(crate) isets: Vec<IntervalSet>,
    pub(crate) state2index: Vec<usize>,
    index: usize,
}

impl<'a, F> IntervalReachVisitor<'a, F> {
    pub fn new(fst: &'a F) -> Self {
        Self {
            fst,
            isets: vec![],
            state2index: vec![],
            index: 1,
        }
    }
}

impl<'a, F: Fst> Visitor<'a, F> for IntervalReachVisitor<'a, F> {
    /// Invoked before DFS visit.
    fn init_visit(&mut self, fst: &'a F) {}

    /// Invoked when state discovered (2nd arg is DFS tree root).
    fn init_state(&mut self, s: StateId, root: StateId) -> bool {
        while self.isets.len() <= s {
            self.isets.push(IntervalSet::default());
        }
        while self.state2index.len() <= s {
            self.state2index.push(UNASSIGNED);
        }
        if self.fst.is_final(s).unwrap() {
            let intervals = &mut self.isets[s].intervals.intervals;
            if self.index == UNASSIGNED {
                if self.fst.num_arcs(s).unwrap() > 0 {
                    panic!("IntervalReachVisitor: state2index map must be empty for this FST")
                }
                let index = self.state2index[s];
                if index == UNASSIGNED {
                    panic!("IntervalReachVisitor: state2index map incomplete")
                }
                intervals.push(IntInterval::new(index, index + 1));
            } else {
                intervals.push(IntInterval::new(self.index, self.index + 1));
                self.state2index[s] = self.index;
                self.index += 1;
            }
        }
        true
    }

    /// Invoked when tree arc to white/undiscovered state examined.
    fn tree_arc(&mut self, s: StateId, arc: &Arc<F::W>) -> bool {
        true
    }

    /// Invoked when back arc to grey/unfinished state examined.
    fn back_arc(&mut self, s: StateId, arc: &Arc<F::W>) -> bool {
        panic!("Cyclic input")
    }

    /// Invoked when forward or cross arc to black/finished state examined.
    fn forward_or_cross_arc(&mut self, s: StateId, arc: &Arc<F::W>) -> bool {
        union_vec_isets_unordered(&mut self.isets, s, arc.nextstate);
        true
    }

    /// Invoked when state finished ('s' is tree root, 'parent' is kNoStateId,
    /// and 'arc' is nullptr).
    fn finish_state(&mut self, s: StateId, parent: Option<StateId>, arc: Option<&Arc<F::W>>) {
        if self.index != UNASSIGNED && self.fst.is_final(s).unwrap() {
            let intervals = &mut self.isets[s].intervals.intervals;
            intervals[0].end = self.index;
        }
        self.isets[s].normalize();
        if let Some(p) = parent {
            union_vec_isets_unordered(&mut self.isets, p, s);
        }
    }

    /// Invoked after DFS visit.
    fn finish_visit(&mut self) {}
}

fn union_vec_isets_ordered(isets: &mut Vec<IntervalSet>, i_inf: usize, i_sup: usize) {
    let (v_0_isupm1, v_isup1_end) = isets.split_at_mut(i_sup);
    v_0_isupm1[i_inf].union(&v_isup1_end[0])
}

// Perform the union of two IntervalSet stored in a vec. Utils to fix issue with borrow checker.
fn union_vec_isets_unordered(isets: &mut Vec<IntervalSet>, i: usize, j: usize) {
    if i < j {
        union_vec_isets_ordered(isets, i, j)
    } else if j > i {
        union_vec_isets_ordered(isets, j, i)
    } else {
        // Useless
        unreachable!()
    }
}