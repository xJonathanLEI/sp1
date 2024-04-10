use p3_baby_bear::BabyBear;
use p3_bn254_fr::Bn254Fr;
use p3_field::extension::BinomialExtensionField;
use sp1_recursion_core::stark::config::{InnerChallenge, InnerVal};

use crate::{asm::AsmConfig, prelude::Config};

pub type InnerConfig = AsmConfig<InnerVal, InnerChallenge>;

#[derive(Clone, Default, Debug)]
pub struct OuterConfig;

impl Config for OuterConfig {
    type N = Bn254Fr;
    type F = BabyBear;
    type EF = BinomialExtensionField<BabyBear, 4>;
}
