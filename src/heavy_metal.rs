use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeavyMetal {
    pub name: String,
    pub index: usize,
    pub code: HeavyEnum,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HeavyEnum {
    Cd,
    Hg,
    As,
    Pb,
    Cr,
    Cu,
    Ni,
    Zn,
}

impl Display for HeavyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HeavyEnum::Cd => "镉",
            HeavyEnum::Hg => "汞",
            HeavyEnum::As => "砷",
            HeavyEnum::Pb => "铅",
            HeavyEnum::Cr => "铬",
            HeavyEnum::Cu => "铜",
            HeavyEnum::Ni => "镍",
            HeavyEnum::Zn => "锌",
        };
        write!(f, "{}", s)
    }
}
