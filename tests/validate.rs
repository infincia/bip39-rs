extern crate bip39;

use ::bip39::{Mnemonic, Language};

#[test]
fn validate_12_english() {
    let phrase = "park remain person kitchen mule spell knee armed position rail grid ankle";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}

#[test]
fn validate_15_english() {
    let phrase = "any paddle cabbage armor atom satoshi fiction night wisdom nasty they midnight chicken play phone";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}

#[test]
fn validate_18_english() {
    let phrase = "soda oak spy claim best oppose gun ghost school use sign shock sign pipe vote follow category filter";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}


#[test]
fn validate_21_english() {
    let phrase = "quality useless orient offer pole host amazing title only clog sight wild anxiety gloom market rescue fan language entry fan oyster";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}


#[test]
fn validate_24_english() {
    let phrase = "always guess retreat devote warm poem giraffe thought prize ready maple daughter girl feel clay silent lemon bracket abstract basket toe tiny sword world";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}


#[test]
fn validate_12_english_uppercase() {
    let invalid_phrase = "Park remain person kitchen mule spell knee armed position rail grid ankle";

    assert!(Mnemonic::from_phrase(invalid_phrase, Language::English).is_err());
}
