#[derive(thiserror::Error, Debug, PartialEq, Eq, Hash)]
/// Error type for errors during ELF object handling
pub enum Error {
    #[error("An I/O error occurred: {kind}")]
    Io { kind: std::io::ErrorKind },
    #[error("Invalid value {value} for ELF Class")]
    InvalidElfClass { value: u8 },
    #[error("Invalid value {value} for ELF Data Encoding")]
    InvalidElfDataEncoding { value: u8 },
    #[error(
        "Invalid value {elf_class} for ELF Class or {elf_data_encoding} for ELF Data Encoding"
    )]
    InvalidElfClassOrDataEncoding {
        elf_class: u8,
        elf_data_encoding: u8,
    },
    #[error("Invalid value {value} for ELF Identifier Version")]
    InvalidElfIdentifierVersion { value: u8 },
    #[error("Invalid value {value} for ELF OS ABI")]
    InvalidElfOsAbi { value: u8 },
}

pub type Result<T> = std::result::Result<T, Error>;
