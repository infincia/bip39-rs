use ::error::{Error, ErrorKind};
use std::collections::HashMap;

mod lazy {
	use std::collections::HashMap;
	
	/// lazy generation of the word list
	fn gen_wordlist(lang_words: &str) -> Vec<String> {

		lang_words.split_whitespace()
			.map(|s| s.into())
			.collect()
	}

	/// lazy generation of the word map
	fn gen_wordmap(word_list: &Vec<String>) -> HashMap<String, u16> {

		let mut word_map: HashMap<String, u16> = HashMap::new();
		for (i, item) in word_list.into_iter().enumerate() {
			word_map.insert(item.to_owned(), i as u16);
		}

		word_map
	}

	static BIP39_WORDLIST_ENGLISH: &'static str = include_str!("bip39_english.txt"); 

	lazy_static! {
		pub static ref VEC_BIP39_WORDLIST_ENGLISH: Vec<String> = { gen_wordlist(BIP39_WORDLIST_ENGLISH) };
	}

	lazy_static! {
		pub static ref HASHMAP_BIP39_WORDMAP_ENGLISH: HashMap<String, u16> = { gen_wordmap(&VEC_BIP39_WORDLIST_ENGLISH) };
	}
}

#[derive(Debug)]
pub enum Language {
    English
}

impl Language {
    /// Get the `Language` value for a specific locale
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
    pub fn for_locale<S>(locale: S) -> Result<Language, Error> where S: Into<String>  {

        let l = locale.into();

        let lang = match &*l {
            "en_US.UTF-8" => Language::English,
            "en_GB.UTF-8" => Language::English,

            _ => { return Err(ErrorKind::LanguageUnavailable.into()) }
        };

        Ok(lang)
    }
    
    pub fn get_wordlist(&self) -> &'static Vec<String> {

		match *self {
            Language::English => &lazy::VEC_BIP39_WORDLIST_ENGLISH
        }
	}
	
	pub fn get_wordmap(&self) -> &'static HashMap<String, u16> {

		match *self {
            Language::English => &lazy::HASHMAP_BIP39_WORDMAP_ENGLISH
        }
	}
}
