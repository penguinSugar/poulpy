use poulpy_hal::test_suite::serialization::test_reader_writer_interface;

use crate::layouts::{
    Base2K, Degree, Dnum, Dsize, GGLWE, GGLWECompressedOwned, GGLWEOwned, GGSW, GGSWCompressedOwned, GGSWOwned, GLWE,
    GLWEAutomorphismKey, GLWEAutomorphismKeyCompressedOwned, GLWEAutomorphismKeyOwned, GLWECompressedOwned, GLWEOwned,
    GLWESwitchingKey, GLWESwitchingKeyCompressedOwned, GLWESwitchingKeyOwned, GLWETensorKey, GLWETensorKeyCompressedOwned,
    GLWETensorKeyOwned, GLWEToLWEKey, GLWEToLWEKeyOwned, GLWEToLWESwitchingKeyCompressedOwned, LWE, LWECompressedOwned, LWEOwned,
    LWESwitchingKey, LWESwitchingKeyCompressedOwned, LWESwitchingKeyOwned, LWEToGLWEKey, LWEToGLWEKeyCompressedOwned,
    LWEToGLWEKeyOwned, Rank, TorusPrecision,
    compressed::{
        GGLWECompressed, GGSWCompressed, GLWEAutomorphismKeyCompressed, GLWECompressed, GLWESwitchingKeyCompressed,
        GLWETensorKeyCompressed, GLWEToLWESwitchingKeyCompressed, LWECompressed, LWESwitchingKeyCompressed,
        LWEToGLWEKeyCompressed,
    },
};

const N_GLWE: Degree = Degree(64);
const N_LWE: Degree = Degree(32);
const BASE2K: Base2K = Base2K(12);
const K: TorusPrecision = TorusPrecision(33);
const DNUM: Dnum = Dnum(3);
const RANK: Rank = Rank(2);
const DSIZE: Dsize = Dsize(1);

#[test]
fn glwe_serialization() {
    let original: GLWEOwned = GLWE::alloc(N_GLWE, BASE2K, K, RANK);
    poulpy_hal::test_suite::serialization::test_reader_writer_interface(original);
}

#[test]
fn glwe_compressed_serialization() {
    let original: GLWECompressedOwned = GLWECompressed::alloc(N_GLWE, BASE2K, K, RANK);
    test_reader_writer_interface(original);
}

#[test]
fn lwe_serialization() {
    let original: LWEOwned = LWE::alloc(N_LWE, BASE2K, K);
    test_reader_writer_interface(original);
}

#[test]
fn lwe_compressed_serialization() {
    let original: LWECompressedOwned = LWECompressed::alloc(BASE2K, K);
    test_reader_writer_interface(original);
}

#[test]
fn test_gglwe_serialization() {
    let original: GGLWEOwned = GGLWE::alloc(N_GLWE, BASE2K, K, RANK, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn test_gglwe_compressed_serialization() {
    let original: GGLWECompressedOwned = GGLWECompressed::alloc(N_GLWE, BASE2K, K, RANK, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn test_glwe_switching_key_serialization() {
    let original: GLWESwitchingKeyOwned = GLWESwitchingKey::alloc(N_GLWE, BASE2K, K, RANK, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn test_glwe_switching_key_compressed_serialization() {
    let original: GLWESwitchingKeyCompressedOwned = GLWESwitchingKeyCompressed::alloc(N_GLWE, BASE2K, K, RANK, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn test_automorphism_key_serialization() {
    let original: GLWEAutomorphismKeyOwned = GLWEAutomorphismKey::alloc(N_GLWE, BASE2K, K, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn test_automorphism_key_compressed_serialization() {
    let original: GLWEAutomorphismKeyCompressedOwned = GLWEAutomorphismKeyCompressed::alloc(N_GLWE, BASE2K, K, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn test_tensor_key_serialization() {
    let original: GLWETensorKeyOwned = GLWETensorKey::alloc(N_GLWE, BASE2K, K, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn test_tensor_key_compressed_serialization() {
    let original: GLWETensorKeyCompressedOwned = GLWETensorKeyCompressed::alloc(N_GLWE, BASE2K, K, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn glwe_to_lwe_key_serialization() {
    let original: GLWEToLWEKeyOwned = GLWEToLWEKey::alloc(N_GLWE, BASE2K, K, RANK, DNUM);
    test_reader_writer_interface(original);
}

#[test]
fn glwe_to_lwe_key_compressed_serialization() {
    let original: GLWEToLWESwitchingKeyCompressedOwned = GLWEToLWESwitchingKeyCompressed::alloc(N_GLWE, BASE2K, K, RANK, DNUM);
    test_reader_writer_interface(original);
}

#[test]
fn lwe_to_glwe_key_serialization() {
    let original: LWEToGLWEKeyOwned = LWEToGLWEKey::alloc(N_GLWE, BASE2K, K, RANK, DNUM);
    test_reader_writer_interface(original);
}

#[test]
fn lwe_to_glwe_key_compressed_serialization() {
    let original: LWEToGLWEKeyCompressedOwned = LWEToGLWEKeyCompressed::alloc(N_GLWE, BASE2K, K, RANK, DNUM);
    test_reader_writer_interface(original);
}

#[test]
fn lwe_switching_key_serialization() {
    let original: LWESwitchingKeyOwned = LWESwitchingKey::alloc(N_GLWE, BASE2K, K, DNUM);
    test_reader_writer_interface(original);
}

#[test]
fn lwe_switching_key_compressed_serialization() {
    let original: LWESwitchingKeyCompressedOwned = LWESwitchingKeyCompressed::alloc(N_GLWE, BASE2K, K, DNUM);
    test_reader_writer_interface(original);
}

#[test]
fn ggsw_serialization() {
    let original: GGSWOwned = GGSW::alloc(N_GLWE, BASE2K, K, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}

#[test]
fn ggsw_compressed_serialization() {
    let original: GGSWCompressedOwned = GGSWCompressed::alloc(N_GLWE, BASE2K, K, RANK, DNUM, DSIZE);
    test_reader_writer_interface(original);
}
