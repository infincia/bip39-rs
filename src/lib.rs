//!
//! This is a Rust implementation of the [bip39][bip39-standard] standard for Bitcoin HD wallet
//! mnemonic phrases.
//!
//!
//! [bip39-standard]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
//!
#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
extern crate data_encoding;
extern crate bitreader;
extern crate bit_vec;
extern crate ring;

mod mnemonic;
mod error;
mod mnemonic_type;
mod language;
mod util;
mod seed;

mod crypto;

pub use language::Language;
pub use mnemonic::Mnemonic;
pub use mnemonic_type::MnemonicType;
pub use seed::Seed;
pub use error::Error;
