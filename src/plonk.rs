use alloc::vec::Vec;
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use core::slice;
use ethabi::ParamType;
use uzkge::plonk::verifier;

use crate::{utils, Error, Result};

pub const PLONL_VERIFY_BASE: u64 = 100;

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_plonk_verify(
    data_ptr: *const u8,
    data_len: usize,
    ret_val: *mut u8,
) -> u8 {
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let ret = unsafe { slice::from_raw_parts_mut(ret_val, 1) };

    match plonk_verify(&data, ret) {
        Ok(()) => 0,
        Err(e) => e.code(),
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_plonk_verify_gas(data_ptr: *const u8, data_len: usize) -> u64 {
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

    PLONL_VERIFY_BASE
}

fn plonk_verify(data: &[u8], ret: &mut [u8]) -> Result<()> {
    todo!()

    Ok(())
}
