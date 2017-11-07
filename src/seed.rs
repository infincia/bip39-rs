use ::crypto::{pbkdf2};

use data_encoding::HEXUPPER;

/// The secret value used to derive HD wallet addresses from a [`Mnemonic`][Mnemonic] phrase.
///
/// It cannot be created directly, you must create a [`Mnemonic`][Mnemonic] instance and get the seed from
/// it with [`Mnemonic::get_seed()`][Mnemonic::get_seed()].
///
/// Because it is not possible to create a [`Mnemonic`][Mnemonic] instance that is invalid, it is
/// therefore impossible to have a [`Seed`][Seed] instance that is invalid. This guarantees that only
/// a valid, intact mnemonic phrase can be used to derive HD wallet addresses.
///
/// To get the raw byte value use [`Seed::as_bytes()`][Seed::as_bytes()], or the hex representation
/// with [`Seed::as_hex()`][Seed::as_hex()]. These can be used to derive HD wallet addresses using
/// another crate (deriving HD wallet addresses is outside the scope of this crate and the BIP39
/// standard).
///
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Mnemonic::get_seed()]: ../mnemonic/struct.Mnemonic.html#method.get_seed
/// [Seed]: ../seed/struct.Seed.html
/// [Seed::as_bytes()]: ../seed/struct.Seed.html#method.as_bytes
/// [Seed::as_hex()]: ../seed/struct.Seed.html#method.as_hex
///
#[derive(Debug, Clone)]
pub struct Seed {
    bytes: Vec<u8>,
    hex: String,

}

impl Seed {

    /// Generates the seed from the original entropy used to create the [`Mnemonic`][Mnemonic] and the password.
    ///
    /// Cannot be used outside the crate, in order to guarantee correctness
    /// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
    pub(crate) fn generate(entropy: &[u8],
                           password: &str) -> Seed {

        let salt = format!("mnemonic{}", password);
        let seed_value = pbkdf2(entropy, salt);
        let hex = HEXUPPER.encode(seed_value.as_ref());

        Seed {
            bytes: seed_value,
            hex: hex,
        }
    }

    /// Get the seed value as a slice
    pub fn as_bytes(&self) -> &[u8] {

        self.bytes.as_ref()

    }

    /// Get the seed value as a hex string
    pub fn as_hex(&self) -> &str {

        self.hex.as_ref()
    }

    /// Get an owned [`Seed`][Seed] from this instance
    ///
    /// Note: this clones the Seed
    /// [Seed]: ../seed/struct.Seed.html
    pub fn to_owned(&self) -> Seed {

        Seed {
            bytes: self.bytes.clone(),
            hex: self.hex.clone(),
        }
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {

        self.as_bytes()
    }
}

impl AsRef<str> for Seed {
    fn as_ref(&self) -> &str {
        
        self.as_hex()
    }
}
