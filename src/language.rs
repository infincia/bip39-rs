use ::error::{ErrorKind, Result};
use std::collections::HashMap;

mod lazy {
    use super::HashMap;

    /// lazy generation of the word list
    fn gen_wordlist(lang_words: &str) -> Vec<&str> {
        lang_words.split_whitespace().collect()
    }

    /// lazy generation of the word map
    fn gen_wordmap(wordlist: &[&'static str]) -> HashMap<&'static str, u16> {
        wordlist
            .iter()
            .enumerate()
            .map(|(i, item)| (*item, i as u16))
            .collect()
    }

    static BIP39_WORDLIST_ENGLISH: &str = include_str!("bip39_english.txt");

    lazy_static! {
        pub static ref VEC_BIP39_WORDLIST_ENGLISH: Vec<&'static str> = gen_wordlist(BIP39_WORDLIST_ENGLISH);

        pub static ref HASHMAP_BIP39_WORDMAP_ENGLISH: HashMap<&'static str, u16> = gen_wordmap(&VEC_BIP39_WORDLIST_ENGLISH);
    }
}

/// The language determines which words will be used in a mnemonic phrase, but also indirectly
/// determines the binary value of each word when a [`Mnemonic`][Mnemonic] is turned into a [`Seed`][Seed].
///
/// These are not of much use right now, and may even be removed from the crate, as there is no
/// official language specified by the standard except English.
///
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Seed]: ../seed/struct.Seed.html
#[derive(Debug, Clone, Copy)]
pub enum Language {
    English
}

impl Language {
    /// Get the [`Language`][Language] value for a specific locale
    ///
    /// Not used much at the moment as the standard specifies english
    ///
    /// # Example
    /// ```
    /// use bip39::{Language};
    ///
    /// let lang = Language::for_locale("en_US.UTF-8").unwrap();
    ///
    /// ```
    /// [Language]: ../language/struct.Language.html
    pub fn for_locale(locale: &str) -> Result<Language> {
        let lang = match locale {
            "en_US.UTF-8" => Language::English,
            "en_GB.UTF-8" => Language::English,

            _ => bail!(ErrorKind::LanguageUnavailable)
        };

        Ok(lang)
    }

    /// Get the word list for this language
    pub fn get_wordlist(&self) -> &'static Vec<&'static str> {

        match *self {
            Language::English => &lazy::VEC_BIP39_WORDLIST_ENGLISH
        }
    }

    /// Get a [`HashMap`][HashMap] that allows word -> index lookups in the word list
    ///
    /// The index of an individual word in the word list is used as the binary value of that word
    /// when the phrase is turned into a [`Seed`][Seed].
    ///
    /// [HashMap]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    /// [Seed]: ../seed/struct.Seed.html
    pub fn get_wordmap(&self) -> &'static HashMap<&'static str, u16> {

        match *self {
            Language::English => &lazy::HASHMAP_BIP39_WORDMAP_ENGLISH
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::English
    }
}
