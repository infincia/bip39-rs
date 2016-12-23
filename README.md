# bip39-rs

A Rust implementation of [BIP0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)

[Documentation](https://docs.rs/bip39)

Add `bip39` to your Cargo file, and then refer to the documentation 
for use.

Only an English wordlist is included at the moment, but support for 
other languages is already present in the code.

A set of simple tests have been written but they only generate new 
mnemonics and validate a selection of known valid mnemonics generated
by other tools

There is a tiny binary called `bip39` that will be built by cargo,
it just generates a new nmemonic and prints the phrase and seed hex value
at the moment, but could easily be expanded to validate phrases too.