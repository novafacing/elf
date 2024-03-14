//! Definitions for ELF Files

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive as _;
use std::{
    fmt::Display,
    io::{Read, Seek, Write},
    mem::size_of,
};
use typed_builder::TypedBuilder;

#[derive(thiserror::Error, Debug)]
/// Error type for errors during ELF object handling
pub enum Error {
    #[error(transparent)]
    /// Error from IO operations
    Io(#[from] std::io::Error),
    #[error("Invalid ELF class {class}")]
    /// Invalid ELF class value
    InvalidClass {
        /// The value that could not be interpreted as a class value
        class: ElfByte,
    },
    #[error("Invalid ELF data encoding {encoding}")]
    /// Invalid ELF data encoding value
    InvalidDataEncoding {
        /// The value that could not be interpreted as a data encoding value
        encoding: ElfByte,
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
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub offset: u64,
    pub context: Vec<u8>
}

impl ErrorContext {
    pub fn from_reader<R>(reader: &mut R, offset: u64, size: usize) -> Result<Self, std::io::Error>
    where
        R: Read + Seek,
    {
        let mut context = vec![0; size];
        reader.seek(std::io::SeekFrom::Start(offset))?;
        reader.read_exact(&mut context)?;
        Ok(ErrorContext {
            offset,
            context
        })
    
}

/// Decode an owned instance of a type from a reader
pub trait FromReader<R>
where
    R: Read + Seek,
    Self: Sized,
{
    /// The error type for this operation
    type Error;

    /// Decode an instance of this type from a reader
    fn from_reader(reader: &mut R) -> Result<Self, Self::Error>;
}

/// Encode an instance of a type to a writer
pub trait ToWriter<W>
where
    W: Write,
    Self: Sized,
{
    /// The error type for this operation
    type Error;

    /// Encode an instance of this type to a writer
    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error>;
}

/// Raw representation of a byte in an ELF file
pub type RawElfByte = u8;
/// Raw representation of a half-word in an ELF class 32 file
pub type RawElf32HalfWord = u16;
/// Raw representation of a word in an ELF class 32 file
pub type RawElf32Word = u32;
/// Raw representation of a signed word in an ELF class 32 file
pub type RawElf32SignedWord = i32;
/// Raw representation of an extended word in an ELF class 32 file
pub type RawElf32ExtendedWord = u64;
/// Raw representation of a signed extended word in an ELF class 32 file
pub type RawElf32SignedExtendedWord = i64;
/// Raw representation of an address in an ELF class 32 file
pub type RawElf32Address = u32;
/// Raw representation of an offset in an ELF class 32 file
pub type RawElf32Offset = u32;
/// Raw representation of a section index in an ELF class 32 file
pub type RawElf32Section = u16;
/// Raw representation of a version symbol in an ELF class 32 file
pub type RawElf32VersionSymbol = u16;
/// Raw representation of a half-word in an ELF class 64 file
pub type RawElf64HalfWord = u16;
/// Raw representation of a word in an ELF class 64 file
pub type RawElf64Word = u32;
/// Raw representation of a signed word in an ELF class 64 file
pub type RawElf64SignedWord = i32;
/// Raw representation of an extended word in an ELF class 64 file
pub type RawElf64ExtendedWord = u64;
/// Raw representation of a signed extended word in an ELF class 64 file
pub type RawElf64SignedExtendedWord = i64;
/// Raw representation of an address in an ELF class 64 file
pub type RawElf64Address = u64;
/// Raw representation of an offset in an ELF class 64 file
pub type RawElf64Offset = u64;
/// Raw representation of a section index in an ELF class 64 file
pub type RawElf64Section = u16;
/// Raw representation of a version symbol in an ELF class 64 file
pub type RawElf64VersionSymbol = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A byte in an ELF file. Always represented as a single byte.
pub struct ElfByte(u8);

impl<R> FromReader<R> for ElfByte
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0; size_of::<RawElfByte>()];
        reader.read_exact(&mut buf)?;
        Ok(ElfByte(buf[0]))
    }
}

impl<W> ToWriter<W> for ElfByte
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        writer.write_all(&[self.0])?;
        Ok(())
    }
}

impl Display for ElfByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A half-word in an ELF file. Represented as 16 bits for both classes.
pub struct ElfHalfWord<const EC: u8, const ED: u8>(pub RawElf64HalfWord);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfHalfWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };

        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64HalfWord>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfHalfWord::<EC, ED>(RawElf64HalfWord::from_le_bytes(buf)))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64HalfWord>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfHalfWord::<EC, ED>(RawElf64HalfWord::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfHalfWord<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfHalfWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A word in an ELF file. Always represented as 32 bits for both classes.
///
pub struct ElfWord<const EC: u8, const ED: u8>(pub RawElf64Word);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };

        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64Word>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfWord::<EC, ED>(RawElf64Word::from_le_bytes(buf)))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64Word>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfWord::<EC, ED>(RawElf64Word::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfWord<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A signed word in an ELF file. Represented as 32 bits for both classes.
pub struct ElfSignedWord<const EC: u8, const ED: u8>(pub RawElf64SignedWord);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfSignedWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };

        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64SignedWord>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfSignedWord::<EC, ED>(RawElf64SignedWord::from_le_bytes(
                    buf,
                )))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64SignedWord>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfSignedWord::<EC, ED>(RawElf64SignedWord::from_be_bytes(
                    buf,
                )))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfSignedWord<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfSignedWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// An extended word in an ELF file. Represented as 64 bits for both classes.
pub struct ElfExtendedWord<const EC: u8, const ED: u8>(pub u64);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfExtendedWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };

        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64ExtendedWord>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfExtendedWord::<EC, ED>(
                    RawElf64ExtendedWord::from_le_bytes(buf),
                ))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64ExtendedWord>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfExtendedWord::<EC, ED>(
                    RawElf64ExtendedWord::from_be_bytes(buf),
                ))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfExtendedWord<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfExtendedWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A signed extended word in an ELF file. Represented as 64 bits for both classes.
pub struct ElfSignedExtendedWord<const EC: u8, const ED: u8>(pub i64);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfSignedExtendedWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64SignedExtendedWord>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfSignedExtendedWord::<EC, ED>(
                    RawElf64SignedExtendedWord::from_le_bytes(buf),
                ))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64SignedExtendedWord>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfSignedExtendedWord::<EC, ED>(
                    RawElf64SignedExtendedWord::from_be_bytes(buf),
                ))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfSignedExtendedWord<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfSignedExtendedWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// An address in an ELF file. Represented as 32 bits for class 32 and 64 bits for class 64.
pub struct ElfAddress<const EC: u8, const ED: u8>(pub u64);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfAddress<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf32Address>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfAddress::<EC, ED>(
                    RawElf32Address::from_le_bytes(buf) as u64
                ))
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf32Address>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfAddress::<EC, ED>(
                    RawElf32Address::from_be_bytes(buf) as u64
                ))
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64Address>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfAddress::<EC, ED>(RawElf64Address::from_le_bytes(buf)))
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64Address>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfAddress::<EC, ED>(RawElf64Address::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfAddress<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&(self.0 as u32).to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                writer.write_all(&(self.0 as u32).to_be_bytes())?;
                Ok(())
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfAddress<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// An offset in an ELF file. Represented as 32 bits for class 32 and 64 bits for class 64.
pub struct ElfOffset<const EC: u8, const ED: u8>(pub u64);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfOffset<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf32Offset>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfOffset::<EC, ED>(
                    RawElf32Offset::from_le_bytes(buf) as u64
                ))
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf32Offset>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfOffset::<EC, ED>(
                    RawElf32Offset::from_be_bytes(buf) as u64
                ))
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64Offset>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfOffset::<EC, ED>(RawElf64Offset::from_le_bytes(buf)))
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64Offset>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfOffset::<EC, ED>(RawElf64Offset::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfOffset<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&(self.0 as u32).to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                writer.write_all(&(self.0 as u32).to_be_bytes())?;
                Ok(())
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfOffset<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A section index in an ELF file. Represented as 16 bits for both classes.
pub struct ElfSection<const EC: u8, const ED: u8>(pub u16);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfSection<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64Section>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfSection::<EC, ED>(RawElf64Section::from_le_bytes(buf)))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64Section>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfSection::<EC, ED>(RawElf64Section::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfSection<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfSection<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A version symbol in an ELF file. Represented as 16 bits for both classes.
pub struct ElfVersionSymbol<const EC: u8, const ED: u8>(pub u16);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfVersionSymbol<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64VersionSymbol>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfVersionSymbol::<EC, ED>(
                    RawElf64VersionSymbol::from_le_bytes(buf),
                ))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64VersionSymbol>()];
                reader.read_exact(&mut buf)?;
                Ok(ElfVersionSymbol::<EC, ED>(
                    RawElf64VersionSymbol::from_be_bytes(buf),
                ))
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfVersionSymbol<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidClass { class: ElfByte(EC) });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidDataEncoding {
                encoding: ElfByte(ED),
            });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer.write_all(&self.0.to_le_bytes())?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer.write_all(&self.0.to_be_bytes())?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidClass { class: ElfByte(EC) }),
        }
    }
}

impl<const EC: u8, const ED: u8> Display for ElfVersionSymbol<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 32-bit little-endian half word
pub type Elf32LEHalfWord =
    ElfHalfWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit little-endian word
pub type Elf32LEWord = ElfWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit little-endian signed word
pub type Elf32LESignedWord =
    ElfSignedWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit little-endian extended word
pub type Elf32LEExtendedWord =
    ElfExtendedWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit little-endian signed extended word
pub type Elf32LESignedExtendedWord =
    ElfSignedExtendedWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit little-endian address
pub type Elf32LEAddress =
    ElfAddress<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit little-endian offset
pub type Elf32LEOffset =
    ElfOffset<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit little-endian section
pub type Elf32LESection =
    ElfSection<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit little-endian version symbol
pub type Elf32LEVersionSymbol =
    ElfVersionSymbol<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 32-bit big-endian half word
pub type Elf32BEHalfWord =
    ElfHalfWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 32-bit big-endian word
pub type Elf32BEWord = ElfWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 32-bit big-endian signed word
pub type Elf32BESignedWord =
    ElfSignedWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 32-bit big-endian extended word
pub type Elf32BEExtendedWord =
    ElfExtendedWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 32-bit big-endian signed extended word
pub type Elf32BESignedExtendedWord =
    ElfSignedExtendedWord<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 32-bit big-endian address
pub type Elf32BEAddress =
    ElfAddress<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 32-bit big-endian offset
pub type Elf32BEOffset = ElfOffset<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 32-bit big-endian section
pub type Elf32BESection =
    ElfSection<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 32-bit big-endian version symbol
pub type Elf32BEVersionSymbol =
    ElfVersionSymbol<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit little-endian half word
pub type Elf64LEHalfWord =
    ElfHalfWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit little-endian word
pub type Elf64LEWord = ElfWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit little-endian signed word
pub type Elf64LESignedWord =
    ElfSignedWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit little-endian extended word
pub type Elf64LEExtendedWord =
    ElfExtendedWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit little-endian signed extended word
pub type Elf64LESignedExtendedWord =
    ElfSignedExtendedWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit little-endian address
pub type Elf64LEAddress =
    ElfAddress<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit little-endian offset
pub type Elf64LEOffset =
    ElfOffset<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit little-endian section
pub type Elf64LESection =
    ElfSection<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit little-endian version symbol
pub type Elf64LEVersionSymbol =
    ElfVersionSymbol<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>;
/// 64-bit big-endian half word
pub type Elf64BEHalfWord =
    ElfHalfWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit big-endian word
pub type Elf64BEWord = ElfWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit big-endian signed word
pub type Elf64BESignedWord =
    ElfSignedWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit big-endian extended word
pub type Elf64BEExtendedWord =
    ElfExtendedWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit big-endian signed extended word
pub type Elf64BESignedExtendedWord =
    ElfSignedExtendedWord<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit big-endian address
pub type Elf64BEAddress =
    ElfAddress<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit big-endian offset
pub type Elf64BEOffset = ElfOffset<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit big-endian section
pub type Elf64BESection =
    ElfSection<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;
/// 64-bit big-endian version symbol
pub type Elf64BEVersionSymbol =
    ElfVersionSymbol<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The file's class/capacity, i.e. whether it is 32-bit or 64-bit.
///
/// A file's data encoding and class specifies how to interpret the basic objects in a
/// file. Class ELFCLASS32 files use objects that occupy 1, 2, and 4 bytes. Class
/// ELFCLASS64 files use objects that occupy 1, 2, 4, and 8 bytes. Under the defined
/// encodings, objects are represented as shown below.
pub enum ElfClass {
    /// Unspecified (TODO: Make a best guess based on the file's contents)
    ///
    /// NOTE: It does not have to be a hard error to have a file with an unspecified
    /// class, but the decoder will rely on a best guess based on the file's contents
    /// which may not be accurate.
    None = 0,
    /// 32-bit
    Elf32 = 1,
    /// 64-bit
    Elf64 = 2,
}

impl<R> FromReader<R> for ElfClass
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let class = ElfByte::from_reader(reader)?;
        Self::from_u8(class.0).ok_or_else(|| Error::InvalidClass { class })
    }
}

impl<W> ToWriter<W> for ElfClass
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfByte(*self as u8).to_writer(writer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The file's data encoding, i.e. whether it is little-endian or big-endian.
///
/// A file's data encoding and class specifies how to interpret the basic objects in a
/// file. Encoding ELFDATA2LSB specifies 2's complement values, with the least
/// significant byte occupying the lowest address.
pub enum ElfDataEncoding {
    /// Unspecified or invalid data encoding (TODO: Make a best guess based on the
    /// file's contents)
    ///
    /// NOTE: It does not have to be a hard error to have a file with an unspecified
    /// data encoding, but the decoder will rely on a best guess based on the file's
    /// contents which may not be accurate.
    None = 0,
    /// Little-endian
    LittleEndian = 1,
    /// Big-endian
    BigEndian = 2,
}

impl<R> FromReader<R> for ElfDataEncoding
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let encoding = ElfByte::from_reader(reader)?;
        Self::from_u8(encoding.0).ok_or_else(|| Error::InvalidDataEncoding { encoding })
    }
}

impl<W> ToWriter<W> for ElfDataEncoding
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfByte(*self as u8).to_writer(writer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The file's version
pub enum ElfIdentifierVersion {
    /// Unspecified or invalid version
    ///
    /// NOTE: It is not a hard error to have an unspecified or invalid version,
    /// and this field can be effectively ignored if desired.
    None = 0,
    /// Current version
    Current = 1,
}

impl<R> FromReader<R> for ElfIdentifierVersion
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let version = ElfByte::from_reader(reader)?;
        Self::from_u8(version.0).ok_or_else(|| Error::InvalidIdentifierVersion { version })
    }
}

impl<W> ToWriter<W> for ElfIdentifierVersion
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfByte(*self as u8).to_writer(writer)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The file's OS/ABI
///
/// Identifies the OS- or ABI-specific ELF extensions used by this file. Some fields in
/// other ELF structures have ﬂags and values that have operating system and/or ABI
/// specific meanings; the interpretation of those fields is determined by the value of
/// this byte. If the object file does not use any extensions, it is recommended that
/// this byte be set to 0. If the value for this byte is 64 through 255, its meaning
/// depends on the value of the machine header member. The ABI processor supplement
/// for an architecture can define its own associated set of values for this byte in this
/// range. If the processor supplement does not specify a set of values, one of the
/// following values shall be used, where 0 can also be taken to mean unspecified.
///
/// NOTE: It is not a hard error to have an unspecified or invalid OS/ABI
pub enum ElfOSABI {
    /// Unix System V ABI or None, parsing None for this identifier field is *not* an
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
    ///
    /// NOTE: This value is specified by the the ARM ABI processor supplement.
    ArmExtendedApplicationBinaryInterface = 64,
    /// FDPIC ELF for either XTensa or ARM, depending on the detected machine. For ARM, this
    /// is described in the fdpic document.
    ///
    /// NOTE: This value is specified by the the ARM ABI processor supplement and the
    /// XTensa ABI processor supplement, respectively, depending on the detected machine.
    ArmXTensaFunctionDescriptorPositionIndependentCode = 65,
    /// ARM (non-EABI)
    Arm = 97,
    /// Standalone system
    Standalone = 255,
}

impl<R> FromReader<R> for ElfOSABI
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        let os_abi = ElfByte::from_reader(reader)?;
        Self::from_u8(os_abi.0).ok_or_else(|| Error::InvalidOsAbi { os_abi })
    }
}

impl<W> ToWriter<W> for ElfOSABI
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfByte(*self as u8).to_writer(writer)
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, TypedBuilder)]
/// The identifier field of an ELF header. Note that this structure is only
/// decoded in order, with no regard to the file's class or data encoding, and
/// is therefore always decoded the same way for all architectures and platforms.
pub struct ElfHeaderIdentifier {
    /// The magic value indicating that this is an ELF file (0x7F, 'E', 'L', 'F' in ASCII)
    magic: [ElfByte; 4],
    /// The file's class. See [ElfClass].
    class: ElfClass,
    /// The file's data encoding. See [ElfDataEncoding].
    data_encoding: ElfDataEncoding,
    /// The file's version. See [ElfIdentifierVersion].
    version: ElfIdentifierVersion,
    /// The file's OS/ABI. See [ElfOSABI].
    os_abi: ElfOSABI,
    /// The ABI version
    ///
    /// Identifies the version of the ABI to which the object is targeted. This field is
    /// used to distinguish among incompatible versions of an ABI. The interpretation of
    /// this version number is dependent on the ABI identified by the EI_OSABI field. If no
    /// values are specified for the EI_OSABI field by the processor supplement or no
    /// version values are specified for the ABI determined by a particular value of the
    /// EI_OSABI byte, the value 0 shall be used for the EI_ABIVERSION byte; it indicates
    /// unspecified.
    abi_version: ElfByte,
    /// Marks the beginning of the unused bytes in the identifier. These bytes are
    /// reserved and set to zero; programs that read object ﬁles should ignore them. The
    /// value of EI_PAD will change in the future if currently unused bytes are given
    /// meanings.
    pad: [ElfByte; 7],
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The ELF object type
pub enum ElfType {
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

#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The ELF object's machine
pub enum ElfMachine {
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

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[non_exhaustive]
/// The ELF object's version
pub enum ElfVersion {
    /// Invalid version
    None = 0,
    /// Current version
    Current = 1,
}

/// The header for an ELF object. Resides at the beginning and holds a ``road map''
/// describing the file's organization
pub struct ElfHeader<const EC: u8, const ED: u8> {
    /// The file's identifier information, which marks the file as an object file
    /// and provide machine- independent data with which to decode and interpret the
    pub identifier: ElfHeaderIdentifier,
    /// The object file type
    pub r#type: ElfType,
    /// The file's machine, which specifies the required architecture for this
    /// object file
    pub machine: ElfMachine,
    /// The object file version
    pub version: ElfVersion,
    /// The file's entrypoint. This is the virtual address to which the system
    /// first transfers control, thus starting the process. If the object has no
    /// associated entry point, this member is zero (absent).
    pub entrypoint: Option<ElfAddress<EC, ED>>,
    /// The program header table's file offset in bytes. If the file has no program
    /// header table, this member is zero (absent).
    pub program_header_offset: Option<ElfOffset<EC, ED>>,
    /// The section header table's file offset in bytes. If the file has no section
    /// header table, this member is zero (absent).
    pub section_header_offset: Option<ElfOffset<EC, ED>>,
    /// The processor-specific flags associated with the file.
    /// TODO: Make this a trait abstract over the various architectures' flags
    pub flags: ElfWord<EC, ED>,
    /// The ELF header's size in bytes
    pub header_size: ElfHalfWord<EC, ED>,
    /// The size in bytes of a program header table entry; all entries are the same
    /// size
    pub program_header_entry_size: ElfHalfWord<EC, ED>,
    /// The number of entries in the program header table. If the file has no
    /// program header table, this member is zero (absent).
    pub program_header_entry_count: ElfHalfWord<EC, ED>,
    /// The size in bytes of a section header table entry; all entries are the same
    /// size
    pub section_header_entry_size: ElfHalfWord<EC, ED>,
    /// The number of entries in the section header table.  Thus the product of
    /// e_shentsize and e_shnum gives the section header table's size in bytes. If a file
    /// has no section header table, e_shnum holds the value zero.  If the number of
    /// sections is greater than or equal to SHN_LORESERVE (0xﬀ00), this member has the
    /// value zero and the actual number of section header table entries is contained in
    /// the sh_size field of the section header at index 0. (Otherwise, the sh_size
    /// member of the initial entry contains 0.)
    pub section_header_entry_count: ElfHalfWord<EC, ED>,
    /// This member holds the section header table index of the entry associated with
    /// the section name string table. If the file has no section name string table, this
    /// member holds the value SHN_UNDEF. See ``Sections'' and ``String Table'' below
    /// for more information.  If the section name string table section index is greater
    /// than or equal to SHN_LORESERVE (0xﬀ00), this member has the value SHN_XINDEX
    /// (0xﬀﬀ) and the actual index of the section name string table section is
    /// contained in the sh_link field of the section header at index 0.  (Otherwise, the
    /// sh_link member of the initial entry contains 0.)
    pub section_name_string_table_index: ElfHalfWord<EC, ED>,
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_elf_half_word() {
        let mut val = &[0x01, 0x02];
        let le32hw: Elf32LEHalfWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32hw: Elf32BEHalfWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64hw: Elf64LEHalfWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64hw: Elf64BEHalfWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        assert_eq!(le32hw.0, 0x0201);
        assert_eq!(be32hw.0, 0x0102);
        assert_eq!(le64hw.0, 0x0201);
        assert_eq!(be64hw.0, 0x0102);

        let mut val_out = Vec::new();
        le32hw.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be32hw.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        le64hw.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64hw.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }

    #[test]
    fn test_elf_word() {
        let mut val = &[0x01, 0x02, 0x03, 0x04];
        let le32w: Elf32LEWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32w: Elf32BEWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64w: Elf64LEWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64w: Elf64BEWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        assert_eq!(le32w.0, 0x04030201);
        assert_eq!(be32w.0, 0x01020304);
        assert_eq!(le64w.0, 0x04030201);
        assert_eq!(be64w.0, 0x01020304);
        let mut val_out = Vec::new();
        le32w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be32w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        le64w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }

    #[test]
    fn test_elf_signed_word() {
        let mut val = &[0x01, 0x02, 0x03, 0x04];
        let le32w: Elf32LEWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32w: Elf32BEWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64w: Elf64LEWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64w: Elf64BEWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        assert_eq!(le32w.0, 0x04030201);
        assert_eq!(be32w.0, 0x01020304);
        assert_eq!(le64w.0, 0x04030201);
        assert_eq!(be64w.0, 0x01020304);
        let mut val_out = Vec::new();
        le32w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be32w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        le64w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }

    #[test]
    fn test_elf_extended_word() {
        let mut val = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let le32w: Elf32LEExtendedWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32w: Elf32BEExtendedWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64w: Elf64LEExtendedWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64w: Elf64BEExtendedWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        assert_eq!(le32w.0, 0x0807060504030201);
        assert_eq!(be32w.0, 0x0102030405060708);
        assert_eq!(le64w.0, 0x0807060504030201);
        assert_eq!(be64w.0, 0x0102030405060708);
        let mut val_out = Vec::new();
        le32w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be32w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        le64w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }

    #[test]
    fn test_elf_signed_extended_word() {
        let mut val = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let le32w: Elf32LESignedExtendedWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32w: Elf32BESignedExtendedWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64w: Elf64LESignedExtendedWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64w: Elf64BESignedExtendedWord =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        assert_eq!(le32w.0, 0x0807060504030201);
        assert_eq!(be32w.0, 0x0102030405060708);
        assert_eq!(le64w.0, 0x0807060504030201);
        assert_eq!(be64w.0, 0x0102030405060708);
        let mut val_out = Vec::new();
        le32w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be32w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        le64w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64w.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }

    #[test]
    fn test_elf_address() {
        let mut val = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let le32a: Elf32LEAddress =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32a: Elf32BEAddress =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64a: Elf64LEAddress =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64a: Elf64BEAddress =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();

        assert_eq!(le32a.0, 0x04030201);
        assert_eq!(be32a.0, 0x01020304);
        assert_eq!(le64a.0, 0x0807060504030201);
        assert_eq!(be64a.0, 0x0102030405060708);

        let mut val_out = Vec::new();
        le32a.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val[..4]);
        val_out.clear();
        be32a.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val[..4]);
        val_out.clear();
        le64a.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64a.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }

    #[test]
    fn test_elf_offset() {
        let mut val = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let le32o: Elf32LEOffset =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32o: Elf32BEOffset =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64o: Elf64LEOffset =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64o: Elf64BEOffset =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        assert_eq!(le32o.0, 0x04030201);
        assert_eq!(be32o.0, 0x01020304);
        assert_eq!(le64o.0, 0x0807060504030201);
        assert_eq!(be64o.0, 0x0102030405060708);
        let mut val_out = Vec::new();
        le32o.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val[..4]);
        val_out.clear();
        be32o.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val[..4]);
        val_out.clear();
        le64o.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64o.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }

    #[test]
    fn test_elf_section() {
        let mut val = &[0x01, 0x02];
        let le32s: Elf32LESection =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32s: Elf32BESection =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64s: Elf64LESection =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64s: Elf64BESection =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        assert_eq!(le32s.0, 0x0201);
        assert_eq!(be32s.0, 0x0102);
        assert_eq!(le64s.0, 0x0201);
        assert_eq!(be64s.0, 0x0102);
        let mut val_out = Vec::new();
        le32s.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be32s.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        le64s.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64s.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }

    #[test]
    fn test_elf_version_symbol() {
        let mut val = &[0x01, 0x02];
        let le32vs: Elf32LEVersionSymbol =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be32vs: Elf32BEVersionSymbol =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let le64vs: Elf64LEVersionSymbol =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        let be64vs: Elf64BEVersionSymbol =
            FromReader::from_reader(&mut std::io::Cursor::new(&mut val)).unwrap();
        assert_eq!(le32vs.0, 0x0201);
        assert_eq!(be32vs.0, 0x0102);
        assert_eq!(le64vs.0, 0x0201);
        assert_eq!(be64vs.0, 0x0102);
        let mut val_out = Vec::new();
        le32vs.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be32vs.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        le64vs.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
        val_out.clear();
        be64vs.to_writer(&mut val_out).unwrap();
        assert_eq!(val_out, val);
    }
}
