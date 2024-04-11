//! Error types for ELF parsing and decoding

use std::{
    fmt::Display,
    io::{Read, Seek},
};

use typed_builder::TypedBuilder;

use crate::{
    base::ElfByte,
    header::elf::identification::{ElfClass, ElfDataEncoding},
};

#[derive(thiserror::Error, Debug, PartialEq, Eq, Hash)]
/// Error type for errors during ELF object handling
pub enum Error {
    #[error("I/O Error {kind}")]
    /// A wrapped I/O error that is hashable and comparable
    Io {
        /// The kind of I/O error
        kind: std::io::ErrorKind,
    },
    #[error("Invalid ELF class {class}")]
    /// Invalid ELF class value
    InvalidClass {
        /// The value that could not be interpreted as a class value
        class: ElfByte,
    },
    #[error("Invalid constant ELF class {class}")]
    /// Invalid ELF class value
    InvalidConstantClass {
        /// The value that could not be interpreted as a class value
        class: u8,
    },
    #[error("Invalid ELF data encoding {encoding}")]
    /// Invalid ELF data encoding value
    InvalidDataEncoding {
        /// The value that could not be interpreted as a data encoding value
        encoding: ElfByte,
    },
    #[error("Invalid constant ELF data encoding {encoding}")]
    /// Invalid ELF data encoding value
    InvalidConstantDataEncoding {
        /// The value that could not be interpreted as a data encoding value
        encoding: u8,
    },
    #[error("Invalid ELF class ({class:?}) and data encoding ({encoding:?})")]
    /// Invalid ELF data encoding value
    InvalidClassEncodingPair {
        /// The value that could not be interpreted as an ELF class value
        class: ElfClass,
        /// The value that could not be interpreted as a data encoding value
        encoding: ElfDataEncoding,
    },
    #[error("Invalid ELF identifier version {version}")]
    /// Invalid ELF version value
    InvalidIdentifierVersion {
        /// The value that could not be interpreted as a version value
        version: ElfByte,
    },
    #[error("Invalid ELF OS ABI {os_abi}")]
    /// Invalid ELF OS ABI value
    InvalidOsAbi {
        /// The value that could not be interpreted as an OS ABI value
        os_abi: ElfByte,
    },
    #[error("Invalid ELF ABI version {version}")]
    /// Invalid ELF ABI version value
    InvalidAbiVersion {
        /// The value that could not be interpreted as an ABI version value
        version: ElfByte,
    },
    #[error("Invalid ELF type {context}")]
    /// Invalid ELF type value
    InvalidType {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF machine {context}")]
    /// Invalid ELF machine value
    InvalidMachine {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF version {context}")]
    /// Invalid ELF version value
    InvalidVersion {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF entry point {context}")]
    /// Invalid ELF entry point value
    InvalidEntryPoint {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF program header offset {context}")]
    /// Invalid ELF program header offset value
    InvalidProgramHeaderOffset {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF section header offset {context}")]
    /// Invalid ELF section header offset value
    InvalidSectionHeaderOffset {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF flags {context}")]
    /// Invalid ELF flags value
    InvalidFlags {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF header size {context}")]
    /// Invalid ELF header size value
    InvalidHeaderSize {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF program header table entry size {context}")]
    /// Invalid ELF program header table entry size value
    InvalidProgramHeaderTableEntrySize {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF program header table entry count {context}")]
    /// Invalid ELF program header table entry count value
    InvalidProgramHeaderTableEntryCount {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF section header table entry size {context}")]
    /// Invalid ELF section header table entry size value
    InvalidSectionHeaderTableEntrySize {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF section header table entry count {context}")]
    /// Invalid ELF section header table entry count value
    InvalidSectionHeaderTableEntryCount {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF section header string table index {context}")]
    /// Invalid ELF section header string table index value
    InvalidSectionHeaderStringTableIndex {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF section header type {context}")]
    /// Invalid ELF section header type
    InvalidElfSectionHeaderType {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF section header flags {context}")]
    /// Invalid ELF section header flags
    InvalidElfSectionHeaderFlags {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF compression header type {context}")]
    /// Invalid ELF compression header type value
    InvalidCompressionHeaderType {
        /// The decoding context
        context: ErrorContext,
    },
    #[error("Invalid ELF Flag value for MIPS machine {value}")]
    /// The EF_MIPS_MACH field of the header flags field was invalid
    InvalidHeaderFlagMipsMachine {
        /// The invalid value
        value: u32,
    },
    #[error("Invalid ELF flag value for MIPS architecture {value}")]
    /// The EF_MIPS_ARCH field of the header flags field was invalid
    InvalidHeaderFlagMipsArchitecture {
        /// The invalid value
        value: u32,
    },
    #[error("Invalid ELF flag value for MIPS architecture {value}")]
    /// The EF_MIPS_ARCH field of the header flags field was invalid
    InvalidHeaderFlagMipsArchitectureExtension {
        /// The invalid value
        value: u32,
    },
    #[error("Invalid ELF flag value for MIPS architecture {value}")]
    /// The EF_MIPS_ARCH field of the header flags field was invalid
    InvalidHeaderFlagMipsAbi {
        /// The invalid value
        value: u32,
    },
    #[error("Invalid ELF flag value for PARISC architecture {value}")]
    /// The EF_PARISC_ARCH field of the header flags field was invalid
    InvalidHeaderFlagPariscArchitectureExtensions {
        /// The invalid value
        value: u32,
    },
}

#[derive(Debug, Clone, Eq, PartialOrd, Ord, TypedBuilder)]
/// A context for an error
pub struct ErrorContext {
    /// The offset in the file where the error occurred
    pub offset: u64,
    #[builder(default, setter(into))]
    /// The context around the error
    pub context: Vec<u8>,
}

impl ErrorContext {
    /// Read the error context from a reader at a certain offset and size
    pub fn from_reader_at<R>(reader: &mut R, offset: u64, size: usize) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let mut context = vec![0; size];
        reader
            .seek(std::io::SeekFrom::Start(offset))
            .map_err(|e| Error::Io { kind: e.kind() })?;
        // Try to read exactly the size and if we fail read one less until we read nothing
        while reader.read_exact(&mut context).is_err() {
            context.pop();
        }
        Ok(ErrorContext { offset, context })
    }

    /// Read the error context from a size of a read that just errored. Unlike `from_reader_at` this
    /// rewinds the reader to the start of the read instead of seeking to the offset directly.
    pub fn from_reader<R>(reader: &mut R, size: usize) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let offset = reader
            .stream_position()
            .map_err(|e| Error::Io { kind: e.kind() })?;
        let begin = offset.saturating_sub(size as u64);
        Self::from_reader_at(reader, begin, size)
    }
}

impl Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset: {:#x}, context: {:?}", self.offset, self.context)
    }
}

impl PartialEq for ErrorContext {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl std::hash::Hash for ErrorContext {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
    }
}
