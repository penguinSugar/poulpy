use std::collections::HashMap;

use poulpy_core::layouts::{GLWEAutomorphismKeyCompressedOwned, GLWETensorKeyCompressedOwned};
use poulpy_hal::layouts::Data;

use crate::bin_fhe::blind_rotation::{BlindRotationAlgo, BlindRotationKeyCompressed};

#[allow(dead_code)]
pub struct CircuitBootstrappingKey<D: Data, BRA: BlindRotationAlgo> {
    pub(crate) brk: BlindRotationKeyCompressed<D, BRA>,
    pub(crate) tsk: GLWETensorKeyCompressedOwned,
    pub(crate) atk: HashMap<i64, GLWEAutomorphismKeyCompressedOwned>,
}
