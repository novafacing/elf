use std::{
    fmt::Display,
    io::{Read, Seek},
    ops::BitOr,
};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone, Debug)]
/// The context in which an ELF file is being handled
pub struct Context {
    #[builder(default)]
    /// A set of errors to ignore if they occur. This is useful for handling errors that
    /// are known to occur in certain files but are not critical to a point where the
    /// ELF cannot be used.
    pub ignore: Vec<Error>,
    #[builder(default)]
    /// Whether all errors should be ignored
    pub ignore_all: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Context {
    #[allow(unused)] // NOTE: False positive
    /// Check if an error should be ignored
    pub fn should_ignore(&self, error: &Error) -> bool {
        self.ignore_all || self.ignore.iter().map(|e| matches!(error, e)).any(|b| b)
    }
}

#[derive(thiserror::Error, Debug)]
/// An error in ELF handling
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Invalid magic {context} for ELF file. Expected {expected}.")]
    /// An invalid magic number was found in the ELF file
    InvalidMagic {
        /// The context in which the error occurred
        context: ErrorContext,
        /// The expected context
        expected: ErrorContext,
    },
    #[error("Invalid class {context} for ELF file.")]
    /// An invalid class was found in the ELF file
    InvalidClass {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid data encoding {context} for ELF file.")]
    /// An invalid data encoding was found in the ELF file
    InvalidDataEncoding {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid version {context} for ELF file.")]
    /// An invalid file version was found in the ELF file
    InvalidVersion {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid OS/ABI {context} for ELF file.")]
    /// An invalid OS/ABI was found in the ELF file
    InvalidOsAbi {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid ABI version {context} for ELF file.")]
    /// An invalid ABI version was found in the ELF file
    InvalidAbiVersion {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid types {lhs} and {rhs} for BitOr operation.")]
    /// An invalid type was found in a BitOr operation
    InvalidBitOr {
        /// The left-hand side of the BitOr operation
        lhs: String,
        /// The right-hand side of the BitOr operation
        rhs: String,
    },
}

#[derive(TypedBuilder, Debug, Clone)]
/// The context of an error in ELF handling
pub struct ErrorContext {
    /// The offset at which the error occurred
    pub offset: u64,
    /// Relevant context for the error
    pub context: Vec<u8>,
}

impl Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let context = self
            .context
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "0x{:x}: {}", self.offset, context)
    }
}

impl ErrorContext {
    pub fn from_reader<R>(reader: &mut R, offset: u64, size: usize) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let mut context = vec![0; size];
        reader.seek(std::io::SeekFrom::Start(offset))?;
        reader.read_exact(&mut context)?;
        Ok(Self::builder().offset(offset).context(context).build())
    }
}

type Elf32HalfWord = u16;
type Elf64HalfWord = u16;
type ELf32Word = u32;
type Elf32SignedWord = i32;
type Elf64Word = u32;
type Elf64SignedWord = i32;
type Elf32ExtendedWord = u64;
type Elf32SignedExtendedWord = i64;
type Elf64ExtendedWord = u64;
type Elf64SignedExtendedWord = i64;
type Elf32Address = u32;
type Elf64Address = u64;
type Elf32Offset = u32;
type Elf64Offset = u64;
type Elf32Section = u16;
type Elf64Section = u16;
type Elf32VersionSymbol = Elf32HalfWord;
type Elf64VersionSymbol = Elf64HalfWord;

/// Decode a value from a reader
pub trait FromReader<R>
where
    R: Read + Seek,
    Self: Sized,
{
    /// An error that can occur when decoding a value from a reader
    type Error;

    /// Decodes a value from a reader with a context and returns an owned instance of it
    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error>;

    /// Decodes a value from a reader and returns an owned instance of it, creating a new context
    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let mut context = Context::builder().build();
        Self::from_reader_with(reader, &mut context)
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The magic number that identifies an ELF file
pub struct ElfHeaderIdentificationMagic {
    pub magic: [u8; 4],
}

impl Default for ElfHeaderIdentificationMagic {
    fn default() -> Self {
        Self { magic: Self::MAGIC }
    }
}

impl ElfHeaderIdentificationMagic {
    pub const MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
}

impl<R> FromReader<R> for ElfHeaderIdentificationMagic
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;
        let mut magic = [0; 4];
        reader.read_exact(&mut magic)?;

        if magic != Self::MAGIC {
            let error = Error::InvalidMagic {
                context: ErrorContext::from_reader(reader, offset, magic.len())?,
                expected: ErrorContext::builder()
                    .offset(0)
                    .context(Self::MAGIC.to_vec())
                    .build(),
            };

            context
                .should_ignore(&error)
                .then(|| Ok(Self::default()))
                .unwrap_or(Err(error))
        } else {
            Ok(Self { magic })
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// The file's class (noted as capacity in the ELF specification), i.e. its bit width,
/// which also determines the size of the file's data structures and types.
pub enum ElfHeaderIdentificationClass {
    /// An invalid class
    None = 0,
    /// 32-bit objects
    ElfClass32 = 1,
    /// 64-bit objects
    ElfClass64 = 2,
}

impl<R> FromReader<R> for ElfHeaderIdentificationClass
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;
        let mut class = [0];
        reader.read_exact(&mut class)?;

        match class[0] {
            1 => Ok(Self::ElfClass32),
            2 => Ok(Self::ElfClass64),
            _ => {
                let error = Error::InvalidClass {
                    context: ErrorContext::from_reader(reader, offset, class.len())?,
                };

                context
                    .should_ignore(&error)
                    .then(|| Ok(Self::None))
                    .unwrap_or(Err(error))
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// The data encoding of the file's data structures and types
/// (i.e. the byte order of the file's data)
pub enum ElfHeaderIdentificationDataEncoding {
    /// An invalid data encoding
    None = 0,
    /// Little-endian
    LittleEndian = 1,
    /// Big-endian
    BigEndian = 2,
}

impl<R> FromReader<R> for ElfHeaderIdentificationDataEncoding
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;
        let mut encoding = [0];
        reader.read_exact(&mut encoding)?;

        match encoding[0] {
            1 => Ok(Self::LittleEndian),
            2 => Ok(Self::BigEndian),
            _ => {
                let error = Error::InvalidDataEncoding {
                    context: ErrorContext::from_reader(reader, offset, encoding.len())?,
                };

                context
                    .should_ignore(&error)
                    .then(|| Ok(Self::None))
                    .unwrap_or(Err(error))
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// The file's version
pub enum ElfHeaderIdentificationVersion {
    /// An invalid version
    None = 0,
    /// The current version of ELF
    Current = 1,
}

impl<R> FromReader<R> for ElfHeaderIdentificationVersion
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;
        let mut version = [0];
        reader.read_exact(&mut version)?;

        match version[0] {
            1 => Ok(Self::Current),
            _ => {
                let error = Error::InvalidVersion {
                    context: ErrorContext::from_reader(reader, offset, version.len())?,
                };

                context
                    .should_ignore(&error)
                    .then(|| Ok(Self::None))
                    .unwrap_or(Err(error))
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum X86ElfHeaderIdentificationOSABI {}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// The file's OS/ABI
/// (i.e. the operating system and/or ABI for which the file is intended)
pub enum ElfHeaderIdentificationOSABI {
    None = 0,
    HPUX = 1,
    NetBSD = 2,
    GnuLinux = 3,
    Solaris = 6,
    AIX = 7,
    IRIX = 8,
    FreeBSD = 9,
    Tru64 = 10,
    NovellModesto = 11,
    OpenBSD = 12,
    OpenVMS = 13,
    NonStopKernel = 14,
    AROS = 15,
    FenixOS = 16,
    CloudABI = 17,
    OpenVOS = 18,
    X86(X86ElfHeaderIdentificationOSABI),
}

impl BitOr for ElfHeaderIdentificationOSABI {
    type Output = Result<Self, Error>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ElfHeaderIdentificationOSABI::X86(lhs), ElfHeaderIdentificationOSABI::X86(rhs)) => {
                Ok(Self::X86(lhs as u8 | rhs as u8))
            }
        }
    }
}

pub struct ElfHeaderIdentification {}
pub trait ElfHeaderType {}
pub trait ElfHeaderMachine {}
pub trait ElfHeaderVersion {}
pub trait ElfHeaderEntry {}
pub trait ElfHeaderProgramHeaderTableOffset {}
pub trait ElfHeaderSectionHeaderTableOffset {}

#[cfg(test)]
mod tests {
    mod test_elf_header_identification_magic {
        use super::super::*;

        #[test]
        fn test_elf_header_identification_magic() {
            let mut reader = std::io::Cursor::new(&[0x7f, b'E', b'L', b'F']);
            let magic = ElfHeaderIdentificationMagic::from_reader(&mut reader).unwrap();
            assert_eq!(magic.magic, [0x7f, b'E', b'L', b'F']);
        }

        #[test]
        fn test_elf_header_identification_magic_invalid() {
            let mut reader = std::io::Cursor::new(&[0x7f, b'E', b'L', b'G']);
            let error = ElfHeaderIdentificationMagic::from_reader(&mut reader).unwrap_err();
            assert_eq!(
                error.to_string(),
                "Invalid magic 0x0: 7f 45 4c 47 for ELF file. Expected 0x0: 7f 45 4c 46."
            );
        }
    }
}
