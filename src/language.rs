use ::error::Bip39Error;


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
    pub fn for_locale(locale: &str) -> Result<Language, Bip39Error> {
        let lang = match locale {
            "en_US.UTF-8" => Language::English,
            "en_GB.UTF-8" => Language::English,

            _ => { return Err(Bip39Error::LanguageUnavailable) }
        };

        Ok(lang)
    }
}