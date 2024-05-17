//! Error types for ELF parsing and decoding

use std::{
    fmt::Display,
    io::{Read, Seek},
};

use typed_builder::TypedBuilder;

use crate::{
    base::ElfByte,
    header::elf::{
        identification::{
            ElfClass, ElfDataEncoding, ElfOSABI, ELF_CLASS_DEFAULT, ELF_DATA_ENCODING_DEFAULT,
        },
        ElfMachine,
    },
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
    #[error("Invalid ELF Header Flag value {value} for {machine:?}")]
    /// The value was invalid for the machine
    InvalidHeaderFlagForMachine {
        /// The machine the flag is invalid for
        machine: Option<ElfMachine<ELF_CLASS_DEFAULT, ELF_DATA_ENCODING_DEFAULT>>,
        /// The value that was invalid
        value: u32,
    },
    #[error(
        "Invalid ELF Machine {machine:?} for expected machine(s) {expected_machines:?} ELF Section Header Type {value}"
    )]
    /// The machine was invalid for a processor-specific section header type
    InvalidMachineForSectionHeaderType {
        /// The machine that was invalid
        machine: Option<ElfMachine<ELF_CLASS_DEFAULT, ELF_DATA_ENCODING_DEFAULT>>,
        /// The expected machine
        expected_machines: Vec<ElfMachine<ELF_CLASS_DEFAULT, ELF_DATA_ENCODING_DEFAULT>>,
        /// The value that was invalid
        value: u32,
    },
    #[error(
        "Invalid ELF OS/ABI {os_abi:?} for expected OS/ABI(s) {expected_os_abis:?} ELF Section Header Type {value}"
    )]
    /// The OS/ABI was invalid for an OS-specific section header type
    InvalidOsAbiForSectionHeaderType {
        /// The OS/ABI that was invalid
        os_abi: Option<ElfOSABI>,
        /// The expected OS/ABI
        expected_os_abis: Vec<ElfOSABI>,
        /// The value that was invalid
        value: u32,
    },
    #[error("Invalid ELF Section Header Type {value} for {machine:?}")]
    /// The SHT_ value was invalid for the AARCH64 architecture
    InvalidSectionHeaderType {
        /// The machine the section header type is invalid for
        machine: Option<ElfMachine<ELF_CLASS_DEFAULT, ELF_DATA_ENCODING_DEFAULT>>,
        /// The value that was invalid
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
