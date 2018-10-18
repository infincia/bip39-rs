//! These are internal helper functions used when creating a new [`Mnemonic`][Mnemonic], and when turning a [`Mnemonic`][Mnemonic]
//! into a [`Seed`][Seed].
//!
//! [Mnemonic]: ../mnemonic/struct.Mnemonic.html
//! [Seed]: ../seed/struct.Seed.html
//!


use ring::digest::{self, digest, Digest};
use ring::pbkdf2;

extern crate rand;
use self::rand::{OsRng, Rng};

const PBKDF2_ROUNDS: u32 = 2048;
const PBKDF2_BYTES: usize = 64;


/// SHA256 helper function, internal to the crate
///
pub(crate) fn sha256(input: &[u8]) -> Digest {

    static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;

    digest(DIGEST_ALG, input)
}

/// Random byte generator, used to create new mnemonics
///
pub(crate) fn gen_random_bytes(byte_length: usize) -> Vec<u8> {
    let mut rng = OsRng::new().expect("Unable to initialize a random number generator!");
    let mut bytes = Vec::with_capacity(byte_length);

    // Calling into the generator is expensive, so let's collect 64 bits of entropy at a time
    let mut source = 0u64;

    for i in 0..byte_length {
        if i % 8 == 0 {
            source = rng.gen();
        }

        bytes.push(source as u8);

        source >>= 8;
    }

    bytes
}

/// PBKDF2 helper, used to generate [`Seed`][Seed] from [`Mnemonic`][Mnemonic]
///
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Seed]: ../seed/struct.Seed.html
///
pub(crate) fn pbkdf2(input: &[u8], salt: &str) -> Vec<u8> {
    let mut seed = vec![0u8; PBKDF2_BYTES];

    static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA512;

    pbkdf2::derive(DIGEST_ALG, PBKDF2_ROUNDS, salt.as_bytes(), input, &mut seed);

    seed
}
