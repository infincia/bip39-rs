#![feature(test)]

extern crate test;
extern crate bip39;

use test::Bencher;

use bip39::{Mnemonic, Language};

#[bench]
fn parse_to_ast(b: &mut Bencher) {
    let phrase = "silly laptop awake length nature thunder category claim reveal supply attitude drip";

    b.iter(|| {
        let _ = Mnemonic::validate(phrase, Language::English);
    });
}
