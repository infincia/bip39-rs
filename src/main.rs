extern crate bip39;

extern crate rustc_serialize;

use self::rustc_serialize::hex::{ToHex};

use bip39::{Bip39, KeyType, Language};

fn main() {
    let kt = KeyType::for_word_length(12).unwrap();

    let bip39 = match Bip39::new(&kt, Language::English, "") {
        Ok(b) => b,
        Err(e) => { println!("e: {:?}", e); return }
    };

    let phrase = &bip39.mnemonic;
    let hex = &bip39.to_hex();
    println!("phrase: {}", phrase);
    println!("seed: {}", hex);
}
