use std::collections::hash_map::HashMap;

extern crate rustc_serialize;
use self::rustc_serialize::hex::{FromHex, ToHex};

extern crate bitreader;
use self::bitreader::BitReader;

extern crate bit_vec;
use self::bit_vec::BitVec;



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

impl ToHex for Bip39 {
    fn to_hex(&self) -> String {
        let seed: &[u8] = self.seed.as_ref();
        let hex = seed.to_hex();

        hex
    }
}

impl Bip39 {

    /// Generates a new `Bip39` struct
    ///
    /// When returned, the struct will be filled in with the phrase and the seed value
    /// as 64byte/512bit hex
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
    /// Ok(b) => b,
    /// Err(e) => { println!("e: {:?}", e); return }
    /// };
    ///
    /// let phrase = &bip39.mnemonic;
    /// let seed = &bip39.seed;
    /// println!("phrase: {}", phrase);
    /// ```
    pub fn new(key_type: &KeyType, lang: Language, password: &str) -> Result<Bip39, Bip39Error> {
        let entropy_bits = key_type.entropy_bits();
        let total_bits = key_type.total_bits();

        let num_words = key_type.word_length();

        let word_list = Bip39::get_wordlist(&lang);

        let entropy = try!(gen_random_bytes(entropy_bits / 8));

        let entropy_hash = sha256(entropy.as_ref()).from_hex().unwrap();

        // we put both the entropy and the hash of the entropy (in that order) into a single vec
        // and then just read 11 bits at a time out of the entire thing `num_words` times. We
        // can do that because:
        //
        // 12 words * 11bits = 132bits
        // 15 words * 11bits = 165bits
        //
        // ... and so on. It grabs the entropy and then the right number of hash bits and no more.

        let mut combined = Vec::from(entropy);
        combined.extend(&entropy_hash);

        let mut reader = BitReader::new(combined.as_ref());

        let mut words: Vec<&str> = Vec::new();
        for _ in 0..num_words {
            let n = reader.read_u16(11);
            words.push(word_list[n.unwrap() as usize].as_ref());
        }

        let mnemonic = words.join(" ");

        Bip39::from_mnemonic(mnemonic, lang, password.to_string())
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
    /// let b = Bip39::from_mnemonic(test_mnemonic.to_string(), Language::English, "".to_string()).unwrap();
    /// ```
    ///

    pub fn from_mnemonic(mnemonic: String, lang: Language, password: String) -> Result<Bip39, Bip39Error> {
        try!(Bip39::validate(&mnemonic, &lang));

        Ok(Bip39 { mnemonic: mnemonic.clone(), seed: Bip39::generate_seed(&mnemonic.as_bytes(), &password), lang: lang})
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
    ///  Ok(_) => {},
    ///  Err(e) => {}
    /// }
    /// ```
    ///
    pub fn validate(mnemonic: &str, lang: &Language) -> Result<(), Bip39Error> {
        let key_type = try!(KeyType::for_mnemonic(&mnemonic));
        let entropy_bits = key_type.entropy_bits();
        let checksum_bits = key_type.checksum_bits();

        let mut word_map: HashMap<String, u16> = HashMap::new();
        let word_list = Bip39::get_wordlist(lang);

        for (i, item) in word_list.into_iter().enumerate() {
            word_map.insert(item, i as u16);
        }

        let mut to_validate: BitVec = BitVec::new();

        for word in mnemonic.split(" ").into_iter() {
            let n = word_map.get(word).unwrap();
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

        let hash = sha256(entropy_to_validate.to_bytes().as_ref()).from_hex().unwrap();

        let entropy_hash_to_validate_bits = BitVec::from_bytes(hash.as_ref());


        let mut new_checksum = BitVec::new();
        &new_checksum.extend(entropy_hash_to_validate_bits.into_iter().take(checksum_bits));
        assert!(new_checksum.len() == checksum_bits, "invalid new checksum size");
        if !(new_checksum == checksum_to_validate) {
            return Err(Bip39Error::InvalidChecksum)
        }

        Ok(())
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