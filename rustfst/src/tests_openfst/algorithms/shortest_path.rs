use std::fmt::Display;
use std::marker::PhantomData;

use anyhow::{format_err, Result};
use serde::{Deserialize, Serialize};

use crate::algorithms::{shortest_path_with_config, ShortestPathConfig};
use crate::fst_path::check_path_in_fst;
use crate::fst_traits::{MutableFst, SerializableFst};
use crate::semirings::WeaklyDivisibleSemiring;
use crate::semirings::WeightQuantize;
use crate::semirings::{Semiring, SerializableSemiring};
use crate::tests_openfst::utils::test_correctness_properties;
use crate::tests_openfst::FstTestData;
use crate::FstPath;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShorestPathOperationResult {
    unique: bool,
    nshortest: usize,
    result_path: String,
}

pub struct ShortestPathTestData<W, F>
where
    F: SerializableFst<W>,
    W: SerializableSemiring,
{
    unique: bool,
    nshortest: usize,
    result: Result<F>,
    w: PhantomData<W>,
}

impl ShorestPathOperationResult {
    pub fn parse<W, F, P>(&self, dir_path: P) -> ShortestPathTestData<W, F>
    where
        F: SerializableFst<W>,
        W: SerializableSemiring,
        P: AsRef<Path>,
    {
        ShortestPathTestData {
            unique: self.unique,
            nshortest: self.nshortest,
            result: match self.result_path.as_str() {
                "error" => Err(format_err!("lol")),
                _ => F::read(dir_path.as_ref().join(&self.result_path)),
            },
            w: PhantomData,
        }
    }
}

pub fn test_shortest_path<W, F>(test_data: &FstTestData<W, F>) -> Result<()>
where
    F: SerializableFst<W> + MutableFst<W> + Display,
    W: SerializableSemiring + WeaklyDivisibleSemiring + WeightQuantize,
    <W as Semiring>::ReverseWeight: WeaklyDivisibleSemiring + WeightQuantize,
    W: Into<<W as Semiring>::ReverseWeight> + From<<W as Semiring>::ReverseWeight>,
{
    for data in &test_data.shortest_path {
        let config = ShortestPathConfig::default()
            .with_nshortest(data.nshortest)
            .with_unique(data.unique);
        let fst_res: Result<F> = shortest_path_with_config(&test_data.raw, config);
        match (&data.result, &fst_res) {
            (Ok(fst_expected), Ok(ref fst_shortest)) => {
                // Comparing directly the fsts doesn't work because there is undefined behaviour
                // when multiple paths have the same weights.
                // Instead, what we do here is to generate the paths in the output fst of
                // the shortest_path operation and then check :
                // 1) Same number of paths
                // 2) Paths at the same position have the same weight
                // 3) Paths generated by rustfst does exist and is final in the input fst

                let fst_paths_expected : Vec<FstPath<W>> = fst_expected.paths_iter().collect();
                let fst_paths_computed : Vec<FstPath<W>> = fst_shortest.paths_iter().collect();

                assert_eq!(fst_paths_expected.len(), fst_paths_computed.len());

                for (path_expected, path_computed) in fst_paths_expected.iter().zip(fst_paths_computed.iter()) {
                    assert_eq!(path_expected.weight, path_computed.weight);
                }

                let msg =                         format!(
                    "ShortestPath fail for nshortest = {:?} and unique = {:?}",
                    data.nshortest, data.unique
                );
                for path_computed in fst_paths_computed.iter() {
                    assert!(check_path_in_fst(&test_data.raw, path_computed), "{} -> Missing path {:?} in Fst : \n{}", &msg, &path_computed, &test_data.raw);
                }

                test_correctness_properties(fst_expected, fst_shortest, msg)
            }
            (Ok(_fst_expected), Err(e)) => panic!(
                "ShortestPath fail for nshortest = {:?} and unique = {:?}. Got Err. Expected Ok \n{:?}",
                data.nshortest, data.unique, e
            ),
            (Err(_), Ok(_fst_shortest)) => panic!(
                "ShortestPath fail for nshortest = {:?} and unique = {:?}. Got Ok. Expected Err \n{}",
                data.nshortest, data.unique, _fst_shortest
            ),
            (Err(_), Err(_)) => {
                // Ok
            }
        };
    }

    Ok(())
}
