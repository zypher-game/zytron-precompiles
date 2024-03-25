use alloc::{boxed::Box, vec::Vec};
use ark_bn254::Fr;
use ark_ed_on_bn254::{EdwardsAffine, EdwardsProjective, Fq};
use ark_ff::PrimeField;
use core::slice;
use ethabi::ParamType;
use uzkge::gen_params::VerifierParams;
use zmatchmaking::build_cs::{verify_matchmaking, Proof};
use zshuffle::{
    build_cs::{verify_shuffle, ShuffleProof},
    MaskedCard,
};

use crate::{utils, Error, Result};

pub const PLONL_VERIFY_BASE: u64 = 100;

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_verify_matchmaking(data_ptr: *const u8, data_len: usize) -> u8 {
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

    match plonk_verify_matchmaking(&data) {
        Ok(()) => 0,
        Err(e) => e.code(),
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_verify_shuffle(data_ptr: *const u8, data_len: usize) -> u8 {
    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

    match plonk_verify_shuffle(&data) {
        Ok(()) => 0,
        Err(e) => e.code(),
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_plonk_verify_gas(_data_ptr: *const u8, _data_len: usize) -> u64 {
    //let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

    PLONL_VERIFY_BASE
}

fn plonk_verify_matchmaking(data: &[u8]) -> Result<()> {
    let r = ethabi::decode(
        &[
            ParamType::Bytes,
            ParamType::Array(Box::new(ParamType::Bytes)),
            ParamType::Array(Box::new(ParamType::Bytes)),
            ParamType::Bytes,
            ParamType::Bytes,
            ParamType::Bytes,
        ],
        data,
    )
    .map_err(|_| Error::Deserialize)?;
    let verifier_params: VerifierParams = utils::into_bytes(r.get(0).cloned())
        .ok_or(Error::Deserialize)
        .and_then(|v| bincode::deserialize(&v).map_err(|_e| Error::Deserialize))?;

    let inputs = utils::into_bytes_array(r.get(1).cloned())
        .map(|is| {
            is.iter()
                .map(|v| Fr::from_be_bytes_mod_order(v))
                .collect::<Vec<_>>()
        })
        .ok_or(Error::Deserialize)?;

    let outputs = utils::into_bytes_array(r.get(2).cloned())
        .map(|is| {
            is.iter()
                .map(|v| Fr::from_be_bytes_mod_order(v))
                .collect::<Vec<_>>()
        })
        .ok_or(Error::Deserialize)?;

    let commitment = utils::into_bytes(r.get(3).cloned())
        .map(|v| Fr::from_be_bytes_mod_order(&v))
        .ok_or(Error::Deserialize)?;

    let random_number = utils::into_bytes(r.get(4).cloned())
        .map(|v| Fr::from_be_bytes_mod_order(&v))
        .ok_or(Error::Deserialize)?;

    let proof: Proof = utils::into_bytes(r.get(3).cloned())
        .ok_or(Error::Deserialize)
        .and_then(|v| bincode::deserialize(&v).map_err(|_e| Error::Deserialize))?;

    verify_matchmaking(
        &verifier_params,
        &inputs,
        &outputs,
        &commitment,
        &random_number,
        &proof,
    )
    .map_err(|_e| Error::VerifyFail)
}

fn bytes_2_masked_card(cards: &[Vec<u8>]) -> Result<MaskedCard> {
    let e1: EdwardsProjective = {
        let x = Fq::from_be_bytes_mod_order(cards.first().ok_or(Error::Deserialize)?);
        let y = Fq::from_be_bytes_mod_order(cards.get(1).ok_or(Error::Deserialize)?);
        let affine = EdwardsAffine::new(x, y);
        affine.into()
    };

    let e2: EdwardsProjective = {
        let x = Fq::from_be_bytes_mod_order(cards.get(0).ok_or(Error::Deserialize)?);
        let y = Fq::from_be_bytes_mod_order(cards.get(1).ok_or(Error::Deserialize)?);
        let affine = EdwardsAffine::new(x, y);
        affine.into()
    };
    Ok(MaskedCard { e1, e2 })
}

fn plonk_verify_shuffle(data: &[u8]) -> Result<()> {
    let r = ethabi::decode(
        &[
            ParamType::Bytes,
            ParamType::Array(Box::new(ParamType::Array(Box::new(ParamType::Bytes)))),
            ParamType::Array(Box::new(ParamType::Array(Box::new(ParamType::Bytes)))),
            ParamType::Bytes,
        ],
        data,
    )
    .map_err(|_| Error::Deserialize)?;
    let verifier_params: VerifierParams = utils::into_bytes(r.get(0).cloned())
        .ok_or(Error::Deserialize)
        .and_then(|v| bincode::deserialize(&v).map_err(|_e| Error::Deserialize))?;
    let input_cards = {
        let cards = utils::into_bytes_array_array(r.get(1).cloned()).ok_or(Error::Deserialize)?;
        let mut ret = Vec::new();
        for card in cards {
            ret.push(bytes_2_masked_card(&card)?);
        }
        ret
    };

    let output_cards = {
        let cards = utils::into_bytes_array_array(r.get(2).cloned()).ok_or(Error::Deserialize)?;
        let mut ret = Vec::new();
        for card in cards {
            ret.push(bytes_2_masked_card(&card)?);
        }
        ret
    };

    let proof: ShuffleProof = utils::into_bytes(r.get(3).cloned())
        .ok_or(Error::Deserialize)
        .and_then(|v| bincode::deserialize(&v).map_err(|_e| Error::Deserialize))?;
    verify_shuffle(&verifier_params, &input_cards, &output_cards, &proof)
        .map_err(|_e| Error::VerifyFail)
}
