use rustc_hash::FxHashMap;
use error::{ErrorKind, Result};
use util::{Bits11, Bits};

pub struct WordMap {
    inner: FxHashMap<&'static str, Bits11>
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

    lazy_static! {
        pub static ref WORDLIST_ENGLISH: WordList = gen_wordlist(include_str!("langs/english.txt"));
        pub static ref WORDLIST_CHINESE_SIMPLIFIED: WordList = gen_wordlist(include_str!("langs/chinese_simplified.txt"));
        pub static ref WORDLIST_CHINESE_TRADITIONAL: WordList = gen_wordlist(include_str!("langs/chinese_traditional.txt"));
        pub static ref WORDLIST_FRENCH: WordList = gen_wordlist(include_str!("langs/french.txt"));
        pub static ref WORDLIST_ITALIAN: WordList = gen_wordlist(include_str!("langs/italian.txt"));
        pub static ref WORDLIST_JAPANESE: WordList = gen_wordlist(include_str!("langs/japanese.txt"));
        pub static ref WORDLIST_KOREAN: WordList = gen_wordlist(include_str!("langs/korean.txt"));
        pub static ref WORDLIST_SPANISH: WordList = gen_wordlist(include_str!("langs/spanish.txt"));

        pub static ref WORDMAP_ENGLISH: WordMap = gen_wordmap(&WORDLIST_ENGLISH);
        pub static ref WORDMAP_CHINESE_SIMPLIFIED: WordMap = gen_wordmap(&WORDLIST_CHINESE_SIMPLIFIED);
        pub static ref WORDMAP_CHINESE_TRADITIONAL: WordMap = gen_wordmap(&WORDLIST_CHINESE_TRADITIONAL);
        pub static ref WORDMAP_FRENCH: WordMap = gen_wordmap(&WORDLIST_FRENCH);
        pub static ref WORDMAP_ITALIAN: WordMap = gen_wordmap(&WORDLIST_ITALIAN);
        pub static ref WORDMAP_JAPANESE: WordMap = gen_wordmap(&WORDLIST_JAPANESE);
        pub static ref WORDMAP_KOREAN: WordMap = gen_wordmap(&WORDLIST_KOREAN);
        pub static ref WORDMAP_SPANISH: WordMap = gen_wordmap(&WORDLIST_SPANISH);
    }
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
            Language::Spanish => &lazy::WORDLIST_KOREAN,
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
            Language::Spanish => &lazy::WORDMAP_KOREAN,
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::English
    }
}
