use std::mem::swap;

use anyhow::{ensure, Result};

use crate::fst_properties::FstProperties;
use crate::fst_traits::MutableFst;
use crate::semirings::Semiring;
use crate::{StateId, Trs};

/// Sorts the input states of an FST. order[i] gives the the state ID after
/// sorting that corresponds to the state ID i before sorting; it must
/// therefore be a permutation of the input FST's states ID sequence.
pub fn state_sort<W, F>(fst: &mut F, order: &[StateId]) -> Result<()>
where
    W: Semiring,
    F: MutableFst<W>,
{
    ensure!(
        order.len() == fst.num_states(),
        "StateSort : Bad order vector size : {}. Expected {}",
        order.len(),
        fst.num_states()
    );
    if fst.start().is_none() {
        return Ok(());
    }
    // TODO: Use properties with mask once available
    let props = fst.properties_revamp() & FstProperties::statesort_properties();

    let start_state = fst.start().unwrap();

    let mut done = vec![false; order.len()];

    if cfg!(debug_assertions) {
        assert!(start_state < order.len());
        assert!(order[start_state] < fst.num_states());
    }

    fst.set_start(order[start_state])?;

    for mut s1 in 0..fst.num_states() {
        if done[s1] {
            continue;
        }
        let mut final1 = unsafe { fst.final_weight_unchecked(s1) };
        let mut final2 = None;
        let mut trsa: Vec<_> = fst.get_trs(s1)?.trs().to_vec();
        let mut trsb = vec![];
        while !done[s1] {
            let s2 = order[s1];
            if !done[s2] {
                final2 = unsafe { fst.final_weight_unchecked(s2) };
                trsb = fst.get_trs(s2)?.trs().to_vec();
            }
            match final1 {
                None => fst.delete_final_weight(s2)?,
                Some(v) => fst.set_final(s2, v)?,
            };
            fst.delete_trs(s2)?;
            for tr in trsa.iter() {
                let mut tr = tr.clone();
                tr.nextstate = order[tr.nextstate];
                fst.add_tr(s2, tr)?;
            }
            done[s1] = true;

            // next
            swap(&mut trsa, &mut trsb);
            final1 = final2.clone();
            s1 = s2;
        }
    }

    fst.set_properties_with_mask(props, FstProperties::all_properties());

    Ok(())
}
