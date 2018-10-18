use mnemonic_type::MnemonicType;

error_chain! {
    foreign_links {
        EntropyUnavailable(::std::io::Error);
    }

    errors {
        InvalidChecksum {
            description("invalid checksum")
            display("Invalid checksum")
        }
        InvalidWord {
            description("invalid word in phrase")
            display("Invalid word in phrase")
        }
        InvalidKeysize {
            description("invalid keysize")
            display("Invalid keysize")
        }
        InvalidWordLength {
            description("invalid number of words in phrase")
            display("Invalid number of words in phrase")
        }
        InvalidEntropyLength(entropy_length_bits: usize, mnemonic_type: MnemonicType) {
            description("invalid entropy length for mnemonic type")
            display("Invalid entropy length {}bits for mnemonic type {}", entropy_length_bits, mnemonic_type)
        }
    }
}
