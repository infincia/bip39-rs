extern crate bip39;

use ::bip39::{Mnemonic, KeyType, Language};


#[test]
fn generate_12_english() {
    let kt = KeyType::for_word_length(12).unwrap();

    let b = match Mnemonic::new(&kt, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };
    let phrase = b.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 12);
    assert!(words.len() == kt.word_length());
    assert!(b.get_seed().len() == 64);
}

#[test]
fn generate_15_english() {
    let kt = KeyType::for_word_length(15).unwrap();

    let b = match Mnemonic::new(&kt, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };
    let phrase = b.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 15);
    assert!(words.len() == kt.word_length());
    assert!(b.get_seed().len() == 64);
}

#[test]
fn generate_18_english() {
    let kt = KeyType::for_word_length(18).unwrap();

    let b = match Mnemonic::new(&kt, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };
    let phrase = b.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 18);
    assert!(words.len() == kt.word_length());
    assert!(b.get_seed().len() == 64);

}

#[test]
fn generate_21_english() {
    let kt = KeyType::for_word_length(21).unwrap();

    let b = match Mnemonic::new(&kt, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };

    let phrase = b.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 21);
    assert!(words.len() == kt.word_length());
    assert!(b.get_seed().len() == 64);
}

#[test]
fn generate_24_english() {
    let kt = KeyType::for_word_length(24).unwrap();

    let b = match Mnemonic::new(&kt, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };

    let phrase = b.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 24);
    assert!(words.len() == kt.word_length());
    assert!(b.get_seed().len() == 64);
}
