use std;


#[derive(Debug)]
pub enum Bip39Error {
    InvalidChecksum,
    InvalidWord,
    InvalidKeysize,
    InvalidWordLength,
    EntropyUnavailable(std::io::Error),
    LanguageUnavailable
}

impl From<std::io::Error> for Bip39Error {
    fn from(err: std::io::Error) -> Bip39Error {
        return Bip39Error::EntropyUnavailable(err)
    }
}

impl std::fmt::Display for Bip39Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Bip39Error::InvalidChecksum => {
                write!(f, "Invalid checksum")
            },
            Bip39Error::InvalidWord => {
                write!(f, "Invalid word in phrase")
            },
            Bip39Error::InvalidKeysize => {
                write!(f, "Invalid keysize")
            },
            Bip39Error::InvalidWordLength => {
                write!(f, "Invalid number of words in phrase")
            },
            Bip39Error::EntropyUnavailable(ref err) => {
                write!(f, "Entropy unavailable: {}", err)
            },
            Bip39Error::LanguageUnavailable => {
                write!(f, "Language unavailable")
            },
        }
    }
}


impl std::error::Error for Bip39Error {
    fn description(&self) -> &str {
        match *self {
            Bip39Error::InvalidChecksum => "invalid checksum",
            Bip39Error::InvalidWord => "invalid word in phrase",
            Bip39Error::InvalidKeysize => "invalid keysize",
            Bip39Error::InvalidWordLength => "invalid number of words in phrase",
            Bip39Error::EntropyUnavailable(ref err) => err.description(),
            Bip39Error::LanguageUnavailable => "wrapping key failed",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Bip39Error::InvalidChecksum => None,
            Bip39Error::InvalidWord => None,
            Bip39Error::InvalidKeysize => None,
            Bip39Error::InvalidWordLength => None,
            Bip39Error::EntropyUnavailable(ref err) => Some(err),
            Bip39Error::LanguageUnavailable => None,
        }
    }
}