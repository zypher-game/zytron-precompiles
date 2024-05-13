use alloc::boxed::Box;
use alloc::vec::Vec;
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use core::slice;
use ethabi::ParamType;
use uzkge::anemoi::{AnemoiJive, AnemoiJive254};

use crate::{Error, Result};

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
    let rs = ethabi::decode(
        &[ParamType::Array(Box::new(ParamType::FixedBytes(32)))],
        data,
    )
    .map_err(|_| Error::Deserialize)?;
    let hs = rs
        .get(0)
        .and_then(|v| v.clone().into_array())
        .ok_or(Error::Deserialize)?;

    let mut inputs: Vec<Fr> = Vec::new();
    for r in hs {
        let h = r.into_fixed_bytes().ok_or(Error::Deserialize)?;
        inputs.push(Fr::from_be_bytes_mod_order(&h));
    }

    let res = AnemoiJive254::eval_variable_length_hash(&inputs);

    ret.copy_from_slice(&res.into_bigint().to_bytes_be());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{vec, UniformRand};
    use ark_std::rand::SeedableRng;
    use ethabi::Token;
    use rand_chacha::ChaChaRng;

    #[test]
    fn anemoi_works() {
        let mut prng = ChaChaRng::from_seed([0u8; 32]);
        let f1 = Fr::rand(&mut prng);
        let f2 = Fr::rand(&mut prng);
        let f3 = Fr::rand(&mut prng);

        // add with rust
        let res1 = AnemoiJive254::eval_variable_length_hash(&[f1, f2, f3]);
        let r1 = res1.into_bigint().to_bytes_be();

        // test from precompile serialize
        let h1 = f1.into_bigint().to_bytes_be();
        let h2 = f2.into_bigint().to_bytes_be();
        let h3 = f3.into_bigint().to_bytes_be();

        let data = ethabi::encode(&[Token::Array(vec![
            Token::FixedBytes(h1),
            Token::FixedBytes(h2),
            Token::FixedBytes(h3),
        ])]);
        let mut ret = vec![0u8; 32];

        eval_variable_length_hash(&data, &mut ret).unwrap();

        let res2 = ethabi::decode(&[ParamType::FixedBytes(32)], &ret).unwrap();
        let r2 = res2[0].clone().into_fixed_bytes().unwrap();

        assert_eq!(r1, r2);
    }
}
