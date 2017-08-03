use ring::digest::{self, digest};
use ring::pbkdf2;

extern crate rand;
use self::rand::{OsRng, Rng};

use ::error::Error;

static PBKDF2_ROUNDS: u32 = 2048;
static PBKDF2_BYTES: usize = 64;


pub fn sha256(input: &[u8]) -> Vec<u8> {

    static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;

    let hash = digest(DIGEST_ALG, input);

    hash.as_ref().to_vec()
}

pub fn gen_random_bytes(byte_length: usize) -> Result<Vec<u8>, Error> {

    let mut rng = try!(OsRng::new());
    let entropy = rng.gen_iter::<u8>().take(byte_length).collect::<Vec<u8>>();

    Ok(entropy)
}

pub fn pbkdf2(input: &[u8], salt: String) -> Vec<u8> {

    let mut seed = vec![0u8; PBKDF2_BYTES];

    static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA512;

    pbkdf2::derive(DIGEST_ALG, PBKDF2_ROUNDS, salt.as_bytes(), input, &mut seed);

    seed
}
