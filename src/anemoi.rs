use alloc::vec::Vec;
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use core::slice;
use ethabi::ParamType;
use uzkge::anemoi::{AnemoiJive, AnemoiJive254};

use crate::{utils, Error, Result};

pub const ANEMOI_EVAL: u64 = 100;

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_anemoi(
    data_ptr: *const u8,
    data_len: usize,
    ret_val: *mut u8,
) -> u8 {
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let ret = unsafe { slice::from_raw_parts_mut(ret_val, 32) };

    match eval_variable_length_hash(&data, ret) {
        Ok(()) => 0,
        Err(e) => e.code(),
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_anemoi_gas(data_ptr: *const u8, data_len: usize) -> u64 {
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

    data.len() as u64 / 32 * ANEMOI_EVAL
}

fn eval_variable_length_hash(data: &[u8], ret: &mut [u8]) -> Result<()> {
    let param = ParamType::FixedBytes(32);
    let uid_param = ParamType::Uint(64);

    let r = ethabi::decode(&[uid_param, param], data).map_err(|_| Error::Deserialize)?;

    let _h0 = utils::into_uint(r.get(0).cloned()).ok_or(Error::Deserialize)? as u64;
    let h1 = utils::into_bytes32(r.get(1).cloned()).ok_or(Error::Deserialize)?;

    let inputs: Vec<Fr> = h1
        .chunks(32)
        .map(|bytes| Fr::from_be_bytes_mod_order(bytes))
        .collect();
    let res = AnemoiJive254::eval_variable_length_hash(&inputs);

    ret.copy_from_slice(&res.into_bigint().to_bytes_be());

    Ok(())
}
