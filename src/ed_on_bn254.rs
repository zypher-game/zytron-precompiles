use ark_ec::{AffineRepr, CurveGroup};
use ark_ed_on_bn254::{EdwardsAffine, Fq, Fr};
use ark_ff::{BigInteger, PrimeField};
use core::slice;
use ethabi::ParamType;

use crate::{utils, Error, Result};

pub const POINY_ADD_GAS: u64 = 100;
pub const SCALAR_MUL_GAS: u64 = 200;

// support
// 1. point add
// 2. scalar mul

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_ed_on_bn254_point_add(
    data_ptr: *const u8,
    data_len: usize,
    ret_val: *mut u8,
) -> u8 {
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let ret = unsafe { slice::from_raw_parts_mut(ret_val, 64) };

    match point_add(&data, ret) {
        Ok(()) => 0,
        Err(e) => e.code(),
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_ed_on_bn254_point_add_gas(_: *const u8, _: usize) -> u64 {
    POINY_ADD_GAS
}

fn point_add(data: &[u8], ret: &mut [u8]) -> Result<()> {
    let n = ParamType::Uint(256);
    let r = ethabi::decode(&[n.clone(), n.clone(), n.clone(), n], data)
        .map_err(|_| Error::Deserialize)?;
    let h1 = utils::into_uint256(r.get(0).cloned()).ok_or(Error::Deserialize)?;
    let h2 = utils::into_uint256(r.get(1).cloned()).ok_or(Error::Deserialize)?;
    let h3 = utils::into_uint256(r.get(2).cloned()).ok_or(Error::Deserialize)?;
    let h4 = utils::into_uint256(r.get(3).cloned()).ok_or(Error::Deserialize)?;
    let mut tmp_bytes = [0u8; 32];
    h1.to_big_endian(&mut tmp_bytes);
    let x_1 = Fq::from_be_bytes_mod_order(&tmp_bytes);
    h2.to_big_endian(&mut tmp_bytes);
    let y_1 = Fq::from_be_bytes_mod_order(&tmp_bytes);
    h3.to_big_endian(&mut tmp_bytes);
    let x_2 = Fq::from_be_bytes_mod_order(&tmp_bytes);
    h4.to_big_endian(&mut tmp_bytes);
    let y_2 = Fq::from_be_bytes_mod_order(&tmp_bytes);

    let p1 = EdwardsAffine::new(x_1, y_1);
    let p2 = EdwardsAffine::new(x_2, y_2);
    let p3 = p1 + p2;

    let (r_x, r_y) = p3.into_affine().xy().unwrap();
    ret[0..32].copy_from_slice(&r_x.into_bigint().to_bytes_be());
    ret[32..64].copy_from_slice(&r_y.into_bigint().to_bytes_be());

    Ok(())
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_ed_on_bn254_scalar_mul(
    data_ptr: *const u8,
    data_len: usize,
    ret_val: *mut u8,
) -> u8 {
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let ret = unsafe { slice::from_raw_parts_mut(ret_val, 64) };

    match scalar_mul(&data, ret) {
        Ok(()) => 0,
        Err(e) => e.code(),
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_ed_on_bn254_scalar_mul_gas(_: *const u8, _: usize) -> u64 {
    SCALAR_MUL_GAS
}

fn scalar_mul(data: &[u8], ret: &mut [u8]) -> Result<()> {
    let n = ParamType::Uint(256);
    let r = ethabi::decode(&[n.clone(), n.clone(), n], data).map_err(|_| Error::Deserialize)?;
    let h1 = utils::into_uint256(r.get(0).cloned()).ok_or(Error::Deserialize)?;
    let h2 = utils::into_uint256(r.get(1).cloned()).ok_or(Error::Deserialize)?;
    let h3 = utils::into_uint256(r.get(2).cloned()).ok_or(Error::Deserialize)?;
    let mut tmp_bytes = [0u8; 32];
    h1.to_big_endian(&mut tmp_bytes);
    let s = Fr::from_be_bytes_mod_order(&tmp_bytes);
    h2.to_big_endian(&mut tmp_bytes);
    let x = Fq::from_be_bytes_mod_order(&tmp_bytes);
    h3.to_big_endian(&mut tmp_bytes);
    let y = Fq::from_be_bytes_mod_order(&tmp_bytes);

    let p = EdwardsAffine::new(x, y);
    let p2 = p * s;

    let (r_x, r_y) = p2.into_affine().xy().unwrap();
    ret[0..32].copy_from_slice(&r_x.into_bigint().to_bytes_be());
    ret[32..64].copy_from_slice(&r_y.into_bigint().to_bytes_be());

    Ok(())
}
