use hashbrown::HashMap;
use error::{ErrorKind, Result};
use util::{Bits11, Bits};

pub struct WordMap {
    inner: HashMap<&'static str, Bits11>

}

pub struct WordList {
    inner: Vec<&'static str>
}

impl WordMap {
    pub fn get_bits(&self, word: &str) -> Result<Bits11> {
        match self.inner.get(word) {
            Some(n) => Ok(*n),
            None => bail!(ErrorKind::InvalidWord)
        }
    }
}

impl WordList {
    pub fn get_word(&self, bits: Bits11) -> &'static str {
        self.inner[bits.bits() as usize]
    }
}

mod lazy {
    use super::{Bits11, WordList, WordMap};
    use once_cell::sync::Lazy;

    /// lazy generation of the word list
    fn gen_wordlist(lang_words: &'static str) -> WordList {
        let inner: Vec<_> = lang_words.split_whitespace().collect();

        debug_assert!(inner.len() == 2048, "Invalid wordlist length");

        WordList {
            inner
        }
    }

    /// lazy generation of the word map
    fn gen_wordmap(wordlist: &WordList) -> WordMap {
        let inner = wordlist.inner
                            .iter()
                            .enumerate()
                            .map(|(i, item)| (*item, Bits11::from(i as u16)))
                            .collect();

        WordMap {
            inner
        }
    }

    pub static WORDLIST_ENGLISH: Lazy<WordList> = sync_lazy!{ gen_wordlist(include_str!("langs/english.txt")) };
    pub static WORDLIST_CHINESE_SIMPLIFIED: Lazy<WordList> = sync_lazy!{ gen_wordlist(include_str!("langs/chinese_simplified.txt")) };
    pub static WORDLIST_CHINESE_TRADITIONAL: Lazy<WordList> = sync_lazy!{ gen_wordlist(include_str!("langs/chinese_traditional.txt")) };
    pub static WORDLIST_FRENCH: Lazy<WordList> = sync_lazy!{ gen_wordlist(include_str!("langs/french.txt")) };
    pub static WORDLIST_ITALIAN: Lazy<WordList> = sync_lazy!{ gen_wordlist(include_str!("langs/italian.txt")) };
    pub static WORDLIST_JAPANESE: Lazy<WordList> = sync_lazy!{ gen_wordlist(include_str!("langs/japanese.txt")) };
    pub static WORDLIST_KOREAN: Lazy<WordList> = sync_lazy!{ gen_wordlist(include_str!("langs/korean.txt")) };
    pub static WORDLIST_SPANISH: Lazy<WordList> = sync_lazy!{ gen_wordlist(include_str!("langs/spanish.txt")) };

    pub static WORDMAP_ENGLISH: Lazy<WordMap> = sync_lazy!{ gen_wordmap(&WORDLIST_ENGLISH) };
    pub static WORDMAP_CHINESE_SIMPLIFIED: Lazy<WordMap> = sync_lazy!{  gen_wordmap(&WORDLIST_CHINESE_SIMPLIFIED) };
    pub static WORDMAP_CHINESE_TRADITIONAL: Lazy<WordMap> = sync_lazy!{ gen_wordmap(&WORDLIST_CHINESE_TRADITIONAL) };
    pub static WORDMAP_FRENCH: Lazy<WordMap> = sync_lazy!{ gen_wordmap(&WORDLIST_FRENCH) };
    pub static WORDMAP_ITALIAN: Lazy<WordMap> = sync_lazy!{ gen_wordmap(&WORDLIST_ITALIAN) };
    pub static WORDMAP_JAPANESE: Lazy<WordMap> = sync_lazy!{ gen_wordmap(&WORDLIST_JAPANESE) };
    pub static WORDMAP_KOREAN: Lazy<WordMap> = sync_lazy!{ gen_wordmap(&WORDLIST_KOREAN) };
    pub static WORDMAP_SPANISH: Lazy<WordMap> = sync_lazy!{ gen_wordmap(&WORDLIST_SPANISH) };

}

/// The language determines which words will be used in a mnemonic phrase, but also indirectly
/// determines the binary value of each word when a [`Mnemonic`][Mnemonic] is turned into a [`Seed`][Seed].
///
/// These are not of much use right now, and may even be removed from the crate, as there is no
/// official language specified by the standard except English.
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [Seed]: ./seed/struct.Seed.html
#[derive(Debug, Clone, Copy)]
pub enum Language {
    English,
    ChineseSimplified,
    ChineseTraditional,
    French,
    Italian,
    Japanese,
    Korean,
    Spanish,
}

impl Language {
    /// Get the word list for this language
    pub fn wordlist(&self) -> &'static WordList {
        match *self {
            Language::English => &lazy::WORDLIST_ENGLISH,
            Language::ChineseSimplified => &lazy::WORDLIST_CHINESE_SIMPLIFIED,
            Language::ChineseTraditional => &lazy::WORDLIST_CHINESE_TRADITIONAL,
            Language::French => &lazy::WORDLIST_FRENCH,
            Language::Italian => &lazy::WORDLIST_ITALIAN,
            Language::Japanese => &lazy::WORDLIST_JAPANESE,
            Language::Korean => &lazy::WORDLIST_KOREAN,
            Language::Spanish => &lazy::WORDLIST_SPANISH,
        }
    }

    /// Get a [`WordMap`][WordMap] that allows word -> index lookups in the word list
    ///
    /// The index of an individual word in the word list is used as the binary value of that word
    /// when the phrase is turned into a [`Seed`][Seed].
    pub fn wordmap(&self) -> &'static WordMap {
        match *self {
            Language::English => &lazy::WORDMAP_ENGLISH,
            Language::ChineseSimplified => &lazy::WORDMAP_CHINESE_SIMPLIFIED,
            Language::ChineseTraditional => &lazy::WORDMAP_CHINESE_TRADITIONAL,
            Language::French => &lazy::WORDMAP_FRENCH,
            Language::Italian => &lazy::WORDMAP_ITALIAN,
            Language::Japanese => &lazy::WORDMAP_JAPANESE,
            Language::Korean => &lazy::WORDMAP_KOREAN,
            Language::Spanish => &lazy::WORDMAP_SPANISH,
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::English
    }
}
