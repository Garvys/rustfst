use std::collections::VecDeque;

use crate::fst_path::FstPath;
use crate::fst_traits::Fst;
use crate::semirings::Semiring;
use crate::StateId;

/// Trait to iterate over the paths accepted by an FST.
pub trait PathsIterator<'a> {
    type W: Semiring;
    type Iter: Iterator<Item = FstPath<Self::W>>;
    fn paths_iter(&'a self) -> Self::Iter;
}

impl<'a, F> PathsIterator<'a> for F
where
    F: 'a + Fst,
{
    type W = F::W;
    type Iter = StructPathsIterator<'a, F>;
    fn paths_iter(&'a self) -> Self::Iter {
        StructPathsIterator::new(&self)
    }
}

pub struct StructPathsIterator<'a, F>
where
    F: 'a + Fst,
{
    fst: &'a F,
    queue: VecDeque<(StateId, FstPath<F::W>)>,
}

impl<'a, F> StructPathsIterator<'a, F>
where
    F: 'a + Fst,
{
    pub fn new(fst: &'a F) -> Self {
        let mut queue = VecDeque::new();

        if let Some(state_start) = fst.start() {
            queue.push_back((state_start, FstPath::default()));
        }

        StructPathsIterator { fst, queue }
    }
}

impl<'a, F> Iterator for StructPathsIterator<'a, F>
where
    F: 'a + Fst,
{
    type Item = FstPath<F::W>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let (state_id, mut path) = self.queue.pop_front().unwrap();

            for arc in self.fst.arcs_iter(state_id).unwrap() {
                let mut new_path = path.clone();
                new_path
                    .add_to_path(arc.ilabel, arc.olabel, arc.weight.clone())
                    .expect("Error add_to_path in PathsIterator");
                self.queue.push_back((arc.nextstate, new_path));
            }

            if let Some(final_weight) = self.fst.final_weight(state_id) {
                path.add_weight(final_weight)
                    .expect("Error add_weight in PathsIterator");
                return Some(path);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use counter::Counter;

    use crate::arc::Arc;
    use crate::fst_impls::VectorFst;
    use crate::fst_traits::MutableFst;
    use crate::semirings::{IntegerWeight, Semiring};
    use crate::utils::acceptor;

    #[test]
    fn test_paths_iterator_empty_fst() {
        let fst = VectorFst::<IntegerWeight>::new();

        assert_eq!(fst.paths_iter().count(), 0);
    }

    #[test]
    fn test_paths_iterator_single_state_start_and_final() {
        let mut fst = VectorFst::<IntegerWeight>::new();

        let s = fst.add_state();
        fst.set_start(s).unwrap();
        fst.set_final(s, IntegerWeight::one()).unwrap();

        let paths: Counter<_> = fst.paths_iter().collect();

        let mut paths_ref: Counter<_> = Counter::new();
        paths_ref.update(vec![FstPath::default()]);

        assert_eq!(paths, paths_ref);
    }

    #[test]
    fn test_paths_iterator_linear_fst() {
        let labels = vec![153, 45, 96];

        let fst: VectorFst<IntegerWeight> = acceptor(&labels, IntegerWeight::one());

        assert_eq!(fst.paths_iter().count(), 1);

        for path in fst.paths_iter() {
            assert_eq!(
                path,
                FstPath::new(labels.clone(), labels.clone(), IntegerWeight::one())
            );
        }
    }

    #[test]
    fn test_paths_iterator_small_fst_one_final_state() {
        let mut fst: VectorFst<IntegerWeight> = VectorFst::new();

        let s1 = fst.add_state();
        let s2 = fst.add_state();
        let s3 = fst.add_state();
        let s4 = fst.add_state();

        fst.set_start(s1).unwrap();
        fst.set_final(s4, IntegerWeight::new(18)).unwrap();

        fst.add_arc(s1, Arc::new(1, 1, IntegerWeight::new(1), s2))
            .unwrap();
        fst.add_arc(s1, Arc::new(2, 2, IntegerWeight::new(2), s3))
            .unwrap();
        fst.add_arc(s1, Arc::new(3, 3, IntegerWeight::new(3), s4))
            .unwrap();
        fst.add_arc(s2, Arc::new(4, 4, IntegerWeight::new(4), s4))
            .unwrap();
        fst.add_arc(s3, Arc::new(5, 5, IntegerWeight::new(5), s4))
            .unwrap();

        assert_eq!(fst.paths_iter().count(), 3);

        let mut paths_ref = Counter::new();
        paths_ref.update(vec![FstPath::new(
            vec![1, 4],
            vec![1, 4],
            IntegerWeight::new(4 * 18),
        )]);
        paths_ref.update(vec![FstPath::new(
            vec![2, 5],
            vec![2, 5],
            IntegerWeight::new(10 * 18),
        )]);
        paths_ref.update(vec![FstPath::new(
            vec![3],
            vec![3],
            IntegerWeight::new(3 * 18),
        )]);

        let paths: Counter<_> = fst.paths_iter().collect();

        assert_eq!(paths_ref, paths);
    }

    #[test]
    fn test_paths_iterator_small_fst_multiple_final_states() {
        let mut fst: VectorFst<IntegerWeight> = VectorFst::new();

        let s1 = fst.add_state();
        let s2 = fst.add_state();
        let s3 = fst.add_state();
        let s4 = fst.add_state();

        fst.set_start(s1).unwrap();
        fst.set_final(s1, IntegerWeight::new(38)).unwrap();
        fst.set_final(s2, IntegerWeight::new(41)).unwrap();
        fst.set_final(s3, IntegerWeight::new(53)).unwrap();
        fst.set_final(s4, IntegerWeight::new(185)).unwrap();

        fst.add_arc(s1, Arc::new(1, 1, IntegerWeight::new(1), s2))
            .unwrap();
        fst.add_arc(s1, Arc::new(2, 2, IntegerWeight::new(2), s3))
            .unwrap();
        fst.add_arc(s1, Arc::new(3, 3, IntegerWeight::new(3), s4))
            .unwrap();
        fst.add_arc(s2, Arc::new(4, 4, IntegerWeight::new(4), s4))
            .unwrap();
        fst.add_arc(s3, Arc::new(5, 5, IntegerWeight::new(5), s4))
            .unwrap();

        assert_eq!(fst.paths_iter().count(), 6);

        let mut paths_ref = Counter::new();
        paths_ref.update(vec![FstPath::new(vec![], vec![], IntegerWeight::new(38))]);
        paths_ref.update(vec![FstPath::new(
            vec![1],
            vec![1],
            IntegerWeight::new(1 * 41),
        )]);
        paths_ref.update(vec![FstPath::new(
            vec![2],
            vec![2],
            IntegerWeight::new(2 * 53),
        )]);
        paths_ref.update(vec![FstPath::new(
            vec![1, 4],
            vec![1, 4],
            IntegerWeight::new(4 * 185),
        )]);
        paths_ref.update(vec![FstPath::new(
            vec![2, 5],
            vec![2, 5],
            IntegerWeight::new(10 * 185),
        )]);
        paths_ref.update(vec![FstPath::new(
            vec![3],
            vec![3],
            IntegerWeight::new(3 * 185),
        )]);

        let paths: Counter<_> = fst.paths_iter().collect();

        assert_eq!(paths_ref, paths);
    }
}
