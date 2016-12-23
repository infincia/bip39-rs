use std;


#[derive(Debug)]
pub enum Bip39Error {
    InvalidChecksum,
    InvalidKeysize,
    InvalidWordLength,
    EntropyUnavailable,
    LanguageUnavailable
}

impl From<std::io::Error> for Bip39Error {
    fn from(_: std::io::Error) -> Bip39Error {
        return Bip39Error::EntropyUnavailable
    }
}