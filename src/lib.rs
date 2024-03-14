//! Definitions for ELF files. Resources include:
//!
//! GLIBC ELF Definitions:
//! - [elf.h](https://github.com/bminor/glibc/blob/master/elf/elf.h)
//!
//! The latest System V ABI Drafts & Specifications:
//! - [System V Application Binary Interface](https://www.sco.com/developers/gabi/latest/contents.html)
//! - [x86-64 ABI](https://gitlab.com/x86-psABIs/x86-64-ABI)
//! - [x86 ABI](https://gitlab.com/x86-psABIs/i386-ABI)
//! - [ARM & AARCH64](https://github.com/ARM-software/abi-aa/releases)
//! - [RISCV](https://github.com/riscv-non-isa/riscv-elf-psabi-doc/releases)
//!
use bitflags::bitflags;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive as _, Saturating as _, SaturatingSub as _, ToPrimitive as _};
use std::{
    fmt::Display,
    io::{Read, Seek},
    mem::size_of,
};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug)]
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
    #[builder(default = ElfHeaderIdentificationClass::ElfClass64)]
    /// The class of the ELF file, which specifies the bit width of structures
    /// and types in the file
    pub class: ElfHeaderIdentificationClass,
    #[builder(default = ElfHeaderIdentificationDataEncoding::LittleEndian)]
    /// The data encoding of the ELF file, which specifies the byte order of the file's data
    /// structures and types
    pub data_encoding: ElfHeaderIdentificationDataEncoding,
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

    pub fn read_uchar<R>(&mut self, reader: &mut R) -> Result<u8, Error>
    where
        R: Read + Seek,
    {
        u8::from_reader_with(reader, self)
    }

    pub fn read_half_word<R>(&mut self, reader: &mut R) -> Result<u16, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => {
                Elf32HalfWord::from_reader_with(reader, self)
            }
            ElfHeaderIdentificationClass::ElfClass64 => {
                Elf64HalfWord::from_reader_with(reader, self)
            }
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 2)?,
            }),
        }
    }

    pub fn read_word<R>(&mut self, reader: &mut R) -> Result<u32, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => ELf32Word::from_reader_with(reader, self),
            ElfHeaderIdentificationClass::ElfClass64 => Elf64Word::from_reader_with(reader, self),
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 4)?,
            }),
        }
    }

    pub fn read_signed_word<R>(&mut self, reader: &mut R) -> Result<i32, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => {
                Elf32SignedWord::from_reader_with(reader, self)
            }
            ElfHeaderIdentificationClass::ElfClass64 => {
                Elf64SignedWord::from_reader_with(reader, self)
            }
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 4)?,
            }),
        }
    }

    pub fn read_extended_word<R>(&mut self, reader: &mut R) -> Result<u64, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => {
                Elf32ExtendedWord::from_reader_with(reader, self)
            }
            ElfHeaderIdentificationClass::ElfClass64 => {
                Elf64ExtendedWord::from_reader_with(reader, self)
            }
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 8)?,
            }),
        }
    }

    pub fn read_signed_extended_word<R>(&mut self, reader: &mut R) -> Result<i64, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => {
                Elf32SignedExtendedWord::from_reader_with(reader, self)
            }
            ElfHeaderIdentificationClass::ElfClass64 => {
                Elf64SignedExtendedWord::from_reader_with(reader, self)
            }
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 8)?,
            }),
        }
    }

    pub fn read_address<R>(&mut self, reader: &mut R) -> Result<u64, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => {
                Elf32Address::from_reader_with(reader, self).map(|v| v as u64)
            }
            ElfHeaderIdentificationClass::ElfClass64 => {
                Elf64Address::from_reader_with(reader, self)
            }
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 8)?,
            }),
        }
    }

    pub fn read_offset<R>(&mut self, reader: &mut R) -> Result<u64, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => {
                Elf32Offset::from_reader_with(reader, self).map(|v| v as u64)
            }
            ElfHeaderIdentificationClass::ElfClass64 => Elf64Offset::from_reader_with(reader, self),
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 8)?,
            }),
        }
    }

    pub fn read_section<R>(&mut self, reader: &mut R) -> Result<u16, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => {
                Elf32Section::from_reader_with(reader, self)
            }
            ElfHeaderIdentificationClass::ElfClass64 => {
                Elf64Section::from_reader_with(reader, self)
            }
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 2)?,
            }),
        }
    }

    pub fn read_version_symbol<R>(&mut self, reader: &mut R) -> Result<u16, Error>
    where
        R: Read + Seek,
    {
        let offset = reader.stream_position()?;
        match self.class {
            ElfHeaderIdentificationClass::ElfClass32 => {
                Elf32VersionSymbol::from_reader_with(reader, self)
            }
            ElfHeaderIdentificationClass::ElfClass64 => {
                Elf64VersionSymbol::from_reader_with(reader, self)
            }
            _ => Err(Error::InvalidClass {
                context: ErrorContext::from_reader(reader, offset, 2)?,
            }),
        }
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
    #[error("Invalid type {context} for ELF file.")]
    /// An invalid ELF type was found in the ELF file
    InvalidType {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid machine {context} for ELF file.")]
    /// An invalid machine was found in the ELF file
    InvalidMachine {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header name {context} for ELF file.")]
    /// An invalid section header name was found in the ELF file
    InvalidSectionHeaderName {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header type {context} for ELF file.")]
    /// An invalid section header type was found in the ELF file
    InvalidSectionHeaderType {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header flags {context} for ELF file.")]
    /// An invalid section header flags were found in the ELF file
    InvalidSectionHeaderFlags {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header address {context} for ELF file.")]
    /// An invalid section header address was found in the ELF file
    InvalidSectionHeaderAddress {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header offset {context} for ELF file.")]
    /// An invalid section header offset was found in the ELF file
    InvalidSectionHeaderOffset {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header size {context} for ELF file.")]
    /// An invalid section header size was found in the ELF file
    InvalidSectionHeaderSize {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header link {context} for ELF file.")]
    /// An invalid section header link was found in the ELF file
    InvalidSectionHeaderLink {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header info {context} for ELF file.")]
    /// An invalid section header info was found in the ELF file
    InvalidSectionHeaderInfo {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header alignment {context} for ELF file.")]
    /// An invalid section header alignment was found in the ELF file
    InvalidSectionHeaderAlignment {
        /// The context in which the error occurred
        context: ErrorContext,
    },
    #[error("Invalid section header entry size {context} for ELF file.")]
    /// An invalid section header entry size was found in the ELF file
    InvalidSectionHeaderEntrySize {
        /// The context in which the error occurred
        context: ErrorContext,
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

// Implement FromReader for basic types
macro_rules! impl_from_reader {
    ($($t:ty),*) => {
        $(
            impl<R> FromReader<R> for $t
            where
                R: Read + Seek,
            {
                type Error = Error;

                fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
                    let mut value = [0; std::mem::size_of::<Self>()];
                    reader.read_exact(&mut value)?;
                    if context.data_encoding == ElfHeaderIdentificationDataEncoding::BigEndian {
                        Ok(Self::from_be_bytes(value))
                    } else {
                        Ok(Self::from_le_bytes(value))
                    }
                }
            }
        )*
    };
}

impl_from_reader!(u8, u16, u32, u64, i8, i16, i32, i64);

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
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

        context
            .read_uchar(reader)
            .and_then(|class| match Self::from_u8(class) {
                Some(c) => {
                    context.class = c.clone();
                    Ok(c)
                }
                None => {
                    let error = Error::InvalidClass {
                        context: ErrorContext::from_reader(reader, offset, size_of::<Self>())?,
                    };

                    context.class = Self::None;

                    context
                        .should_ignore(&error)
                        .then(|| Ok(Self::None))
                        .unwrap_or(Err(error))
                }
            })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
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

        context
            .read_uchar(reader)
            .and_then(|encoding| match Self::from_u8(encoding) {
                Some(e) => {
                    context.data_encoding = e.clone();
                    Ok(e)
                }
                None => {
                    let error = Error::InvalidDataEncoding {
                        context: ErrorContext::from_reader(reader, offset, size_of::<Self>())?,
                    };

                    context.data_encoding = Self::None;

                    context
                        .should_ignore(&error)
                        .then(|| Ok(Self::None))
                        .unwrap_or(Err(error))
                }
            })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
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
        context
            .read_uchar(reader)
            .and_then(|version| match Self::from_u8(version) {
                Some(v) => Ok(v),
                None => {
                    let error = Error::InvalidVersion {
                        context: ErrorContext::from_reader(reader, offset, size_of::<Self>())?,
                    };

                    context
                        .should_ignore(&error)
                        .then(|| Ok(Self::None))
                        .unwrap_or(Err(error))
                }
            })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The file's OS/ABI
/// (i.e. the operating system and/or ABI for which the file is intended)
///
/// This value
pub enum ElfHeaderIdentificationOSABI {
    /// Unix System V ABI or None, parsing None for this identification field is *not* an
    /// error.
    NoneSystemV = 0,
    /// HP-UX
    HPUX = 1,
    /// NetBSD
    NetBSD = 2,
    /// Object uses GNU ELF extensions.
    GnuLinux = 3,
    /// SUN Solaris
    Solaris = 6,
    /// IBM AIX
    AIX = 7,
    /// SGI Irix
    IRIX = 8,
    /// FreeBSD
    FreeBSD = 9,
    /// Compaq TRU64 UNIX
    Tru64 = 10,
    /// Novell Modesto
    NovellModesto = 11,
    /// OpenBSD
    OpenBSD = 12,
    /// Open Virtual Memory System
    OpenVMS = 13,
    /// NSK Non-Stop Kernel
    NonStopKernel = 14,
    /// Amiga Research OS
    AROS = 15,
    /// FenixOS Highly scalable multi-core OS
    FenixOS = 16,
    /// Nuxi CloudABI
    CloudABI = 17,
    /// Stratus Technologies OpenVOS
    OpenVOS = 18,
    /// ARM EABI (the object file contains symbol versioning extensions as described
    /// in the aaelf32 documentation)
    ArmExtendedApplicationBinaryInterface = 64,
    /// FDPIC ELF for either XTensa or ARM, depending on the detected machine. For ARM, this
    /// is described in the fdpic document.
    ArmXTensaFunctionDescriptorPositionIndependentCode = 65,
    /// ARM (non-EABI)
    Arm = 97,
    /// Standalone system
    Standalone = 255,
}

impl<R> FromReader<R> for ElfHeaderIdentificationOSABI
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;

        context
            .read_uchar(reader)
            .and_then(|os_abi| match Self::from_u8(os_abi) {
                Some(o) => Ok(o),
                None => {
                    let error = Error::InvalidOsAbi {
                        context: ErrorContext::from_reader(reader, offset, size_of::<Self>())?,
                    };

                    context
                        .should_ignore(&error)
                        .then(|| Ok(Self::NoneSystemV))
                        .unwrap_or(Err(error))
                }
            })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElfHeaderIdentification {
    pub magic: ElfHeaderIdentificationMagic,
    pub class: ElfHeaderIdentificationClass,
    pub data_encoding: ElfHeaderIdentificationDataEncoding,
    pub version: ElfHeaderIdentificationVersion,
    pub os_abi: ElfHeaderIdentificationOSABI,
    pub abi_version: u8,
    pub padding: [u8; 7],
}

impl<R> FromReader<R> for ElfHeaderIdentification
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let mut slf = Self {
            magic: ElfHeaderIdentificationMagic::from_reader_with(reader, context)?,
            class: ElfHeaderIdentificationClass::from_reader_with(reader, context)?,
            data_encoding: ElfHeaderIdentificationDataEncoding::from_reader_with(reader, context)?,
            version: ElfHeaderIdentificationVersion::from_reader_with(reader, context)?,
            os_abi: ElfHeaderIdentificationOSABI::from_reader_with(reader, context)?,
            abi_version: u8::from_reader_with(reader, context)?,
            padding: [0; 7],
        };

        slf.padding.iter_mut().try_for_each(|p| {
            *p = context.read_uchar(reader)?;
            Ok::<(), Error>(())
        })?;

        Ok(slf)
    }
}

#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
pub enum ElfHeaderType {
    /// No file type
    None = 0,
    /// Relocatable file type
    Relocatable = 1,
    /// Executable file type
    Executable = 2,
    /// Shared object file type
    Dynamic = 3,
    /// Core file
    Core = 4,
    /// Number of defined types
    NumberDefined = 5,
    /// OS-specific range of types begin
    LowOperatingSystem = 0xfe00,
    /// OS specific range of types end
    HighOperatingSystem = 0xfeff,
    /// Processor-specific range of types begin
    LowProcessorSpecific = 0xff00,
    /// Processor specific range of types end
    HighProcessorSpecific = 0xffff,
}

impl<R> FromReader<R> for ElfHeaderType
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;

        context
            .read_half_word(reader)
            .and_then(|ty| match Self::from_u16(ty) {
                Some(t) => Ok(t),
                None => {
                    let error = Error::InvalidType {
                        context: ErrorContext::from_reader(reader, offset, size_of::<Self>())?,
                    };

                    context
                        .should_ignore(&error)
                        .then(|| Ok(Self::None))
                        .unwrap_or(Err(error))
                }
            })
    }
}

#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// Elf Machine
pub enum ElfHeaderMachine {
    /// No machine
    NONE = 0,
    /// AT&T WE 32100
    M32 = 1,
    /// SPARC
    SPARC = 2,
    /// Intel 80386
    I386 = 3,
    /// Motorola 68000
    M68K = 4,
    /// Motorola 88000
    M88K = 5,
    /// Intel MCU
    IAMCU = 6,
    /// Intel 80860
    I860 = 7,
    /// MIPS I Architecture
    MIPS = 8,
    /// IBM System/370 Processor
    S370 = 9,
    /// MIPS RS3000 Little-endian
    MIPS_RS3_LE = 10,
    // reserved	11-14	Reserved for future use
    /// Hewlett-Packard PA-RISC
    PARISC = 15,
    // reserved	16	Reserved for future use
    /// Fujitsu VPP500
    VPP500 = 17,
    /// Enhanced instruction set SPARC
    SPARC32PLUS = 18,
    /// Intel 80960
    I960 = 19,
    /// PowerPC
    PPC = 20,
    /// 64-bit PowerPC
    PPC64 = 21,
    /// IBM System/390 Processor
    S390 = 22,
    /// IBM SPU/SPC
    SPU = 23,
    // reserved	24-35	Reserved for future use
    /// NEC V800
    V800 = 36,
    /// Fujitsu FR20
    FR20 = 37,
    /// TRW RH-32
    RH32 = 38,
    /// Motorola RCE
    RCE = 39,
    /// ARM 32-bit architecture (AARCH32)
    ARM = 40,
    /// Digital Alpha
    ALPHA = 41,
    /// Hitachi SH
    SH = 42,
    /// SPARC Version 9
    SPARCV9 = 43,
    /// Siemens TriCore embedded processor
    TRICORE = 44,
    /// Argonaut RISC Core, Argonaut Technologies Inc.
    ARC = 45,
    /// Hitachi H8/300
    H8_300 = 46,
    /// Hitachi H8/300H
    H8_300H = 47,
    /// Hitachi H8S
    H8S = 48,
    /// Hitachi H8/500
    H8_500 = 49,
    /// Intel IA-64 processor architecture
    IA_64 = 50,
    /// Stanford MIPS-X
    MIPS_X = 51,
    /// Motorola ColdFire
    COLDFIRE = 52,
    /// Motorola M68HC12
    M68HC12 = 53,
    /// Fujitsu MMA Multimedia Accelerator
    MMA = 54,
    /// Siemens PCP
    PCP = 55,
    /// Sony nCPU embedded RISC processor
    NCPU = 56,
    /// Denso NDR1 microprocessor
    NDR1 = 57,
    /// Motorola Star*Core processor
    STARCORE = 58,
    /// Toyota ME16 processor
    ME16 = 59,
    /// STMicroelectronics ST100 processor
    ST100 = 60,
    /// Advanced Logic Corp. TinyJ embedded processor family
    TINYJ = 61,
    /// AMD x86-64 architecture
    X86_64 = 62,
    /// Sony DSP Processor
    PDSP = 63,
    /// Digital Equipment Corp. PDP-10
    PDP10 = 64,
    /// Digital Equipment Corp. PDP-11
    PDP11 = 65,
    /// Siemens FX66 microcontroller
    FX66 = 66,
    /// STMicroelectronics ST9+ 8/16 bit microcontroller
    ST9PLUS = 67,
    /// STMicroelectronics ST7 8-bit microcontroller
    ST7 = 68,
    /// Motorola MC68HC16 Microcontroller
    M68HC16 = 69,
    /// Motorola MC68HC11 Microcontroller
    M68HC11 = 70,
    /// Motorola MC68HC08 Microcontroller
    M68HC08 = 71,
    /// Motorola MC68HC05 Microcontroller
    M68HC05 = 72,
    /// Silicon Graphics SVx
    SVX = 73,
    /// STMicroelectronics ST19 8-bit microcontroller
    ST19 = 74,
    /// Digital VAX
    VAX = 75,
    /// Axis Communications 32-bit embedded processor
    CRIS = 76,
    /// Infineon Technologies 32-bit embedded processor
    JAVELIN = 77,
    /// Element 14 64-bit DSP Processor
    FIREPATH = 78,
    /// LSI Logic 16-bit DSP Processor
    ZSP = 79,
    /// Donald Knuth's educational 64-bit processor
    MMIX = 80,
    /// Harvard University machine-independent object files
    HUANY = 81,
    /// SiTera Prism
    PRISM = 82,
    /// Atmel AVR 8-bit microcontroller
    AVR = 83,
    /// Fujitsu FR30
    FR30 = 84,
    /// Mitsubishi D10V
    D10V = 85,
    /// Mitsubishi D30V
    D30V = 86,
    /// NEC v850
    V850 = 87,
    /// Mitsubishi M32R
    M32R = 88,
    /// Matsushita MN10300
    MN10300 = 89,
    /// Matsushita MN10200
    MN10200 = 90,
    /// picoJava
    PJ = 91,
    /// OpenRISC 32-bit embedded processor
    OPENRISC = 92,
    /// ARC International ARCompact processor (old spelling/synonym: ARC_A5)
    ARC_COMPACT = 93,
    /// Tensilica Xtensa Architecture
    XTENSA = 94,
    /// Alphamosaic VideoCore processor
    VIDEOCORE = 95,
    /// Thompson Multimedia General Purpose Processor
    TMM_GPP = 96,
    /// National Semiconductor 32000 series
    NS32K = 97,
    /// Tenor Network TPC processor
    TPC = 98,
    /// Trebia SNP 1000 processor
    SNP1K = 99,
    /// STMicroelectronics (www.st.com) ST200 microcontroller
    ST200 = 100,
    /// Ubicom IP2xxx microcontroller family
    IP2K = 101,
    /// MAX Processor
    MAX = 102,
    /// National Semiconductor CompactRISC microprocessor
    CR = 103,
    /// Fujitsu F2MC16
    F2MC16 = 104,
    /// Texas Instruments embedded microcontroller msp430
    MSP430 = 105,
    /// Analog Devices Blackfin (DSP) processor
    BLACKFIN = 106,
    /// S1C33 Family of Seiko Epson processors
    SE_C33 = 107,
    /// Sharp embedded microprocessor
    SEP = 108,
    /// Arca RISC Microprocessor
    ARCA = 109,
    /// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University
    UNICORE = 110,
    /// eXcess: 16/32/64-bit configurable embedded CPU
    EXCESS = 111,
    /// Icera Semiconductor Inc. Deep Execution Processor
    DXP = 112,
    /// Altera Nios II soft-core processor
    ALTERA_NIOS2 = 113,
    /// National Semiconductor CompactRISC CRX microprocessor
    CRX = 114,
    /// Motorola XGATE embedded processor
    XGATE = 115,
    /// Infineon C16x/XC16x processor
    C166 = 116,
    /// Renesas M16C series microprocessors
    M16C = 117,
    /// Microchip Technology dsPIC30F Digital Signal Controller
    DSPIC30F = 118,
    /// Freescale Communication Engine RISC core
    CE = 119,
    /// Renesas M32C series microprocessors
    M32C = 120,
    // reserved	121-130	Reserved for future use
    /// Altium TSK3000 core
    TSK3000 = 131,
    /// Freescale RS08 embedded processor
    RS08 = 132,
    /// Analog Devices SHARC family of 32-bit DSP processors
    SHARC = 133,
    /// Cyan Technology eCOG2 microprocessor
    ECOG2 = 134,
    /// Sunplus S+core7 RISC processor
    SCORE7 = 135,
    /// New Japan Radio (NJR) 24-bit DSP Processor
    DSP24 = 136,
    /// Broadcom VideoCore III processor
    VIDEOCORE3 = 137,
    /// RISC processor for Lattice FPGA architecture
    LATTICEMICO32 = 138,
    /// Seiko Epson C17 family
    SE_C17 = 139,
    /// The Texas Instruments TMS320C6000 DSP family
    TI_C6000 = 140,
    /// The Texas Instruments TMS320C2000 DSP family
    TI_C2000 = 141,
    /// The Texas Instruments TMS320C55x DSP family
    TI_C5500 = 142,
    /// Texas Instruments Application Specific RISC Processor, 32bit fetch
    TI_ARP32 = 143,
    /// Texas Instruments Programmable Realtime Unit
    TI_PRU = 144,
    // reserved	145-159	Reserved for future use
    /// STMicroelectronics 64bit VLIW Data Signal Processor
    MMDSP_PLUS = 160,
    /// Cypress M8C microprocessor
    CYPRESS_M8C = 161,
    /// Renesas R32C series microprocessors
    R32C = 162,
    /// NXP Semiconductors TriMedia architecture family
    TRIMEDIA = 163,
    /// QUALCOMM DSP6 Processor
    QDSP6 = 164,
    /// Intel 8051 and variants
    I8051 = 165,
    /// STMicroelectronics STxP7x family of configurable and extensible RISC processors
    STXP7X = 166,
    /// Andes Technology compact code size embedded RISC processor family
    NDS32 = 167,
    /// Cyan Technology eCOG1X family
    ECOG1 = 168,
    /// Dallas Semiconductor MAXQ30 Core Micro-controllers
    MAXQ30 = 169,
    /// New Japan Radio (NJR) 16-bit DSP Processor
    XIMO16 = 170,
    /// M2000 Reconfigurable RISC Microprocessor
    MANIK = 171,
    /// Cray Inc. NV2 vector architecture
    CRAYNV2 = 172,
    /// Renesas RX family
    RX = 173,
    /// Imagination Technologies META processor architecture
    METAG = 174,
    /// MCST Elbrus general purpose hardware architecture
    MCST_ELBRUS = 175,
    /// Cyan Technology eCOG16 family
    ECOG16 = 176,
    /// National Semiconductor CompactRISC CR16 16-bit microprocessor
    CR16 = 177,
    /// Freescale Extended Time Processing Unit
    ETPU = 178,
    /// Infineon Technologies SLE9X core
    SLE9X = 179,
    /// Intel L10M
    L10M = 180,
    /// Intel K10M
    K10M = 181,
    // reserved	182	Reserved for future Intel use
    /// ARM 64-bit architecture (AARCH64)
    AARCH64 = 183,
    // reserved	184	Reserved for future ARM use
    /// Atmel Corporation 32-bit microprocessor family
    AVR32 = 185,
    /// STMicroeletronics STM8 8-bit microcontroller
    STM8 = 186,
    /// Tilera TILE64 multicore architecture family
    TILE64 = 187,
    /// Tilera TILEPro multicore architecture family
    TILEPRO = 188,
    /// Xilinx MicroBlaze 32-bit RISC soft processor core
    MICROBLAZE = 189,
    /// NVIDIA CUDA architecture
    CUDA = 190,
    /// Tilera TILE-Gx multicore architecture family
    TILEGX = 191,
    /// CloudShield architecture family
    CLOUDSHIELD = 192,
    /// KIPO-KAIST Core-A 1st generation processor family
    COREA_1ST = 193,
    /// KIPO-KAIST Core-A 2nd generation processor family
    COREA_2ND = 194,
    /// Synopsys ARCompact V2
    ARC_COMPACT2 = 195,
    /// Open8 8-bit RISC soft processor core
    OPEN8 = 196,
    /// Renesas RL78 family
    RL78 = 197,
    /// Broadcom VideoCore V processor
    VIDEOCORE5 = 198,
    /// Renesas 78KOR family
    R78KOR = 199,
    /// Freescale 56800EX Digital Signal Controller (DSC)
    F56800EX = 200,
    /// Beyond BA1 CPU architecture
    BA1 = 201,
    /// Beyond BA2 CPU architecture
    BA2 = 202,
    /// XMOS xCORE processor family
    XCORE = 203,
    /// Microchip 8-bit PIC(r) family
    MCHP_PIC = 204,
    /// Reserved by Intel
    INTEL205 = 205,
    /// Reserved by Intel
    INTEL206 = 206,
    /// Reserved by Intel
    INTEL207 = 207,
    /// Reserved by Intel
    INTEL208 = 208,
    /// Reserved by Intel
    INTEL209 = 209,
    /// KM211 KM32 32-bit processor
    KM32 = 210,
    /// KM211 KMX32 32-bit processor
    KMX32 = 211,
    /// KM211 KMX16 16-bit processor
    KMX16 = 212,
    /// KM211 KMX8 8-bit processor
    KMX8 = 213,
    /// KM211 KVARC processor
    KVARC = 214,
    /// Paneve CDP architecture family
    CDP = 215,
    /// Cognitive Smart Memory Processor
    COGE = 216,
    /// Bluechip Systems CoolEngine
    COOL = 217,
    /// Nanoradio Optimized RISC
    NORC = 218,
    /// CSR Kalimba architecture family
    CSR_KALIMBA = 219,
    /// Zilog Z80
    Z80 = 220,
    /// Controls and Data Services VISIUMcore processor
    VISIUM = 221,
    /// FTDI Chip FT32 high performance 32-bit RISC architecture
    FT32 = 222,
    /// Moxie processor family
    MOXIE = 223,
    /// AMD GPU architecture
    AMDGPU = 224,
    // 225 - 242 reserved
    /// RISC-V
    Riscv = 243,
    /// Linux BPF -- in-kernel virtual machine
    BPF = 247,
    /// C-SKY
    CSKY = 252,
    /// LoongArch
    LOONGARCH = 258,
}

impl<R> FromReader<R> for ElfHeaderMachine
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;

        context
            .read_half_word(reader)
            .and_then(|machine| match Self::from_u16(machine) {
                Some(m) => Ok(m),
                None => {
                    let error = Error::InvalidMachine {
                        context: ErrorContext::from_reader(reader, offset, size_of::<Self>())?,
                    };

                    context
                        .should_ignore(&error)
                        .then(|| Ok(Self::NONE))
                        .unwrap_or(Err(error))
                }
            })
    }
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
pub enum ElfHeaderVersion {
    /// Invalid version
    None = 0,
    /// Current version
    Current = 1,
}

impl<R> FromReader<R> for ElfHeaderVersion
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;

        context
            .read_word(reader)
            .and_then(|version| match Self::from_u32(version) {
                Some(v) => Ok(v),
                None => {
                    let error = Error::InvalidVersion {
                        context: ErrorContext::from_reader(reader, offset, size_of::<Self>())?,
                    };

                    context
                        .should_ignore(&error)
                        .then(|| Ok(Self::None))
                        .unwrap_or(Err(error))
                }
            })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Contains the virtual address of the entry point of the object file, if there
/// is one. Otherwise, the value is 0. Some architectures or platforms may require
/// a non-zero value for the entry point, but some may allow it.
pub struct ElfHeaderEntry {
    entry: u64,
}

impl<R> FromReader<R> for ElfHeaderEntry
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;

        // TODO: Handle invalid address

        context.read_offset(reader).map(|entry| Self { entry })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The program header table file offset in bytes. If the file has no program header table, the value is 0.
pub struct ElfHeaderProgramHeaderOffset {
    offset: u64,
}

impl<R> FromReader<R> for ElfHeaderProgramHeaderOffset
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_offset(reader).map(|offset| Self { offset })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The section header table file offset in bytes. If the file has no section header table, the value is 0.
pub struct ElfHeaderSectionHeaderOffset {
    offset: u64,
}

impl<R> FromReader<R> for ElfHeaderSectionHeaderOffset
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_offset(reader).map(|offset| Self { offset })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The processor-specific flags associated with the file. Flag names and values for the flags are architecture-specific.
pub struct ElfHeaderFlags {
    flags: u32,
}

impl<R> FromReader<R> for ElfHeaderFlags
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_word(reader).map(|flags| Self { flags })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The size in bytes of the ELF header
pub struct ElfHeaderSize {
    size: u16,
}

impl<R> FromReader<R> for ElfHeaderSize
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_half_word(reader).map(|size| Self { size })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The size in bytes of a program header table entry
pub struct ElfHeaderProgramHeaderEntrySize {
    size: u16,
}

impl<R> FromReader<R> for ElfHeaderProgramHeaderEntrySize
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_half_word(reader).map(|size| Self { size })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The number of entries in the program header table
pub struct ElfHeaderProgramHeaderEntryNumber {
    number: u16,
}

impl<R> FromReader<R> for ElfHeaderProgramHeaderEntryNumber
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_half_word(reader).map(|number| Self { number })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The size in bytes of a section header table entry
pub struct ElfHeaderSectionHeaderEntrySize {
    size: u16,
}

impl<R> FromReader<R> for ElfHeaderSectionHeaderEntrySize
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_half_word(reader).map(|size| Self { size })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The number of entries in the section header table
pub struct ElfHeaderSectionHeaderEntryNumber {
    number: u16,
}

impl<R> FromReader<R> for ElfHeaderSectionHeaderEntryNumber
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_half_word(reader).map(|number| Self { number })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The section header table index of the entry associated with the section name string table
pub struct ElfHeaderSectionHeaderStringTableIndex {
    index: u16,
}

impl<R> FromReader<R> for ElfHeaderSectionHeaderStringTableIndex
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_half_word(reader).map(|index| Self { index })
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ElfHeader {
    pub identification: ElfHeaderIdentification,
    pub r#type: ElfHeaderType,
    pub machine: ElfHeaderMachine,
    pub version: ElfHeaderVersion,
    pub entry: ElfHeaderEntry,
    pub program_header_offset: ElfHeaderProgramHeaderOffset,
    pub section_header_offset: ElfHeaderSectionHeaderOffset,
    pub flags: ElfHeaderFlags,
    pub size: ElfHeaderSize,
    pub program_header_entry_size: ElfHeaderProgramHeaderEntrySize,
    pub program_header_entry_number: ElfHeaderProgramHeaderEntryNumber,
    pub section_header_entry_size: ElfHeaderSectionHeaderEntrySize,
    pub section_header_entry_number: ElfHeaderSectionHeaderEntryNumber,
    pub section_header_string_table_index: ElfHeaderSectionHeaderStringTableIndex,
    pub data: Vec<u8>,
}

impl<R> FromReader<R> for ElfHeader
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let identification = ElfHeaderIdentification::from_reader_with(reader, context)?;
        let r#type = ElfHeaderType::from_reader_with(reader, context)?;
        let machine = ElfHeaderMachine::from_reader_with(reader, context)?;
        let version = ElfHeaderVersion::from_reader_with(reader, context)?;
        let entry = ElfHeaderEntry::from_reader_with(reader, context)?;
        let program_header_offset =
            ElfHeaderProgramHeaderOffset::from_reader_with(reader, context)?;
        let section_header_offset =
            ElfHeaderSectionHeaderOffset::from_reader_with(reader, context)?;
        let flags = ElfHeaderFlags::from_reader_with(reader, context)?;
        let size = ElfHeaderSize::from_reader_with(reader, context)?;
        let program_header_entry_size =
            ElfHeaderProgramHeaderEntrySize::from_reader_with(reader, context)?;
        let program_header_entry_number =
            ElfHeaderProgramHeaderEntryNumber::from_reader_with(reader, context)?;
        let section_header_entry_size =
            ElfHeaderSectionHeaderEntrySize::from_reader_with(reader, context)?;
        let section_header_entry_number =
            ElfHeaderSectionHeaderEntryNumber::from_reader_with(reader, context)?;
        let section_header_string_table_index =
            ElfHeaderSectionHeaderStringTableIndex::from_reader_with(reader, context)?;
        let mut data = vec![
            0;
            (size.size as usize).saturating_sub(
                size_of::<ElfHeaderIdentification>()
                    + size_of::<ElfHeaderType>()
                    + size_of::<ElfHeaderMachine>()
                    + size_of::<ElfHeaderVersion>()
                    + size_of::<ElfHeaderEntry>()
                    + size_of::<ElfHeaderProgramHeaderOffset>()
                    + size_of::<ElfHeaderSectionHeaderOffset>()
                    + size_of::<ElfHeaderFlags>()
                    + size_of::<ElfHeaderSize>()
                    + size_of::<ElfHeaderProgramHeaderEntrySize>()
                    + size_of::<ElfHeaderProgramHeaderEntryNumber>()
                    + size_of::<ElfHeaderSectionHeaderEntrySize>()
                    + size_of::<ElfHeaderSectionHeaderEntryNumber>()
                    + size_of::<ElfHeaderSectionHeaderStringTableIndex>()
            )
        ];

        data.iter_mut().try_for_each(|d| {
            *d = context.read_uchar(reader)?;
            Ok::<(), Error>(())
        })?;

        Ok(Self {
            identification,
            r#type,
            machine,
            version,
            entry,
            program_header_offset,
            section_header_offset,
            flags,
            size,
            program_header_entry_size,
            program_header_entry_number,
            section_header_entry_size,
            section_header_entry_number,
            section_header_string_table_index,
            data,
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The index into the section header string table string where the section name is located
pub struct ElfSectionHeaderNameIndex {
    index: u32,
}

impl<R> FromReader<R> for ElfSectionHeaderNameIndex
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;
        context.read_word(reader).map(|index| Self { index })
    }
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The section's type
pub enum ElfSectionHeaderType {
    /// Section header table entry unused
    Null = 0,
    /// Program data
    ProgBits = 1,
    /// Symbol table
    SymTab = 2,
    /// String table
    StrTab = 3,
    /// Relocation entries with addends
    Rela = 4,
    /// Symbol hash table
    Hash = 5,
    /// Dynamic linking information
    Dynamic = 6,
    /// Note section
    Note = 7,
    /// Uninitialized space
    NoBits = 8,
    /// Relocation entries, no addends
    Rel = 9,
    /// Reserved
    ShLib = 10,
    /// Dynamic linker symbol table
    DynSym = 11,
    /// Array of constructors
    InitArray = 14,
    /// Array of destructors
    FiniArray = 15,
    /// Array of pre-constructors
    PreInitArray = 16,
    /// Section group
    Group = 17,
    /// Extended section indices
    SymTabShndx = 18,
    /// Number of defined types
    NumberDefined = 19,
    /// Start OS-specific
    LowOsSpecific = 0x60000000,
    /// End OS-specific
    HighOsSpecific = 0x6fffffff,
    /// Start processor-specific
    LowProcessorSpecific = 0x70000000,
    /// End processor-specific
    HighProcessorSpecific = 0x7fffffff,
    /// Start application-specific
    LowApplicationSpecific = 0x80000000,
    /// End application-specific
    HighApplicationSpecific = 0xffffffff,
}

impl<R> FromReader<R> for ElfSectionHeaderType
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let offset = reader.stream_position()?;

        context
            .read_word(reader)
            .and_then(|ty| match Self::from_u32(ty) {
                Some(t) => Ok(t),
                None => {
                    let error = Error::InvalidSectionHeaderType {
                        context: ErrorContext::from_reader(reader, offset, size_of::<Self>())?,
                    };

                    context
                        .should_ignore(&error)
                        .then(|| Ok(Self::Null))
                        .unwrap_or(Err(error))
                }
            })
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The section's flags
pub struct ElfSectionHeaderFlags(u32);

bitflags! {
    impl ElfSectionHeaderFlags: u32 {
        const WRITE = 0x1;
        const ALLOC = 0x2;
        const EXECINSTR = 0x4;
        const MERGE = 0x10;
        const STRINGS = 0x20;
        const INFO_LINK = 0x40;
        const LINK_ORDER = 0x80;
        const OS_NONCONFORMING = 0x100;
        const GROUP = 0x200;
        const TLS = 0x400;
        const COMPRESSED = 0x800;
        const MASKOS = 0x0ff00000;
        const MASKPROC = 0xf0000000;

    }
}

impl<R> FromReader<R> for ElfSectionHeaderFlags
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, context: &mut Context) -> Result<Self, Self::Error> {
        let _offset = reader.stream_position()?;

        context.read_word(reader).map(Self).or_else(|error| {
            context
                .should_ignore(&error)
                .then(|| Ok(Self(0)))
                .unwrap_or(Err(error))
        })
    }
}

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
