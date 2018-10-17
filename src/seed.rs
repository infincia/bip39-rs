use crypto::pbkdf2;

use mnemonic::Mnemonic;
use data_encoding::HEXUPPER;

/// The secret value used to derive HD wallet addresses from a [`Mnemonic`][Mnemonic] phrase.
///
/// Because it is not possible to create a [`Mnemonic`][Mnemonic] instance that is invalid, it is
/// therefore impossible to have a [`Seed`][Seed] instance that is invalid. This guarantees that only
/// a valid, intact mnemonic phrase can be used to derive HD wallet addresses.
///
/// To get the raw byte value use [`Seed::as_bytes()`][Seed::as_bytes()], or the hex representation
/// with [`Seed::to_hex()`][Seed::to_hex()]. These can be used to derive HD wallet addresses using
/// another crate (deriving HD wallet addresses is outside the scope of this crate and the BIP39
/// standard).
#[derive(Debug, Clone)]
pub struct Seed {
    bytes: Vec<u8>,
}

impl Seed {
    /// Generates the seed from the [`Mnemonic`][Mnemonic] and the password.
    pub fn new(mnemonic: &Mnemonic, password: &str) -> Self {

        let salt = format!("mnemonic{}", password);
        let bytes = pbkdf2(mnemonic.entropy(), &salt);

        Self {
            bytes,
        }
    }

    /// Get the seed value as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get the seed value as a hex `String`
    pub fn to_hex(&self) -> String {
        HEXUPPER.encode(&self.bytes)
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
