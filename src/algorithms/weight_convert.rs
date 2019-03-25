use failure::Fallible;

use crate::algorithms::{FinalArc, MapFinalAction};
use crate::fst_traits::{ExpandedFst, MutableFst};
use crate::semirings::Semiring;
use crate::{Arc, EPS_LABEL};

pub trait WeightConverter<SI: Semiring, SO: Semiring> {
    fn arc_map(&mut self, arc: &Arc<SI>) -> Fallible<Arc<SO>>;
    fn final_arc_map(&mut self, final_arc: &FinalArc<SI>) -> Fallible<FinalArc<SO>>;
    fn final_action(&self) -> MapFinalAction;
}

pub fn weight_convert<F1, F2, M>(fst_in: &F1, mapper: &mut M) -> Fallible<F2>
where
    F1: ExpandedFst,
    F2: MutableFst,
    M: WeightConverter<F1::W, F2::W>,
{
    let mut fst_out = F2::new();
    let final_action = mapper.final_action();

    // Empty FST.
    if fst_in.start().is_none() {
        return Ok(fst_out);
    }

    // Reserve enough space for all the states to avoid re-allocations.
    let mut num_states_needed = fst_in.num_states();
    if !(final_action == MapFinalAction::MapNoSuperfinal) {
        num_states_needed += 1;
    }
    fst_out.reserve_states(num_states_needed);

    // Add all the states from the input fst to the output fst.
    for _ in fst_in.states_iter() {
        fst_out.add_state();
    }

    // Set superfinal states as final.
    let mut superfinal = None;
    if final_action == MapFinalAction::MapRequireSuperfinal {
        superfinal = Some(fst_out.add_state());
        fst_out.set_final(superfinal.unwrap(), F2::W::one())?;
    }

    if let Some(start_state) = fst_in.start() {
        fst_out.set_start(start_state)?;
    }

    let states: Vec<_> = fst_in.states_iter().collect();
    for state in states {
        fst_out.reserve_arcs(state, fst_in.num_arcs(state)?)?;
        for arc in fst_in.arcs_iter(state)? {
            fst_out.add_arc(state, mapper.arc_map(arc)?)?;
        }
        if let Some(w) = fst_in.final_weight(state) {
            let final_arc = FinalArc {
                ilabel: EPS_LABEL,
                olabel: EPS_LABEL,
                weight: w.clone(),
            };
            let mapped_final_arc = mapper.final_arc_map(&final_arc)?;
            match final_action {
                MapFinalAction::MapNoSuperfinal => {
                    if final_arc.ilabel != EPS_LABEL || final_arc.olabel != EPS_LABEL {
                        bail!("ArcMap: Non-zero arc labels for superfinal arc")
                    }

                    fst_out.set_final(state, mapped_final_arc.weight).unwrap();
                }
                MapFinalAction::MapAllowSuperfinal => {
                    if final_arc.ilabel != EPS_LABEL || final_arc.olabel != EPS_LABEL {
                        if superfinal.is_none() {
                            let superfinal_id = fst_out.add_state();
                            superfinal = Some(superfinal_id);
                            fst_out.set_final(superfinal_id, F2::W::one()).unwrap();
                        }

                        fst_out
                            .add_arc(
                                state,
                                Arc::new(
                                    mapped_final_arc.ilabel,
                                    mapped_final_arc.olabel,
                                    mapped_final_arc.weight,
                                    superfinal.unwrap(),
                                ),
                            )
                            .unwrap();

                        fst_out.delete_final_weight(state).unwrap();
                    } else {
                        fst_out.set_final(state, mapped_final_arc.weight).unwrap();
                    }
                }
                MapFinalAction::MapRequireSuperfinal => {
                    if final_arc.ilabel != EPS_LABEL
                        || final_arc.olabel != EPS_LABEL
                        || !final_arc.weight.is_zero()
                    {
                        fst_out
                            .add_arc(
                                state,
                                Arc::new(
                                    mapped_final_arc.ilabel,
                                    mapped_final_arc.olabel,
                                    mapped_final_arc.weight,
                                    superfinal.unwrap(),
                                ),
                            )
                            .unwrap();
                    }
                    fst_out.delete_final_weight(state).unwrap();
                }
            }
        }
    }

    Ok(fst_out)
}
