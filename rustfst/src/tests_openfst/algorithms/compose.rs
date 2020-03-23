use failure::Fallible;
use serde_derive::{Deserialize, Serialize};

use crate::algorithms::compose_filters::{
    AltSequenceComposeFilter, MatchComposeFilter, NoMatchComposeFilter, NullComposeFilter,
    SequenceComposeFilter, TrivialComposeFilter,
};
use crate::algorithms::matchers::SortedMatcher;
use crate::algorithms::{
    compose, compose_with_config, ComposeConfig, ComposeFilterEnum, ComposeFst,
};
use crate::fst_impls::VectorFst;
use crate::fst_traits::SerializableFst;
use crate::semirings::{SerializableSemiring, WeaklyDivisibleSemiring, WeightQuantize};
use crate::tests_openfst::algorithms::dynamic_fst::compare_fst_static_dynamic;
use crate::tests_openfst::FstTestData;

#[derive(Serialize, Deserialize, Debug)]
pub struct ComposeOperationResult {
    fst_2: String,
    result_static: String,
    result_dynamic: String,
    connect: bool,
    filter_name: String,
}

pub struct ComposeTestData<F>
where
    F: SerializableFst,
    F::W: SerializableSemiring,
{
    pub fst_2: F,
    pub result_static: F,
    pub result_dynamic: F,
    pub connect: bool,
    pub filter: ComposeFilterEnum,
}

impl ComposeOperationResult {
    pub fn parse<F>(&self) -> ComposeTestData<F>
    where
        F: SerializableFst,
        F::W: SerializableSemiring,
    {
        ComposeTestData {
            fst_2: F::from_text_string(self.fst_2.as_str()).unwrap(),
            result_static: F::from_text_string(self.result_static.as_str()).unwrap(),
            result_dynamic: F::from_text_string(self.result_dynamic.as_str()).unwrap(),
            connect: self.connect,
            filter: match self.filter_name.as_str() {
                "auto" => ComposeFilterEnum::AutoFilter,
                "null" => ComposeFilterEnum::NullFilter,
                "trivial" => ComposeFilterEnum::TrivialFilter,
                "sequence" => ComposeFilterEnum::SequenceFilter,
                "alt_sequence" => ComposeFilterEnum::SequenceFilter,
                "match" => ComposeFilterEnum::MatchFilter,
                "no_match" => ComposeFilterEnum::NoMatchFilter,
                _ => panic!("Not supported : {}", &self.filter_name),
            },
        }
    }
}

pub fn test_compose<W>(test_data: &FstTestData<VectorFst<W>>) -> Fallible<()>
where
    W: SerializableSemiring + WeightQuantize + WeaklyDivisibleSemiring + 'static,
    W::ReverseWeight: 'static,
{
    for compose_test_data in &test_data.compose {
        let mut config = ComposeConfig::default();
        config.connect = compose_test_data.connect;
        config.compose_filter = compose_test_data.filter;
        std::dbg!(&config);
        let fst_res_static: VectorFst<_> =
            compose_with_config(&test_data.raw, &compose_test_data.fst_2, config)?;

        assert_eq!(
            compose_test_data.result_static,
            fst_res_static,
            "{}",
            error_message_fst!(
                compose_test_data.result_static,
                fst_res_static,
                format!(
                    "Compose failed : connect = {} filter_name = {:?}",
                    compose_test_data.connect, compose_test_data.filter
                )
            )
        );
    }
    Ok(())
}

pub fn test_compose_dynamic<W>(test_data: FstTestData<VectorFst<W>>) -> Fallible<()>
where
    W: SerializableSemiring + WeightQuantize + WeaklyDivisibleSemiring + 'static,
    W::ReverseWeight: 'static,
{
    // TODO: Understand lifetime issue here
    for compose_test_data in &test_data.compose {
        let fst1 = test_data.raw.clone();
        let fst2 = compose_test_data.fst_2.clone();

        match compose_test_data.filter {
            ComposeFilterEnum::AutoFilter => {
                let dyn_fst = ComposeFst::new_auto(&fst1, &fst2)?;

                // compare_fst_static_dynamic(&compose_test_data.result_dynamic.clone(), &dyn_fst)?;
            }
            ComposeFilterEnum::NullFilter => {
                let dyn_fst =
                    ComposeFst::<_, NullComposeFilter<SortedMatcher<_>, SortedMatcher<_>>>::new(
                        &fst1, &fst2,
                    )?;

                // compare_fst_static_dynamic(&compose_test_data.result_dynamic, &dyn_fst)?;
            }
            ComposeFilterEnum::SequenceFilter => {
                let dyn_fst = ComposeFst::<
                    _,
                    SequenceComposeFilter<_, SortedMatcher<_>, SortedMatcher<_>>,
                >::new(&fst1, &fst2)?;
                // compare_fst_static_dynamic(&compose_test_data.result_dynamic, &dyn_fst)?;
            }
            ComposeFilterEnum::AltSequenceFilter => {
                let dyn_fst = ComposeFst::<
                    _,
                    AltSequenceComposeFilter<_, SortedMatcher<_>, SortedMatcher<_>>,
                >::new(&fst1, &fst2)?;
                // compare_fst_static_dynamic(&compose_test_data.result_dynamic, &dyn_fst)?;
            }
            ComposeFilterEnum::MatchFilter => {
                let dyn_fst = ComposeFst::<
                    _,
                    MatchComposeFilter<_, _, SortedMatcher<_>, SortedMatcher<_>>,
                >::new(&fst1, &fst2)?;
                // compare_fst_static_dynamic(&compose_test_data.result_dynamic, &dyn_fst)?;
            }
            ComposeFilterEnum::NoMatchFilter => {
                let dyn_fst = ComposeFst::<
                    _,
                    NoMatchComposeFilter<SortedMatcher<_>, SortedMatcher<_>>,
                >::new(&fst1, &fst2)?;
                // compare_fst_static_dynamic(&compose_test_data.result_dynamic, &dyn_fst)?;
            }
            ComposeFilterEnum::TrivialFilter => {
                let dyn_fst = ComposeFst::<
                    _,
                    TrivialComposeFilter<SortedMatcher<_>, SortedMatcher<_>>,
                >::new(&fst1, &fst2)?;
                // compare_fst_static_dynamic(&compose_test_data.result_dynamic, &dyn_fst)?;
            }
        };
    }
    Ok(())
}