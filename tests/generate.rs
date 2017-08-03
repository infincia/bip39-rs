extern crate bip39;

use ::bip39::{Mnemonic, MnemonicType, Language};


#[test]
fn generate_12_english() {
    let mnemonic_type = MnemonicType::for_word_count(12).unwrap();

    let mnemonic= match Mnemonic::new(mnemonic_type, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };
    let phrase = mnemonic.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 12);
    assert!(words.len() == mnemonic_type.word_count());
    let seed = mnemonic.get_seed();
    let seed_bytes: &[u8] = seed.as_bytes();
    assert!(seed_bytes.len() == 64);
}

#[test]
fn generate_15_english() {
    let mnemonic_type = MnemonicType::for_word_count(15).unwrap();

    let mnemonic= match Mnemonic::new(mnemonic_type, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };
    let phrase = mnemonic.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 15);
    assert!(words.len() == mnemonic_type.word_count());
    let seed = mnemonic.get_seed();
    let seed_bytes: &[u8] = seed.as_bytes();
    assert!(seed_bytes.len() == 64);
}

#[test]
fn generate_18_english() {
    let mnemonic_type = MnemonicType::for_word_count(18).unwrap();

    let mnemonic= match Mnemonic::new(mnemonic_type, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };
    let phrase = mnemonic.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 18);
    assert!(words.len() == mnemonic_type.word_count());
    let seed = mnemonic.get_seed();
    let seed_bytes: &[u8] = seed.as_bytes();
    assert!(seed_bytes.len() == 64);
}

#[test]
fn generate_21_english() {
    let mnemonic_type = MnemonicType::for_word_count(21).unwrap();

    let mnemonic= match Mnemonic::new(mnemonic_type, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };

    let phrase = mnemonic.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 21);
    assert!(words.len() == mnemonic_type.word_count());
    let seed = mnemonic.get_seed();
    let seed_bytes: &[u8] = seed.as_bytes();
    assert!(seed_bytes.len() == 64);
}

#[test]
fn generate_24_english() {
    let mnemonic_type = MnemonicType::for_word_count(24).unwrap();

    let mnemonic= match Mnemonic::new(mnemonic_type, Language::English, "") {
        Ok(b) => b,
        Err(_) => { assert!(false); return }
    };

    let phrase = mnemonic.get_string();
    let words: Vec<&str> = phrase.split(" ").into_iter().collect();

    assert!(words.len() == 24);
    assert!(words.len() == mnemonic_type.word_count());
    let seed = mnemonic.get_seed();
    let seed_bytes: &[u8] = seed.as_bytes();
    assert!(seed_bytes.len() == 64);
}
