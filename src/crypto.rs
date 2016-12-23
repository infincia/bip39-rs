extern crate crypto;
use self::crypto::hmac::Hmac;
use self::crypto::digest::Digest;
use self::crypto::sha2::{Sha256, Sha512};
use self::crypto::pbkdf2::pbkdf2 as rcrypto_pbkdf2;

extern crate rand;
use self::rand::{OsRng, Rng};

use ::error::Bip39Error;

static PBKDF2_ROUNDS: u32 = 2048;
static PBKDF2_BYTES: usize = 64;


pub fn sha256(input: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.input(input);

    hash.result_str()
}

pub fn gen_random_bytes(byte_length: usize) -> Result<Vec<u8>, Bip39Error> {
    let mut rng = try!(OsRng::new());
    let entropy = rng.gen_iter::<u8>().take(byte_length).collect::<Vec<u8>>();

    Ok(entropy)
}

pub fn pbkdf2(input: &[u8], salt: String) -> Vec<u8> {
    let mut hmac = Hmac::new(Sha512::new(), input);
    let mut seed = vec![0u8; PBKDF2_BYTES];
    rcrypto_pbkdf2(&mut hmac, salt.as_bytes(), PBKDF2_ROUNDS, &mut seed);

    seed
}
