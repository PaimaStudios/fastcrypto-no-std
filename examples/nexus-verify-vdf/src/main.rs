#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::print;

use fastcrypto_vdf::{
    rsa_group::{
        RSAGroupElement,
        modulus::{RSAModulus},
    },
    vdf::{VDF, pietrzak::{PietrzaksVDF, DEFAULT_CHALLENGE_SIZE_IN_BYTES}},
};
use fastcrypto::hash::{HashFunction, Keccak256};
use num_bigint::{BigInt, BigUint};
use core::str::FromStr;
extern crate alloc;
use alloc::{vec, vec::Vec};

use core::result::{Result::Ok, Result::Err};

#[nexus_rt::main]
fn main() {
    let AMAZON_MODULUS_2048: RSAModulus = RSAModulus::from_str("22529839904807742196558773392430766620630713202204326167346456925862066285712069978308045976033918808540171076811098215136401323342247576789054764683787147408289170989302937775178809187827657352584557953877946352196797789035355954596527030584944622221752357105572088106020206921431118198373122638305846252087992561841631797199384157902018140720267433956687491591657652730221337591680012205319549572614035105482287002884850178224609018864719685310905426619874727796905080238179726224664042154200651710137931048812546957419686875805576245376866031854569863410951649630469236463991472642618512857920826701027482532358669").unwrap();
    let iterations = 2 as u64;
    let input = RSAGroupElement::new(7u32.into(), &AMAZON_MODULUS_2048);
    // these are copied from the output or running the prove-vdf crate in a non-zk setting e.g.:
    // output: 2401
    // proof: [49]
    let output = RSAGroupElement::new(BigUint::from_str("2401").unwrap(), &AMAZON_MODULUS_2048);
    let proofs = vec![
        RSAGroupElement::new(BigUint::from_str("49").unwrap(), &AMAZON_MODULUS_2048)
    ];
    let vdf: PietrzaksVDF<RSAGroupElement> = PietrzaksVDF::new(&AMAZON_MODULUS_2048, iterations);
    match vdf.verify(&input, &output, &proofs) {
        Ok(()) => {
            print!("success!");
        }
        Err(_) => {
            panic!("error!");
        }
    }
}
