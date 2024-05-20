//! Implementation of the ELF Compression Header which is optionally located
//! at the beginning of a section and specifies how the section data is to be
//! decompressed.

use num_traits::FromPrimitive;
use std::{
    io::{Read, Seek, Write},
    mem::size_of,
};

use typed_builder::TypedBuilder;

use crate::{
    base::{ElfByte, ElfExtendedWord, ElfWord},
    error::ErrorContext,
    from_primitive, Config, FromReader, HasWrittenSize, ToWriter,
};
use crate::{error::Error, header::elf::identification::ElfClass};

from_primitive! {
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[non_exhaustive]
    /// The type of compression algorithm used
    enum ElfCompressionHeaderType<const EC: u8, const ED: u8> {
        /// No compression
        None = 0,
        /// ZLIB compression
        ZLib = 1,
        /// ZStd compression
        ZStd = 2,
    }
}

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfCompressionHeaderType<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        ElfCompressionHeaderType::from_u32(ElfWord::<EC, ED>::from_reader_with(reader, config)?.0)
            .ok_or(Error::InvalidCompressionHeaderType {
                context: ErrorContext::from_reader_at(reader, 0, 4)?,
            })
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfCompressionHeaderType<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfWord::<EC, ED>(*self as u32).to_writer(writer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
/// ELF 32-bit Compression Header
///
/// The sh_size and sh_addralign ﬁelds of the section header for a compressed section
/// reflect the requirements of the compressed section. The ch_size and ch_addralign
/// ﬁelds in the compression header provide the corresponding values for the
/// uncompressed data, thereby supplying the values that sh_size and sh_addralign would
/// have had if the section had not been compressed.  The layout and interpretation of
/// the data that follows the compression header is speciﬁc to each algorithm, and is
/// deﬁned below for each value of ch_type. This area may contain algorithm speciﬁc
/// parameters and alignment padding in addition to compressed data bytes.  A
/// compression header's ch_type member speciﬁes the compression algoritm employed, as
/// shown in the following table
pub struct Elf32CompressionHeader<const ED: u8> {
    /// Specifies the compression algorithm
    r#type: ElfCompressionHeaderType<{ ElfClass::Elf32 as u8 }, ED>,
    /// Provides the size in bytes of the uncompressed data
    size: ElfWord<{ ElfClass::Elf32 as u8 }, ED>,
    /// Specifies the required alignment for the uncompressed data
    address_align: ElfWord<{ ElfClass::Elf32 as u8 }, ED>,
}

impl<R, const ED: u8> FromReader<R> for Elf32CompressionHeader<ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let r#type = ElfCompressionHeaderType::<{ ElfClass::Elf32 as u8 }, ED>::from_reader_with(
            reader, config,
        )?;
        let size = ElfWord::<{ ElfClass::Elf32 as u8 }, ED>::from_reader_with(reader, config)?;
        let address_align =
            ElfWord::<{ ElfClass::Elf32 as u8 }, ED>::from_reader_with(reader, config)?;

        Ok(Self {
            r#type,
            size,
            address_align,
        })
    }
}

impl<W, const ED: u8> ToWriter<W> for Elf32CompressionHeader<ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        self.r#type.to_writer(writer)?;
        self.size.to_writer(writer)?;
        self.address_align.to_writer(writer)?;
        Ok(())
    }
}

impl<const ED: u8> HasWrittenSize for Elf32CompressionHeader<ED> {
    const SIZE: usize = size_of::<ElfCompressionHeaderType<{ ElfClass::Elf32 as u8 }, ED>>()
        + size_of::<ElfWord<{ ElfClass::Elf32 as u8 }, ED>>()
        + size_of::<ElfWord<{ ElfClass::Elf32 as u8 }, ED>>();
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
/// ELF 64-bit Compression Header
///
/// The sh_size and sh_addralign ﬁelds of the section header for a compressed section
/// reflect the requirements of the compressed section. The ch_size and ch_addralign
/// ﬁelds in the compression header provide the corresponding values for the
/// uncompressed data, thereby supplying the values that sh_size and sh_addralign would
/// have had if the section had not been compressed.  The layout and interpretation of
/// the data that follows the compression header is speciﬁc to each algorithm, and is
/// deﬁned below for each value of ch_type. This area may contain algorithm speciﬁc
/// parameters and alignment padding in addition to compressed data bytes.  A
/// compression header's ch_type member speciﬁes the compression algoritm employed, as
/// shown in the following table
pub struct Elf64CompressionHeader<const ED: u8> {
    /// Specifies the compression algorithm
    r#type: ElfCompressionHeaderType<{ ElfClass::Elf64 as u8 }, ED>,
    /// Reserved
    reserved: ElfWord<{ ElfClass::Elf64 as u8 }, ED>,
    /// Provides the size in bytes of the uncompressed data
    size: ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>,
    /// Specifies the required alignment for the uncompressed data
    address_align: ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>,
}

impl<R, const ED: u8> FromReader<R> for Elf64CompressionHeader<ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let r#type = ElfCompressionHeaderType::<{ ElfClass::Elf64 as u8 }, ED>::from_reader_with(
            reader, config,
        )?;
        let reserved = ElfWord::<{ ElfClass::Elf64 as u8 }, ED>::from_reader_with(reader, config)?;
        let size =
            ElfExtendedWord::<{ ElfClass::Elf64 as u8 }, ED>::from_reader_with(reader, config)?;
        let address_align =
            ElfExtendedWord::<{ ElfClass::Elf64 as u8 }, ED>::from_reader_with(reader, config)?;

        Ok(Self {
            r#type,
            reserved,
            size,
            address_align,
        })
    }
}

impl<W, const ED: u8> ToWriter<W> for Elf64CompressionHeader<ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        self.r#type.to_writer(writer)?;
        self.reserved.to_writer(writer)?;
        self.size.to_writer(writer)?;
        self.address_align.to_writer(writer)?;
        Ok(())
    }
}

impl<const ED: u8> HasWrittenSize for Elf64CompressionHeader<ED> {
    const SIZE: usize = size_of::<ElfWord<{ ElfClass::Elf64 as u8 }, ED>>()
        + size_of::<ElfWord<{ ElfClass::Elf64 as u8 }, ED>>()
        + size_of::<ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>>()
        + size_of::<ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>>();
}

/// ELF compression header for either 32-bit or 64-bit ELF files
pub enum ElfCompressionHeader<const EC: u8, const ED: u8> {
    /// A 32-bit ELF compression header
    Elf32(Elf32CompressionHeader<ED>),
    /// A 64-bit ELF compression header
    Elf64(Elf64CompressionHeader<ED>),
}

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfCompressionHeader<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        Ok(
            match ElfClass::from_u8(EC).ok_or(Error::InvalidClass { class: ElfByte(EC) })? {
                ElfClass::None => return Err(Error::InvalidClass { class: ElfByte(EC) }),
                ElfClass::Elf32 => ElfCompressionHeader::Elf32(
                    Elf32CompressionHeader::from_reader_with(reader, config)?,
                ),
                ElfClass::Elf64 => ElfCompressionHeader::Elf64(
                    Elf64CompressionHeader::from_reader_with(reader, config)?,
                ),
            },
        )
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfCompressionHeader<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        match self {
            ElfCompressionHeader::Elf32(header) => header.to_writer(writer),
            ElfCompressionHeader::Elf64(header) => header.to_writer(writer),
        }
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfCompressionHeader<EC, ED> {
    const SIZE: usize = match ElfClass::const_from_u8(EC) {
        ElfClass::Elf32 => Elf32CompressionHeader::<ED>::SIZE,
        ElfClass::Elf64 => Elf64CompressionHeader::<ED>::SIZE,
        _ => panic!("Constant ELF Class must be valid"),
    };
}
