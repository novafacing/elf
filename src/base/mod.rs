use crate::{Error, FromReader, Result, ToWriter};
use std::io::{Read, Write};

pub mod raw;

pub use raw::*;

macro_rules! impl_from_reader {
    ($type:ty, $size32:ty, $size64:ty) => {
        impl<const EC: u8, const ED: u8, R> FromReader<EC, ED, R> for $type
        where
            R: Read,
        {
            fn from_reader(reader: &mut R) -> Result<Self> {
                match (
                    crate::ElfClass::try_from(EC)?,
                    crate::ElfDataEncoding::try_from(ED)?,
                ) {
                    (crate::ElfClass::Elf32, crate::ElfDataEncoding::Lsb) => {
                        let mut buffer = [0; std::mem::size_of::<$size32>()];
                        reader
                            .read_exact(&mut buffer)
                            .map_err(|e| Error::Io { kind: e.kind() })?;
                        Ok(Self(<$size32>::from_le_bytes(buffer) as $size64))
                    }
                    (crate::ElfClass::Elf32, crate::ElfDataEncoding::Msb) => {
                        let mut buffer = [0; std::mem::size_of::<$size32>()];
                        reader
                            .read_exact(&mut buffer)
                            .map_err(|e| Error::Io { kind: e.kind() })?;
                        Ok(Self(<$size32>::from_be_bytes(buffer) as $size64))
                    }
                    (crate::ElfClass::Elf64, crate::ElfDataEncoding::Lsb) => {
                        let mut buffer = [0; std::mem::size_of::<$size64>()];
                        reader
                            .read_exact(&mut buffer)
                            .map_err(|e| Error::Io { kind: e.kind() })?;
                        Ok(Self(<$size64>::from_le_bytes(buffer) as $size64))
                    }
                    (crate::ElfClass::Elf64, crate::ElfDataEncoding::Msb) => {
                        let mut buffer = [0; std::mem::size_of::<$size64>()];
                        reader
                            .read_exact(&mut buffer)
                            .map_err(|e| Error::Io { kind: e.kind() })?;
                        Ok(Self(<$size64>::from_be_bytes(buffer) as $size64))
                    }
                    (_, _) => Err(Error::InvalidElfClassOrDataEncoding {
                        elf_class: EC,
                        elf_data_encoding: ED,
                    }),
                }
            }
        }
    };
}

macro_rules! impl_to_writer {
    ($type:ty, $size32:ty, $size64:ty) => {
        impl<const EC: u8, const ED: u8, W> ToWriter<EC, ED, W> for $type
        where
            W: Write,
        {
            fn to_writer(&self, writer: &mut W) -> Result<()> {
                match (
                    crate::ElfClass::try_from(EC)?,
                    crate::ElfDataEncoding::try_from(ED)?,
                ) {
                    (crate::ElfClass::Elf32, crate::ElfDataEncoding::Lsb) => {
                        let buffer =
                            self.0.to_le_bytes()[..std::mem::size_of::<$size32>()].to_vec();
                        writer
                            .write_all(&buffer)
                            .map_err(|e| Error::Io { kind: e.kind() })
                    }
                    (crate::ElfClass::Elf32, crate::ElfDataEncoding::Msb) => {
                        let buffer = self.0.to_be_bytes()[if std::mem::size_of::<$size32>()
                            != std::mem::size_of::<$size64>()
                        {
                            std::mem::size_of::<$size32>()
                        } else {
                            0
                        }..]
                            .to_vec();
                        writer
                            .write_all(&buffer)
                            .map_err(|e| Error::Io { kind: e.kind() })
                    }
                    (crate::ElfClass::Elf64, crate::ElfDataEncoding::Lsb) => {
                        let buffer =
                            self.0.to_le_bytes()[..std::mem::size_of::<$size64>()].to_vec();
                        writer
                            .write_all(&buffer)
                            .map_err(|e| Error::Io { kind: e.kind() })
                    }
                    (crate::ElfClass::Elf64, crate::ElfDataEncoding::Msb) => {
                        let buffer = self.0.to_be_bytes()[if std::mem::size_of::<$size32>()
                            != std::mem::size_of::<$size64>()
                        {
                            std::mem::size_of::<$size64>()
                        } else {
                            0
                        }..]
                            .to_vec();
                        writer
                            .write_all(&buffer)
                            .map_err(|e| Error::Io { kind: e.kind() })
                    }
                    (_, _) => Err(Error::InvalidElfClassOrDataEncoding {
                        elf_class: EC,
                        elf_data_encoding: ED,
                    }),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElfByte(pub(crate) u8);

impl From<ElfByte> for RawElfByte {
    fn from(byte: ElfByte) -> Self {
        byte.0
    }
}

impl From<RawElfByte> for ElfByte {
    fn from(byte: RawElfByte) -> Self {
        Self(byte)
    }
}

impl_from_reader!(ElfByte, RawElfByte, RawElfByte);
impl_to_writer!(ElfByte, RawElfByte, RawElfByte);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElfHalfWord(pub(crate) u16);

impl From<ElfHalfWord> for RawElf32HalfWord {
    fn from(half_word: ElfHalfWord) -> Self {
        half_word.0
    }
}

impl From<RawElf32HalfWord> for ElfHalfWord {
    fn from(half_word: RawElf32HalfWord) -> Self {
        Self(half_word)
    }
}

impl_from_reader!(ElfHalfWord, RawElf32HalfWord, RawElf64HalfWord);
impl_to_writer!(ElfHalfWord, RawElf32HalfWord, RawElf64HalfWord);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElfWord(pub(crate) u32);

impl From<ElfWord> for RawElf32Word {
    fn from(word: ElfWord) -> Self {
        word.0
    }
}

impl From<RawElf32Word> for ElfWord {
    fn from(word: RawElf32Word) -> Self {
        Self(word)
    }
}

impl_from_reader!(ElfWord, RawElf32Word, RawElf64Word);
impl_to_writer!(ElfWord, RawElf32Word, RawElf64Word);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElfSignedWord(pub(crate) i32);

impl From<ElfSignedWord> for RawElf32SignedWord {
    fn from(signed_word: ElfSignedWord) -> Self {
        signed_word.0
    }
}

impl From<RawElf32SignedWord> for ElfSignedWord {
    fn from(signed_word: RawElf32SignedWord) -> Self {
        Self(signed_word)
    }
}

impl_from_reader!(ElfSignedWord, RawElf32SignedWord, RawElf64SignedWord);
impl_to_writer!(ElfSignedWord, RawElf32SignedWord, RawElf64SignedWord);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElfExtendedWord(pub(crate) u64);

impl From<ElfExtendedWord> for RawElf32ExtendedWord {
    fn from(extended_word: ElfExtendedWord) -> Self {
        extended_word.0
    }
}

impl From<RawElf32ExtendedWord> for ElfExtendedWord {
    fn from(extended_word: RawElf32ExtendedWord) -> Self {
        Self(extended_word)
    }
}

impl_from_reader!(ElfExtendedWord, RawElf32ExtendedWord, RawElf64ExtendedWord);
impl_to_writer!(ElfExtendedWord, RawElf32ExtendedWord, RawElf64ExtendedWord);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElfSignedExtendedWord(pub(crate) i64);

impl From<ElfSignedExtendedWord> for RawElf32SignedExtendedWord {
    fn from(signed_extended_word: ElfSignedExtendedWord) -> Self {
        signed_extended_word.0
    }
}

impl From<RawElf32SignedExtendedWord> for ElfSignedExtendedWord {
    fn from(signed_extended_word: RawElf32SignedExtendedWord) -> Self {
        Self(signed_extended_word)
    }
}

impl_from_reader!(
    ElfSignedExtendedWord,
    RawElf32SignedExtendedWord,
    RawElf64SignedExtendedWord
);
impl_to_writer!(
    ElfSignedExtendedWord,
    RawElf32SignedExtendedWord,
    RawElf64SignedExtendedWord
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// An ElfAddress represents an address in an ELF object file.
///
/// It is either 32 or 64 bits long, depending on the ELF class, and is stored as 64 bits
/// internally, but is written as the correct size when serialized.
pub struct ElfAddress(pub(crate) u64);

impl From<ElfAddress> for RawElf32Address {
    fn from(address: ElfAddress) -> Self {
        address.0 as u32
    }
}

impl From<RawElf32Address> for ElfAddress {
    fn from(address: RawElf32Address) -> Self {
        Self(address as u64)
    }
}

impl From<ElfAddress> for RawElf64Address {
    fn from(address: ElfAddress) -> Self {
        address.0
    }
}

impl From<RawElf64Address> for ElfAddress {
    fn from(address: RawElf64Address) -> Self {
        Self(address)
    }
}

impl_from_reader!(ElfAddress, RawElf32Address, RawElf64Address);
impl_to_writer!(ElfAddress, RawElf32Address, RawElf64Address);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// An ElfOffset represents an offset in an ELF object file.
///
/// It is either 32 or 64 bits long, depending on the ELF class, and is stored as 64 bits
/// internally, but is written as the correct size when serialized.
pub struct ElfOffset(pub(crate) u64);

impl From<ElfOffset> for RawElf32Offset {
    fn from(offset: ElfOffset) -> Self {
        offset.0 as u32
    }
}

impl From<RawElf32Offset> for ElfOffset {
    fn from(offset: RawElf32Offset) -> Self {
        Self(offset as u64)
    }
}

impl From<ElfOffset> for RawElf64Offset {
    fn from(offset: ElfOffset) -> Self {
        offset.0
    }
}

impl From<RawElf64Offset> for ElfOffset {
    fn from(offset: RawElf64Offset) -> Self {
        Self(offset)
    }
}

impl_from_reader!(ElfOffset, RawElf32Offset, RawElf64Offset);
impl_to_writer!(ElfOffset, RawElf32Offset, RawElf64Offset);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElfSection(pub(crate) u16);

impl From<ElfSection> for RawElf32Section {
    fn from(section: ElfSection) -> Self {
        section.0
    }
}

impl From<RawElf32Section> for ElfSection {
    fn from(section: RawElf32Section) -> Self {
        Self(section)
    }
}

impl_from_reader!(ElfSection, RawElf32Section, RawElf64Section);
impl_to_writer!(ElfSection, RawElf32Section, RawElf64Section);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElfVersionSymbol(pub(crate) u16);

impl From<ElfVersionSymbol> for RawElf32VersionSymbol {
    fn from(symbol: ElfVersionSymbol) -> Self {
        symbol.0
    }
}

impl From<RawElf32VersionSymbol> for ElfVersionSymbol {
    fn from(symbol: RawElf32VersionSymbol) -> Self {
        Self(symbol)
    }
}

impl_from_reader!(
    ElfVersionSymbol,
    RawElf32VersionSymbol,
    RawElf64VersionSymbol
);
impl_to_writer!(
    ElfVersionSymbol,
    RawElf32VersionSymbol,
    RawElf64VersionSymbol
);

#[cfg(test)]
mod test {
    use crate::{ElfAddress, ElfByte, ElfClass, ElfDataEncoding, ElfOffset, FromReader, ToWriter};

    pub const BUFFER: [u8; 8] = [0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48];

    #[test]
    fn test_elf_byte() {
        let le32b =
            <ElfByte as FromReader<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::from_reader(
                &mut std::io::Cursor::new(BUFFER),
            )
            .unwrap();
        let be32b =
            <ElfByte as FromReader<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::from_reader(
                &mut std::io::Cursor::new(BUFFER),
            )
            .unwrap();
        let le64b =
            <ElfByte as FromReader<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::from_reader(
                &mut std::io::Cursor::new(BUFFER),
            )
            .unwrap();
        let be64b =
            <ElfByte as FromReader<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::from_reader(
                &mut std::io::Cursor::new(BUFFER),
            )
            .unwrap();

        let mut out = Vec::new();
        <ElfByte as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le32b, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfByte as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be32b, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfByte as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le64b, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfByte as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be64b, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_half_word() {
        let le32h = <crate::ElfHalfWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32h = <crate::ElfHalfWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64h = <crate::ElfHalfWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64h = <crate::ElfHalfWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <crate::ElfHalfWord as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le32h, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfHalfWord as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be32h, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfHalfWord as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le64h, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfHalfWord as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be64h, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_word() {
        let le32w = <crate::ElfWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32w = <crate::ElfWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64w = <crate::ElfWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64w = <crate::ElfWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <crate::ElfWord as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le32w, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfWord as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be32w, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfWord as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le64w, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfWord as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be64w, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_signed_word() {
        let le32sw = <crate::ElfSignedWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32sw = <crate::ElfSignedWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64sw = <crate::ElfSignedWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64sw = <crate::ElfSignedWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <crate::ElfSignedWord as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le32sw, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSignedWord as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be32sw, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSignedWord as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le64sw, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSignedWord as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be64sw, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_extended_word() {
        let le32ew = <crate::ElfExtendedWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32ew = <crate::ElfExtendedWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64ew = <crate::ElfExtendedWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64ew = <crate::ElfExtendedWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <crate::ElfExtendedWord as ToWriter<{ ElfClass::ELF32 }, {
            ElfDataEncoding::LSB
        }, _>>::to_writer(&le32ew, &mut out).unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfExtendedWord as ToWriter<{ ElfClass::ELF32 }, {
            ElfDataEncoding::MSB
        }, _>>::to_writer(&be32ew, &mut out).unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfExtendedWord as ToWriter<{ ElfClass::ELF64 }, {
            ElfDataEncoding::LSB
        }, _>>::to_writer(&le64ew, &mut out).unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfExtendedWord as ToWriter<{ ElfClass::ELF64 }, {
            ElfDataEncoding::MSB
        }, _>>::to_writer(&be64ew, &mut out).unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_signed_extended_word() {
        let le32sew = <crate::ElfSignedExtendedWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32sew = <crate::ElfSignedExtendedWord as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64sew = <crate::ElfSignedExtendedWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64sew = <crate::ElfSignedExtendedWord as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <crate::ElfSignedExtendedWord as ToWriter<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::to_writer(&le32sew, &mut out)
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSignedExtendedWord as ToWriter<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::to_writer(&be32sew, &mut out)
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSignedExtendedWord as ToWriter<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::to_writer(&le64sew, &mut out)
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSignedExtendedWord as ToWriter<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::to_writer(&be64sew, &mut out)
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_address() {
        let le32a = <ElfAddress as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32a = <ElfAddress as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64a = <ElfAddress as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64a = <ElfAddress as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <ElfAddress as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le32a, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfAddress as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be32a, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfAddress as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le64a, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfAddress as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be64a, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_offset() {
        let le32o = <ElfOffset as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32o = <ElfOffset as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64o = <ElfOffset as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64o = <ElfOffset as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <ElfOffset as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le32o, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfOffset as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be32o, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfOffset as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le64o, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <ElfOffset as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be64o, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_section() {
        let le32s = <crate::ElfSection as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32s = <crate::ElfSection as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64s = <crate::ElfSection as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64s = <crate::ElfSection as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <crate::ElfSection as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le32s, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSection as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be32s, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSection as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le64s, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfSection as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be64s, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }

    #[test]
    fn test_elf_version_symbol() {
        let le32vs = <crate::ElfVersionSymbol as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be32vs = <crate::ElfVersionSymbol as FromReader<
            { ElfClass::ELF32 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let le64vs = <crate::ElfVersionSymbol as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::LSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();
        let be64vs = <crate::ElfVersionSymbol as FromReader<
            { ElfClass::ELF64 },
            { ElfDataEncoding::MSB },
            _,
        >>::from_reader(&mut std::io::Cursor::new(&BUFFER))
        .unwrap();

        let mut out = Vec::new();
        <crate::ElfVersionSymbol as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le32vs, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfVersionSymbol as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be32vs, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfVersionSymbol as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, _>>::to_writer(
            &le64vs, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
        out.clear();

        <crate::ElfVersionSymbol as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, _>>::to_writer(
            &be64vs, &mut out,
        )
        .unwrap();
        assert_eq!(out, BUFFER[..out.len()]);
    }
}
