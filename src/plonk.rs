use alloc::{boxed::Box, vec::Vec};
use ark_bn254::Fr;
use ark_ed_on_bn254::{EdwardsAffine, EdwardsProjective, Fq};
use ark_ff::PrimeField;
use ark_std::panic;
use core::slice;
use ethabi::ParamType;
use uzkge::gen_params::VerifierParams;
use zmatchmaking::build_cs::{verify_matchmaking, Proof};
use zshuffle::{
    build_cs::{verify_shuffle, ShuffleProof, TurboCS},
    MaskedCard,
};

use crate::{utils, Error, Result};

pub const PLONL_VERIFY_BASE: u64 = 100;

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_verify_matchmaking(data_ptr: *const u8, data_len: usize) -> u8 {
    let result = panic::catch_unwind(|| {
        let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

        match plonk_verify_matchmaking(data) {
            Ok(()) => 0,
            Err(e) => e.code(),
        }
    });
    if let Ok(code) = result {
        code
    } else {
        Error::Unknown.code()
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn __precompile_verify_shuffle(data_ptr: *const u8, data_len: usize) -> u8 {
    let result = panic::catch_unwind(|| {
        let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

        match plonk_verify_shuffle(data) {
            Ok(()) => 0,
            Err(e) => e.code(),
        }
    });
    if let Ok(code) = result {
        code
    } else {
        Error::Unknown.code()
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

    let verifier_params: VerifierParams = utils::into_bytes(r.first().cloned())
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

    let proof: Proof = utils::into_bytes(r.get(5).cloned())
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
        let x = Fq::from_be_bytes_mod_order(cards.get(2).ok_or(Error::Deserialize)?);
        let y = Fq::from_be_bytes_mod_order(cards.get(3).ok_or(Error::Deserialize)?);
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

    let verifier_params: VerifierParams = utils::into_bytes(r.first().cloned())
        .ok_or(Error::Deserialize)
        .and_then(|v| bincode::deserialize(&v).map_err(|_e| Error::Deserialize))?;

    let input_cards = {
        let cards = utils::into_bytes_2d_array(r.get(1).cloned()).ok_or(Error::Deserialize)?;
        let mut ret = Vec::new();
        for card in cards {
            ret.push(bytes_2_masked_card(&card)?);
        }
        ret
    };

    let output_cards = {
        let cards = utils::into_bytes_2d_array(r.get(2).cloned()).ok_or(Error::Deserialize)?;
        let mut ret = Vec::new();
        for card in cards {
            ret.push(bytes_2_masked_card(&card)?);
        }
        ret
    };

    let proof: ShuffleProof = utils::into_bytes(r.get(3).cloned())
        .ok_or(Error::Deserialize)
        .and_then(|v| {
            ShuffleProof::from_bytes_be::<TurboCS>(&v).map_err(|_e| Error::Deserialize)
        })?;

    verify_shuffle(&verifier_params, &input_cards, &output_cards, &proof)
        .map_err(|_e| Error::VerifyFail)
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use ark_bn254::Fr;
    use ark_ec::{AffineRepr, CurveGroup};
    use ark_ff::{BigInteger, One, PrimeField, UniformRand};
    use ark_std::{
        collections::HashMap,
        rand::{CryptoRng, RngCore, SeedableRng},
    };
    use ethabi::Token;
    use rand_chacha::ChaChaRng;
    use uzkge::anemoi::{AnemoiJive, AnemoiJive254};
    use zmatchmaking::{
        build_cs::{prove_matchmaking, N},
        gen_params::{gen_prover_params, get_verifier_params},
    };
    use zshuffle::{
        build_cs::prove_shuffle,
        gen_params::{
            gen_shuffle_prover_params, get_shuffle_verifier_params,
            refresh_prover_params_public_key,
        },
        keygen::{aggregate_keys, Keypair},
        mask::{mask, verify_mask},
        Card,
    };

    use super::{plonk_verify_matchmaking, plonk_verify_shuffle};

    #[test]
    fn test_plonk_verify_matchmaking() {
        let mut rng = ChaChaRng::from_entropy();

        let inputs = (1..=N)
            .into_iter()
            .map(|i| Fr::from(i as u64))
            .collect::<Vec<_>>();

        let committed_seed = Fr::rand(&mut rng);

        let committment = AnemoiJive254::eval_variable_length_hash(&[committed_seed]);

        let random_number = Fr::rand(&mut rng);

        let (proof, outputs) = prove_matchmaking(
            &mut rng,
            &inputs,
            &committed_seed,
            &random_number,
            &gen_prover_params().unwrap(),
        )
        .unwrap();

        let verifier_params = bincode::serialize(&get_verifier_params().unwrap()).unwrap();

        let inputs = inputs
            .iter()
            .map(|v| Token::Bytes(v.into_bigint().to_bytes_be()))
            .collect();

        let outputs = outputs
            .iter()
            .map(|v| Token::Bytes(v.into_bigint().to_bytes_be()))
            .collect();

        let committment = committment.into_bigint().to_bytes_be();

        let random_number = random_number.into_bigint().to_bytes_be();

        let proof = bincode::serialize(&proof).unwrap();

        let data = ethabi::encode(&[
            Token::Bytes(verifier_params),
            Token::Array(inputs),
            Token::Array(outputs),
            Token::Bytes(committment),
            Token::Bytes(random_number),
            Token::Bytes(proof),
        ]);

        plonk_verify_matchmaking(&data).unwrap()
    }

    #[derive(PartialEq, PartialOrd, Clone, Copy, Eq)]
    pub enum Value {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl Value {
        const VALUES: [Self; 13] = [
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Jack,
            Self::Queen,
            Self::King,
            Self::Ace,
        ];
    }

    pub const N_CARDS: usize = 52;

    #[derive(PartialEq, Clone, Copy, Eq)]
    pub enum Suite {
        Club,
        Diamond,
        Heart,
        Spade,
    }

    impl Suite {
        const SUITES: [Self; 4] = [Self::Club, Self::Diamond, Self::Heart, Self::Spade];
    }

    #[derive(PartialEq, Clone, Eq, Copy)]
    pub struct ClassicPlayingCard {
        value: Value,
        suite: Suite,
    }

    impl ClassicPlayingCard {
        pub fn new(value: Value, suite: Suite) -> Self {
            Self { value, suite }
        }
    }

    fn encode_cards<R: CryptoRng + RngCore>(rng: &mut R) -> HashMap<Card, ClassicPlayingCard> {
        let num_of_cards = Value::VALUES.len() * Suite::SUITES.len();
        let mut map: HashMap<Card, ClassicPlayingCard> = HashMap::new();
        let plaintexts = (0..num_of_cards)
            .map(|_| Card::rand(rng))
            .collect::<Vec<_>>();

        let mut i = 0;
        for value in Value::VALUES.iter().copied() {
            for suite in Suite::SUITES.iter().copied() {
                let current_card = ClassicPlayingCard::new(value, suite);
                map.insert(plaintexts[i], current_card);
                i += 1;
            }
        }

        map
    }

    pub fn point_to_uncompress<F: PrimeField, G: CurveGroup<BaseField = F>>(
        point: &G,
    ) -> (Vec<u8>, Vec<u8>) {
        let affine = G::Affine::from(*point);
        let (x, y) = affine.xy().unwrap();
        (x.into_bigint().to_bytes_be(), y.into_bigint().to_bytes_be())
    }

    #[test]
    fn test_plonk_verify_shuffle() {
        let mut rng = ChaChaRng::from_seed([0u8; 32]);

        let card_mapping = encode_cards(&mut rng);

        let alice = Keypair::generate(&mut rng);

        let keys = [alice.public].to_vec();

        // Each player should run this computation. Alternatively, it can be ran by a smart contract
        let joint_pk = aggregate_keys(&keys).unwrap();

        // Each player should run this computation and verify that all players agree on the initial deck
        let mut deck = Vec::new();
        for card in card_mapping.keys() {
            let (masked_card, masked_proof) =
                mask(&mut rng, &joint_pk, card, &ark_ed_on_bn254::Fr::one()).unwrap();
            verify_mask(&joint_pk, card, &masked_card, &masked_proof).unwrap();

            deck.push(masked_card)
        }

        let mut prover_params = gen_shuffle_prover_params(N_CARDS).unwrap();

        refresh_prover_params_public_key(&mut prover_params, &joint_pk).unwrap();

        let mut verifier_params = get_shuffle_verifier_params(N_CARDS).unwrap();
        verifier_params.verifier_params = prover_params.prover_params.verifier_params.clone();

        // Alice, start shuffling.
        let (proof, alice_shuffle_deck) =
            prove_shuffle(&mut rng, &joint_pk, &deck, &prover_params).unwrap();

        let proof = proof.to_bytes_be();

        let verifier_params = bincode::serialize(&verifier_params).unwrap();
        let deck = {
            let mut ret = Vec::new();
            for it in deck.iter() {
                let mut tmp = Vec::new();

                let (x, y) = point_to_uncompress(&it.e1);
                tmp.push(Token::Bytes(x));
                tmp.push(Token::Bytes(y));

                let (x, y) = point_to_uncompress(&it.e2);
                tmp.push(Token::Bytes(x));
                tmp.push(Token::Bytes(y));
                ret.push(Token::Array(tmp))
            }
            ret
        };

        let alice_shuffle_deck = {
            let mut ret = Vec::new();
            for it in alice_shuffle_deck.iter() {
                let mut tmp = Vec::new();

                let (x, y) = point_to_uncompress(&it.e1);
                tmp.push(Token::Bytes(x));
                tmp.push(Token::Bytes(y));

                let (x, y) = point_to_uncompress(&it.e2);
                tmp.push(Token::Bytes(x));
                tmp.push(Token::Bytes(y));
                ret.push(Token::Array(tmp))
            }
            ret
        };

        let data = ethabi::encode(&[
            Token::Bytes(verifier_params),
            Token::Array(deck),
            Token::Array(alice_shuffle_deck),
            Token::Bytes(proof),
        ]);
        plonk_verify_shuffle(&data).unwrap()
    }
}
