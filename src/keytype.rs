use ::error::Bip39Error;


#[derive(Debug)]
pub enum KeyType {
    Key128,
    Key160,
    Key192,
    Key224,
    Key256
}

impl KeyType {
    /// Create a new `KeyType` struct for a specific key size
    ///
    /// This allows code to request key types for arbitrary key lengths, rather than
    /// pattern matching on their own.
    ///
    /// Note that specifying a bit length not provided for by BIP0039 will return an error.
    ///
    /// # Example
    /// ```
    /// use bip39::{KeyType};
    ///
    /// let kt = KeyType::for_keysize(128).unwrap();
    /// ```
    pub fn for_keysize(size: usize) -> Result<KeyType, Bip39Error> {
        let kt = match size {
            128 => KeyType::Key128,
            160 => KeyType::Key160,
            192 => KeyType::Key192,
            224 => KeyType::Key224,
            256 => KeyType::Key256,
            _ => { return Err(Bip39Error::InvalidKeysize) }
        };

        Ok(kt)
    }

    /// Create a new `KeyType` struct for a specific phrase length
    ///
    /// This allows code to request key types for arbitrary word lengths rather than
    /// pattern matching on their own.
    ///
    /// Note that specifying a word length not provided for by BIP0039 will return an error.
    ///
    /// # Example
    /// ```
    /// use bip39::{KeyType};
    ///
    /// let kt = KeyType::for_word_length(12).unwrap();
    /// ```
    pub fn for_word_length(length: usize) -> Result<KeyType, Bip39Error> {
        let kt = match length {
            12 => KeyType::Key128,
            15 => KeyType::Key160,
            18 => KeyType::Key192,
            21 => KeyType::Key224,
            24 => KeyType::Key256,
            _ => { return Err(Bip39Error::InvalidWordLength) }
        };

        Ok(kt)
    }

    /// Get a `KeyType` struct matching an existing BIP0039 phrase
    ///
    /// This allows code to get information about a mnemonic phrase, like the entropy value of
    /// the seed that can be generated from it
    ///
    /// # Example
    /// ```
    /// use bip39::{KeyType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let key_type = KeyType::for_mnemonic(test_mnemonic).unwrap();
    ///
    /// let entropy_bits = key_type.entropy_bits();
    /// ```
    pub fn for_mnemonic<S>(mnemonic: S) -> Result<KeyType, Bip39Error> where S: Into<String> {
        let m = mnemonic.into();

        let v: Vec<&str> = m.split(" ").into_iter().collect();

        let kt = match v.len() {
            12 => KeyType::Key128,
            15 => KeyType::Key160,
            18 => KeyType::Key192,
            21 => KeyType::Key224,
            24 => KeyType::Key256,
            _ => { return Err(Bip39Error::InvalidWordLength) }
        };

        Ok(kt)
    }

    /// Return the number of entropy+checksum bits for this key type
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{KeyType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let key_type = KeyType::for_mnemonic(test_mnemonic).unwrap();
    ///
    /// let total_bits = key_type.total_bits();
    /// ```
    pub fn total_bits(&self) -> usize {
        let total_bits: usize = match *self {
            KeyType::Key128 => 132,
            KeyType::Key160 => 165,
            KeyType::Key192 => 198,
            KeyType::Key224 => 231,
            KeyType::Key256 => 264
        };

        total_bits
    }

    /// Return the number of entropy bits for this key type
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{KeyType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let key_type = KeyType::for_mnemonic(test_mnemonic).unwrap();
    ///
    /// let entropy_bits = key_type.entropy_bits();
    /// ```
    pub fn entropy_bits(&self) -> usize {
        let entropy_bits: usize = match *self {
            KeyType::Key128 => 128,
            KeyType::Key160 => 160,
            KeyType::Key192 => 192,
            KeyType::Key224 => 224,
            KeyType::Key256 => 256
        };

        entropy_bits
    }

    /// Return the number of checksum bits for this key type
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{KeyType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let key_type = KeyType::for_mnemonic(test_mnemonic).unwrap();
    ///
    /// let checksum_bits = key_type.checksum_bits();
    /// ```
    pub fn checksum_bits(&self) -> usize {
        let checksum_bits: usize = match *self {
            KeyType::Key128 => 4,
            KeyType::Key160 => 5,
            KeyType::Key192 => 6,
            KeyType::Key224 => 7,
            KeyType::Key256 => 8
        };

        checksum_bits
    }

    /// Return the proper number of words for this key type
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{KeyType};
    ///
    /// let key_type = KeyType::Key128;
    ///
    /// let word_length = key_type.word_length();
    /// ```
    pub fn word_length(&self) -> usize {
        let word_length: usize = match *self {
            KeyType::Key128 => 12,
            KeyType::Key160 => 15,
            KeyType::Key192 => 18,
            KeyType::Key224 => 21,
            KeyType::Key256 => 24
        };

        word_length
    }
}