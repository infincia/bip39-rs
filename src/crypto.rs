//! These are internal helper functions used when creating a new [`Mnemonic`][Mnemonic], and when turning a [`Mnemonic`][Mnemonic]
//! into a [`Seed`][Seed].
//!
//! [Mnemonic]: ../mnemonic/struct.Mnemonic.html
//! [Seed]: ../seed/struct.Seed.html
//!


use ring::digest::{self, digest};
use ring::pbkdf2;

extern crate rand;
use self::rand::{OsRng, Rng};

use ::error::Error;

static PBKDF2_ROUNDS: u32 = 2048;
static PBKDF2_BYTES: usize = 64;


/// SHA256 helper function, internal to the crate
///
pub(crate) fn sha256(input: &[u8]) -> Vec<u8> {

    static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;

    let hash = digest(DIGEST_ALG, input);

    hash.as_ref().to_vec()
}

/// Random byte generator, used to create new mnemonics
///
pub(crate) fn gen_random_bytes(byte_length: usize) -> Result<Vec<u8>, Error> {

    let mut rng = OsRng::new()?;
    let entropy = rng.gen_iter::<u8>().take(byte_length).collect::<Vec<u8>>();

    Ok(entropy)
}

/// PBKDF2 helper, used to generate [`Seed`][Seed] from [`Mnemonic`][Mnemonic]
///
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Seed]: ../seed/struct.Seed.html
/// 
pub(crate) fn pbkdf2(input: &[u8],
              salt: String) -> Vec<u8> {

    let mut seed = vec![0u8; PBKDF2_BYTES];

    static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA512;

    pbkdf2::derive(DIGEST_ALG, PBKDF2_ROUNDS, salt.as_bytes(), input, &mut seed);

    seed
}
