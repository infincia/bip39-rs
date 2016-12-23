# bip39-rs

A Rust implementation of [BIP0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)

[Documentation](https://docs.rs/ring)

Add `bip39` to your Cargo file, and then refer to the documentation 
for use.

Only an English wordlist is included at the moment, but support for 
other languages is already present in the code.

A set of simple tests have been written but they only generate new 
mnemonics and validate a selection of known valid mnemonics generated
by other tools
