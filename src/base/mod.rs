//! Base types for ELF object file structures

use num_traits::FromPrimitive;
use std::{
    fmt::Display,
    io::{Read, Seek, Write},
    mem::size_of,
};

use crate::{
    error::Error,
    header::elf::identification::{ElfClass, ElfDataEncoding},
    Config, FromReader, HasWrittenSize, ToWriter,
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A byte in an ELF file. Always represented as a single byte.
pub struct ElfByte(pub(crate) u8);

impl<R> FromReader<R> for ElfByte
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let mut buf = [0; size_of::<RawElfByte>()];
        reader
            .read_exact(&mut buf)
            .map_err(|e| Error::Io { kind: e.kind() })
            .or_else(|e| {
                if config.ignore.contains(&e) {
                    Ok(())
                } else {
                    Err(e)
                }
            })?;
        Ok(ElfByte(buf[0]))
    }
}

impl<W> ToWriter<W> for ElfByte
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        writer
            .write_all(&[self.0])
            .map_err(|e| Error::Io { kind: e.kind() })?;
        Ok(())
    }
}

impl HasWrittenSize for ElfByte {
    const SIZE: usize = size_of::<RawElfByte>();
}

impl Display for ElfByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ElfByte> for u8 {
    fn from(val: ElfByte) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A half-word in an ELF file. Represented as 16 bits for both classes.
pub struct ElfHalfWord<const EC: u8, const ED: u8>(pub(crate) RawElf64HalfWord);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfHalfWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };

        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64HalfWord>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfHalfWord::<EC, ED>(RawElf64HalfWord::from_le_bytes(buf)))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64HalfWord>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfHalfWord::<EC, ED>(RawElf64HalfWord::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfHalfWord<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32HalfWord>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64HalfWord>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfHalfWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const EC: u8, const ED: u8> From<ElfHalfWord<EC, ED>> for u16 {
    fn from(val: ElfHalfWord<EC, ED>) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A word in an ELF file. Always represented as 32 bits for both classes.
pub struct ElfWord<const EC: u8, const ED: u8>(pub(crate) RawElf64Word);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };

        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64Word>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfWord::<EC, ED>(RawElf64Word::from_le_bytes(buf)))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64Word>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfWord::<EC, ED>(RawElf64Word::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfWord<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32Word>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64Word>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const EC: u8, const ED: u8> From<ElfWord<EC, ED>> for u32 {
    fn from(val: ElfWord<EC, ED>) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A signed word in an ELF file. Represented as 32 bits for both classes.
pub struct ElfSignedWord<const EC: u8, const ED: u8>(pub RawElf64SignedWord);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfSignedWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };

        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64SignedWord>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfSignedWord::<EC, ED>(RawElf64SignedWord::from_le_bytes(
                    buf,
                )))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64SignedWord>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfSignedWord::<EC, ED>(RawElf64SignedWord::from_be_bytes(
                    buf,
                )))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfSignedWord<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32SignedWord>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64SignedWord>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfSignedWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const EC: u8, const ED: u8> From<ElfSignedWord<EC, ED>> for i32 {
    fn from(val: ElfSignedWord<EC, ED>) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An extended word in an ELF file. Represented as 64 bits for both classes.
pub struct ElfExtendedWord<const EC: u8, const ED: u8>(pub u64);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfExtendedWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };

        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64ExtendedWord>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfExtendedWord::<EC, ED>(
                    RawElf64ExtendedWord::from_le_bytes(buf),
                ))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64ExtendedWord>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfExtendedWord::<EC, ED>(
                    RawElf64ExtendedWord::from_be_bytes(buf),
                ))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfExtendedWord<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32ExtendedWord>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64ExtendedWord>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfExtendedWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A signed extended word in an ELF file. Represented as 64 bits for both classes.
pub struct ElfSignedExtendedWord<const EC: u8, const ED: u8>(pub i64);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfSignedExtendedWord<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64SignedExtendedWord>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfSignedExtendedWord::<EC, ED>(
                    RawElf64SignedExtendedWord::from_le_bytes(buf),
                ))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64SignedExtendedWord>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfSignedExtendedWord::<EC, ED>(
                    RawElf64SignedExtendedWord::from_be_bytes(buf),
                ))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfSignedExtendedWord<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32SignedExtendedWord>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64SignedExtendedWord>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfSignedExtendedWord<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const EC: u8, const ED: u8> From<ElfSignedExtendedWord<EC, ED>> for i64 {
    fn from(val: ElfSignedExtendedWord<EC, ED>) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An address in an ELF file. Represented as 32 bits for class 32 and 64 bits for class 64.
pub struct ElfAddress<const EC: u8, const ED: u8>(pub u64);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfAddress<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf32Address>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfAddress::<EC, ED>(
                    RawElf32Address::from_le_bytes(buf) as u64
                ))
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf32Address>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfAddress::<EC, ED>(
                    RawElf32Address::from_be_bytes(buf) as u64
                ))
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64Address>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfAddress::<EC, ED>(RawElf64Address::from_le_bytes(buf)))
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64Address>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfAddress::<EC, ED>(RawElf64Address::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&(self.0 as u32).to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&(self.0 as u32).to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfAddress<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32Address>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64Address>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfAddress<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const EC: u8, const ED: u8> From<ElfAddress<EC, ED>> for u64 {
    fn from(val: ElfAddress<EC, ED>) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An offset in an ELF file. Represented as 32 bits for class 32 and 64 bits for class 64.
pub struct ElfOffset<const EC: u8, const ED: u8>(pub u64);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfOffset<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf32Offset>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfOffset::<EC, ED>(
                    RawElf32Offset::from_le_bytes(buf) as u64
                ))
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf32Offset>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfOffset::<EC, ED>(
                    RawElf32Offset::from_be_bytes(buf) as u64
                ))
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64Offset>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfOffset::<EC, ED>(RawElf64Offset::from_le_bytes(buf)))
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64Offset>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfOffset::<EC, ED>(RawElf64Offset::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&(self.0 as u32).to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&(self.0 as u32).to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfOffset<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32Offset>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64Offset>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfOffset<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const EC: u8, const ED: u8> From<ElfOffset<EC, ED>> for u64 {
    fn from(val: ElfOffset<EC, ED>) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A section index in an ELF file. Represented as 16 bits for both classes.
pub struct ElfSection<const EC: u8, const ED: u8>(pub u16);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfSection<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64Section>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfSection::<EC, ED>(RawElf64Section::from_le_bytes(buf)))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64Section>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfSection::<EC, ED>(RawElf64Section::from_be_bytes(buf)))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfSection<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32Section>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64Section>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfSection<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const EC: u8, const ED: u8> From<ElfSection<EC, ED>> for u16 {
    fn from(val: ElfSection<EC, ED>) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A version symbol in an ELF file. Represented as 16 bits for both classes.
pub struct ElfVersionSymbol<const EC: u8, const ED: u8>(pub u16);

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfVersionSymbol<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let Some(class) = ElfClass::from_u8(EC) else {
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                let mut buf = [0; size_of::<RawElf64VersionSymbol>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfVersionSymbol::<EC, ED>(
                    RawElf64VersionSymbol::from_le_bytes(buf),
                ))
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                let mut buf = [0; size_of::<RawElf64VersionSymbol>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| Error::Io { kind: e.kind() })
                    .or_else(|e| {
                        if config.ignore.contains(&e) {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })?;
                Ok(ElfVersionSymbol::<EC, ED>(
                    RawElf64VersionSymbol::from_be_bytes(buf),
                ))
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
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
            return Err(Error::InvalidConstantClass { class: EC });
        };
        let Some(data_encoding) = ElfDataEncoding::from_u8(ED) else {
            return Err(Error::InvalidConstantDataEncoding { encoding: ED });
        };
        match (class, data_encoding) {
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                writer
                    .write_all(&self.0.to_le_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (ElfClass::Elf32 | ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                writer
                    .write_all(&self.0.to_be_bytes())
                    .map_err(|e| Error::Io { kind: e.kind() })?;
                Ok(())
            }
            (_, _) => Err(Error::InvalidConstantClass { class: EC }),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfVersionSymbol<EC, ED> {
    const SIZE: usize = {
        match (EC, ED) {
            (ElfClass::ELF_CLASS_32, _) => size_of::<RawElf32VersionSymbol>(),
            (ElfClass::ELF_CLASS_64, _) => size_of::<RawElf64VersionSymbol>(),
            (_, _) => panic!("Invalid class"),
        }
    };
}

impl<const EC: u8, const ED: u8> Display for ElfVersionSymbol<EC, ED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const EC: u8, const ED: u8> From<ElfVersionSymbol<EC, ED>> for u16 {
    fn from(val: ElfVersionSymbol<EC, ED>) -> Self {
        val.0
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

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;
    use paste::paste;

    macro_rules! size_test {
        ($ty:ty, $size_le32:expr, $size_be32:expr, $size_le64:expr, $size_be64:expr) => {
            paste! {
                #[test]
                fn [<test_ $ty:lower _le32_size>]() {
                    let mut out_le32 = Vec::new();
                    $ty::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>(0).to_writer(&mut out_le32).unwrap();
                    assert_eq!(
                        out_le32.len(),
                        $size_le32,
                        "Size of {} is {} bytes, expected {}",
                        stringify!($ty),
                        out_le32.len(),
                        $size_le32
                    );
                }

                #[test]
                fn [<test_ $ty:lower _be32_size>]() {
                    let mut out_be32 = Vec::new();
                    $ty::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>(0).to_writer(&mut out_be32).unwrap();
                    assert_eq!(
                        out_be32.len(),
                        $size_be32,
                        "Size of {} is {} bytes, expected {}",
                        stringify!($ty),
                        out_be32.len(),
                        $size_be32
                    );
                }

                #[test]
                fn [<test_ $ty:lower _le64_size>]() {
                    let mut out_le64 = Vec::new();
                    $ty::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>(0).to_writer(&mut out_le64).unwrap();
                    assert_eq!(
                        out_le64.len(),
                        $size_le64,
                        "Size of {} is {} bytes, expected {}",
                        stringify!($ty),
                        out_le64.len(),
                        $size_le64
                    );
                }

                #[test]
                fn [<test_ $ty:lower _be64_size>]() {
                    let mut out_be64 = Vec::new();
                    $ty::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>(0).to_writer(&mut out_be64).unwrap();
                    assert_eq!(
                        out_be64.len(),
                        $size_be64,
                        "Size of {} is {} bytes, expected {}",
                        stringify!($ty),
                        out_be64.len(),
                        $size_be64
                    );
                }
            }
        };
    }

    size_test!(
        ElfHalfWord,
        size_of::<u16>(),
        size_of::<u16>(),
        size_of::<u16>(),
        size_of::<u16>()
    );

    size_test!(
        ElfWord,
        size_of::<u32>(),
        size_of::<u32>(),
        size_of::<u32>(),
        size_of::<u32>()
    );

    size_test!(
        ElfSignedWord,
        size_of::<i32>(),
        size_of::<i32>(),
        size_of::<i32>(),
        size_of::<i32>()
    );

    size_test!(
        ElfExtendedWord,
        size_of::<u64>(),
        size_of::<u64>(),
        size_of::<u64>(),
        size_of::<u64>()
    );

    size_test!(
        ElfSignedExtendedWord,
        size_of::<i64>(),
        size_of::<i64>(),
        size_of::<i64>(),
        size_of::<i64>()
    );

    size_test!(
        ElfAddress,
        size_of::<u32>(),
        size_of::<u32>(),
        size_of::<u64>(),
        size_of::<u64>()
    );

    size_test!(
        ElfOffset,
        size_of::<u32>(),
        size_of::<u32>(),
        size_of::<u64>(),
        size_of::<u64>()
    );

    size_test!(
        ElfSection,
        size_of::<u16>(),
        size_of::<u16>(),
        size_of::<u16>(),
        size_of::<u16>()
    );

    size_test!(
        ElfVersionSymbol,
        size_of::<u16>(),
        size_of::<u16>(),
        size_of::<u16>(),
        size_of::<u16>()
    );

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
