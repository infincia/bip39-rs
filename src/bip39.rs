use std::collections::hash_map::HashMap;

use bitreader::BitReader;
use bit_vec::BitVec;


use data_encoding::hex;


static BIP39_WORDLIST_ENGLISH: &'static str = include_str!("bip39_english.txt");


use ::crypto::{gen_random_bytes, sha256, pbkdf2};
use ::error::Bip39Error;
use ::keytype::KeyType;
use ::language::Language;


#[derive(Debug)]
pub struct Bip39 {
    pub mnemonic: String,
    pub seed: Vec<u8>,
    pub lang: Language
}

impl Bip39 {

    /// Create a new `Bip39` struct from a random key
    ///
    /// When returned, the struct will be filled in with the phrase and the seed value
    /// as 64 bytes raw
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use bip39::{Bip39, KeyType, Language};
    ///
    /// let kt = KeyType::for_word_length(12).unwrap();
    ///
    /// let bip39 = match Bip39::new(&kt, Language::English, "") {
    ///     Ok(b) => b,
    ///     Err(e) => { println!("e: {}", e); return }
    /// };
    ///
    /// let phrase = &bip39.mnemonic;
    /// let seed = &bip39.seed;
    /// println!("phrase: {}", phrase);
    /// ```
    pub fn new<S>(key_type: &KeyType, lang: Language, password: S) -> Result<Bip39, Bip39Error>  where S: Into<String> {
        let entropy_bits = key_type.entropy_bits();
        let entropy = gen_random_bytes(entropy_bits / 8)?;

        Bip39::from_key(&entropy, key_type, lang, password)

    }

    /// Create a new `Bip39` struct from an existing key
    pub fn from_key<S>(bytes : &[u8], key_type: &KeyType, lang: Language, password: S) -> Result<Bip39, Bip39Error>  where S: Into<String> {

        let num_words = key_type.word_length();

        let word_list = Bip39::get_wordlist(&lang);

        let bytes_hash = sha256(bytes.as_ref());

        // we put both the bytes and the hash of the bytes (in that order) into a single vec
        // and then just read 11 bits at a time out of the entire thing `num_words` times. We
        // can do that because:
        //
        // 12 words * 11bits = 132bits
        // 15 words * 11bits = 165bits
        //
        // ... and so on. It grabs the bytes and then the right number of hash bits and no more.

        let mut combined = Vec::from(bytes);
        combined.extend(&bytes_hash);

        let mut reader = BitReader::new(combined.as_ref());

        let mut words: Vec<&str> = Vec::new();
        for _ in 0..num_words {
            let n = reader.read_u16(11);
            words.push(word_list[n.unwrap() as usize].as_ref());
        }

        let mnemonic = words.join(" ");

        Bip39::from_mnemonic(mnemonic, lang, password.into())
    }

    /// Create a `Bip39` struct from an existing mnemonic phrase
    ///
    /// The phrase supplied will be checked for word length and validated according to the checksum
    /// specified in BIP0039
    ///
    /// # Example
    ///
    /// ```
    /// use bip39::{Bip39, KeyType, Language};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let b = Bip39::from_mnemonic(test_mnemonic, Language::English, "").unwrap();
    /// ```
    ///

    pub fn from_mnemonic<S>(mnemonic: S, lang: Language, password: S) -> Result<Bip39, Bip39Error> where S: Into<String> {
        let m = mnemonic.into();
        let p = password.into();
        try!(Bip39::validate(&*m, &lang));

        Ok(Bip39 { mnemonic: (&m).clone(), seed: Bip39::generate_seed(&m.as_bytes(), &p), lang: lang})
    }

    /// Validate a mnemonic phrase
    ///
    /// The phrase supplied will be checked for word length and validated according to the checksum
    /// specified in BIP0039
    ///
    /// # Example
    ///
    /// ```
    /// use bip39::{Bip39, KeyType, Language};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// match Bip39::validate(test_mnemonic, &Language::English) {
    ///     Ok(_) => { println!("valid: {}", test_mnemonic); },
    ///     Err(e) => { println!("e: {}", e); return }
    /// }
    /// ```
    ///
    pub fn validate<S>(mnemonic: S, lang: &Language) -> Result<(), Bip39Error>  where S: Into<String> {
        let m = mnemonic.into();

        let key_type = try!(KeyType::for_mnemonic(&*m));
        let entropy_bits = key_type.entropy_bits();
        let checksum_bits = key_type.checksum_bits();

        let mut word_map: HashMap<String, u16> = HashMap::new();
        let word_list = Bip39::get_wordlist(lang);

        for (i, item) in word_list.into_iter().enumerate() {
            word_map.insert(item, i as u16);
        }

        let mut to_validate: BitVec = BitVec::new();

        for word in m.split(" ").into_iter() {
            let n = match word_map.get(word) {
                Some(n) => n,
                None => return Err(Bip39Error::InvalidWord)
            };
            for i in 0..11 {
                let bit = Bip39::bit_from_u16_as_u11(*n, i);
                to_validate.push(bit);
            }
        }

        let mut checksum_to_validate = BitVec::new();
        &checksum_to_validate.extend((&to_validate).into_iter().skip(entropy_bits).take(checksum_bits));
        assert!(checksum_to_validate.len() == checksum_bits, "invalid checksum size");

        let mut entropy_to_validate = BitVec::new();
        &entropy_to_validate.extend((&to_validate).into_iter().take(entropy_bits));
        assert!(entropy_to_validate.len() == entropy_bits, "invalid entropy size");

        let hash = sha256(entropy_to_validate.to_bytes().as_ref());

        let entropy_hash_to_validate_bits = BitVec::from_bytes(hash.as_ref());


        let mut new_checksum = BitVec::new();
        &new_checksum.extend(entropy_hash_to_validate_bits.into_iter().take(checksum_bits));
        assert!(new_checksum.len() == checksum_bits, "invalid new checksum size");
        if !(new_checksum == checksum_to_validate) {
            return Err(Bip39Error::InvalidChecksum)
        }

        Ok(())
    }

    pub fn to_hex(&self) -> String {
        let seed: &[u8] = self.seed.as_ref();
        let hex = hex::encode(seed);

        hex
    }


    fn get_wordlist(lang: &Language) -> Vec<String> {
        let lang_words = match *lang {
            Language::English => BIP39_WORDLIST_ENGLISH
        };

        let words: Vec<String> = lang_words.split_whitespace()
            .map(|s| s.into())
            .collect();

        words
    }


    fn generate_seed(entropy: &[u8], password: &str) -> Vec<u8> {
        let salt = format!("mnemonic{}", password);
        let seed = pbkdf2(entropy, salt);

        seed
    }

    fn bit_from_u16_as_u11(input: u16, position: u16) -> bool {
        if position < 11 {
            input & (1 << (10 - position)) != 0
        } else {
            false
        }
    }

}
