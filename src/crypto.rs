use ring::digest::{SHA256, digest};
use ring::pbkdf2;

extern crate rand;
use self::rand::{OsRng, Rng};

use ::error::Bip39Error;

static PBKDF2_ROUNDS: u32 = 2048;
static PBKDF2_BYTES: usize = 64;


pub fn sha256(input: &[u8]) -> Vec<u8> {
    let hash = digest(&SHA256, input);

    hash.as_ref().to_vec()
}

pub fn gen_random_bytes(byte_length: usize) -> Result<Vec<u8>, Bip39Error> {
    let mut rng = try!(OsRng::new());
    let entropy = rng.gen_iter::<u8>().take(byte_length).collect::<Vec<u8>>();

    Ok(entropy)
}

pub fn pbkdf2(input: &[u8], salt: String) -> Vec<u8> {
    let mut seed = vec![0u8; PBKDF2_BYTES];
    static PBKDF2_PRF: &'static pbkdf2::PRF = &pbkdf2::HMAC_SHA512;

    pbkdf2::derive(PBKDF2_PRF, PBKDF2_ROUNDS, salt.as_bytes(), input, &mut seed);

    seed
}
