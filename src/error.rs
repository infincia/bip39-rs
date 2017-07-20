
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
        LanguageUnavailable {
            description("wrapping key failed")
            display("Language unavailable")
        }
    }
}
