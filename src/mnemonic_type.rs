use ::error::{Error, ErrorKind};
use std::fmt;

/// Determines the number of words that will be present in a [`Mnemonic`][Mnemonic] phrase
///
/// Also directly affects the amount of entropy that will be used to create a [`Mnemonic`][Mnemonic],
/// and therefore the cryptographic strength of the HD wallet keys/addresses that can be derived from
/// it using the [`Seed`][Seed].
///
/// For example, a 12 word mnemonic phrase is essentially a friendly representation of a 128-bit key,
/// while a 24 word mnemonic phrase is essentially a 256-bit key.
///
/// If you know you want a specific phrase length, you can use the enum variant directly, for example
/// `MnemonicType::Type12Words`.
///
/// You can also get a `MnemonicType` that corresponds to one of the standard BIP39 key sizes by
/// passing arbitrary `usize` values:
///
/// ```
/// use bip39::{MnemonicType};
///
/// let mnemonic_type = MnemonicType::for_key_size(128).unwrap();
///
/// ```
///
/// [MnemonicType]: ../mnemonic_type/struct.MnemonicType.html
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Seed]: ../seed/struct.Seed.html
///
#[derive(Debug, Copy, Clone)]
pub enum MnemonicType {
    Type12Words,
    Type15Words,
    Type18Words,
    Type21Words,
    Type24Words
}

impl MnemonicType {
    /// Get a `MnemonicType` for a mnemonic phrase with a specific number of words
    ///
    /// Specifying a word count not provided for by the BIP39 standard will return an `Error`
    /// of kind `ErrorKind::InvalidWordLength`.
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let mnemonic_type = MnemonicType::for_word_count(12).unwrap();
    /// ```
    pub fn for_word_count(size: usize) -> Result<MnemonicType, Error> {

        let mnemonic_type = match size {
            12 => MnemonicType::Type12Words,
            15 => MnemonicType::Type15Words,
            18 => MnemonicType::Type18Words,
            21 => MnemonicType::Type21Words,
            24 => MnemonicType::Type24Words,
            _ => { return Err(ErrorKind::InvalidWordLength.into()) }
        };

        Ok(mnemonic_type)
    }

    /// Get a `MnemonicType` for a mnemonic phrase representing the given key size as bits
    ///
    /// Specifying a key size not provided for by the BIP39 standard will return an `Error`
    /// of kind `ErrorKind::InvalidKeysize`.
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let mnemonic_type = MnemonicType::for_key_size(128).unwrap();
    /// ```
    pub fn for_key_size(size: usize) -> Result<MnemonicType, Error> {

        let mnemonic_type = match size {
            128 => MnemonicType::Type12Words,
            160 => MnemonicType::Type15Words,
            192 => MnemonicType::Type18Words,
            224 => MnemonicType::Type21Words,
            256 => MnemonicType::Type24Words,
            _ => { return Err(ErrorKind::InvalidKeysize.into()) }
        };

        Ok(mnemonic_type)
    }

    /// Get a `MnemonicType` for an existing mnemonic phrase
    ///
    /// This can be used when you need information about a mnemonic phrase based on the number of
    /// words, for example you can get the entropy value using [`MnemonicType::entropy_bits`][MnemonicType::entropy_bits()].
    ///
    /// Specifying a phrase that does not match one of the standard BIP39 phrase lengths will return
    /// an `Error` of kind `ErrorKind::InvalidWordLength`. The phrase will not be validated in any
    /// other way.
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::for_phrase(test_mnemonic).unwrap();
    ///
    /// let entropy_bits = mnemonic_type.entropy_bits();
    /// ```
    ///
    /// [MnemonicType::entropy_bits()]: ../mnemonic_type/struct.MnemonicType.html#method.entropy_bits
    pub fn for_phrase<S>(phrase: S) -> Result<MnemonicType, Error> where S: Into<String> {

        let m = phrase.into();

        let v: Vec<&str> = m.split(" ").into_iter().collect();

        let mnemonic_type = match v.len() {
            12 => MnemonicType::Type12Words,
            15 => MnemonicType::Type15Words,
            18 => MnemonicType::Type18Words,
            21 => MnemonicType::Type21Words,
            24 => MnemonicType::Type24Words,
            _ => { return Err(ErrorKind::InvalidWordLength.into()) }
        };

        Ok(mnemonic_type)
    }

    /// Return the number of entropy+checksum bits
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::for_phrase(test_mnemonic).unwrap();
    ///
    /// let total_bits = mnemonic_type.total_bits();
    /// ```
    pub fn total_bits(&self) -> usize {

        let total_bits: usize = match *self {
            MnemonicType::Type12Words => 132,
            MnemonicType::Type15Words => 165,
            MnemonicType::Type18Words => 198,
            MnemonicType::Type21Words => 231,
            MnemonicType::Type24Words => 264
        };

        total_bits
    }

    /// Return the number of entropy bits
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::for_phrase(test_mnemonic).unwrap();
    ///
    /// let entropy_bits = mnemonic_type.entropy_bits();
    /// ```
    pub fn entropy_bits(&self) -> usize {

        let entropy_bits: usize = match *self {
            MnemonicType::Type12Words => 128,
            MnemonicType::Type15Words => 160,
            MnemonicType::Type18Words => 192,
            MnemonicType::Type21Words => 224,
            MnemonicType::Type24Words => 256
        };

        entropy_bits
    }

    /// Return the number of checksum bits
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::for_phrase(test_mnemonic).unwrap();
    ///
    /// let checksum_bits = mnemonic_type.checksum_bits();
    /// ```
    pub fn checksum_bits(&self) -> usize {

        let checksum_bits: usize = match *self {
            MnemonicType::Type12Words => 4,
            MnemonicType::Type15Words => 5,
            MnemonicType::Type18Words => 6,
            MnemonicType::Type21Words => 7,
            MnemonicType::Type24Words => 8
        };

        checksum_bits
    }

    /// Return the number of words
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let mnemonic_type = MnemonicType::Type12Words;
    ///
    /// let word_count = mnemonic_type.word_count();
    /// ```
    pub fn word_count(&self) -> usize {

        let word_count: usize = match *self {
            MnemonicType::Type12Words => 12,
            MnemonicType::Type15Words => 15,
            MnemonicType::Type18Words => 18,
            MnemonicType::Type21Words => 21,
            MnemonicType::Type24Words => 24
        };

        word_count
    }
}

impl Default for MnemonicType {
    fn default() -> MnemonicType {
        MnemonicType::Type12Words
    }
}

impl fmt::Display for MnemonicType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} words ({}bits)", self.word_count(), self.entropy_bits())
    }
}
