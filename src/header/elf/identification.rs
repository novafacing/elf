//! Implementation of the `ident` field of the ELF eader. This field is located
//! at the beginning of an ELF object file and specifies how it is to be decoded.

use std::{
    io::{Read, Seek, Write},
    mem::size_of,
};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use typed_builder::TypedBuilder;

use crate::{base::ElfByte, error::Error, Config, FromReader, HasWrittenSize, ToWriter};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
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

impl ElfClass {
    /// Constant u8 value for ELFCLASS32
    pub const ELF_CLASS_32: u8 = Self::Elf32 as u8;
    /// Constant u8 value for ELFCLASS64
    pub const ELF_CLASS_64: u8 = Self::Elf64 as u8;
    #[cfg(target_pointer_width = "32")]
    /// Default u8 value for ELFCLASS64
    pub const ELF_CLASS_DEFAULT: u8 = Self::ELF_CLASS_32;
    #[cfg(target_pointer_width = "64")]
    /// Default u8 value for ELFCLASS64
    pub const ELF_CLASS_DEFAULT: u8 = Self::ELF_CLASS_64;
}

impl<R> FromReader<R> for ElfClass
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let class = ElfByte::from_reader_with(reader, config)?;
        Self::from_u8(class.0).ok_or(Error::InvalidClass { class })
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

impl HasWrittenSize for ElfClass {
    const SIZE: usize = size_of::<ElfByte>();
}

impl ElfClass {
    /// Convert a constant u8 to an `ElfClass`
    pub(crate) const fn const_from_u8(val: u8) -> Self {
        if val == Self::ELF_CLASS_32 {
            Self::Elf32
        } else if val == Self::ELF_CLASS_64 {
            Self::Elf64
        } else {
            Self::None
        }
    }
}

impl Default for ElfClass {
    fn default() -> Self {
        Self::const_from_u8(Self::ELF_CLASS_DEFAULT)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
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

impl ElfDataEncoding {
    /// Constant u8 value for ELFDATA2LSB
    pub const ELF_DATA_ENCODING_LITTLE_ENDIAN: u8 = Self::LittleEndian as u8;
    /// Constant u8 value for ELFDATA2MSB
    pub const ELF_DATA_ENCODING_BIG_ENDIAN: u8 = Self::BigEndian as u8;
    #[cfg(target_endian = "little")]
    /// Default u8 value for ELFDATA2LSB
    pub const ELF_DATA_ENCODING_DEFAULT: u8 = Self::ELF_DATA_ENCODING_LITTLE_ENDIAN;
    #[cfg(target_endian = "big")]
    /// Default u8 value for ELFDATA2LSB
    pub const ELF_DATA_ENCODING_DEFAULT: u8 = Self::ELF_DATA_ENCODING_BIG_ENDIAN;
}

impl<R> FromReader<R> for ElfDataEncoding
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let encoding = ElfByte::from_reader_with(reader, config)?;
        Self::from_u8(encoding.0).ok_or(Error::InvalidDataEncoding { encoding })
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

impl HasWrittenSize for ElfDataEncoding {
    const SIZE: usize = size_of::<ElfByte>();
}

impl ElfDataEncoding {
    /// Convert a constant u8 to an `ElfClass`
    pub(crate) const fn const_from_u8(val: u8) -> Self {
        if val == Self::ELF_DATA_ENCODING_LITTLE_ENDIAN {
            Self::LittleEndian
        } else if val == Self::ELF_DATA_ENCODING_BIG_ENDIAN {
            Self::BigEndian
        } else {
            Self::None
        }
    }
}

impl Default for ElfDataEncoding {
    fn default() -> Self {
        Self::const_from_u8(Self::ELF_DATA_ENCODING_DEFAULT)
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

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let version = ElfByte::from_reader_with(reader, config)?;
        Self::from_u8(version.0).ok_or(Error::InvalidIdentifierVersion { version })
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

impl HasWrittenSize for ElfIdentifierVersion {
    const SIZE: usize = size_of::<ElfByte>();
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

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let os_abi = ElfByte::from_reader_with(reader, config)?;
        Self::from_u8(os_abi.0).ok_or(Error::InvalidOsAbi { os_abi })
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

impl HasWrittenSize for ElfOSABI {
    const SIZE: usize = size_of::<ElfByte>();
}

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
/// The identifier field of an ELF header. Note that this structure is only
/// decoded in order, with no regard to the file's class or data encoding, and
/// is therefore always decoded the same way for all architectures and platforms.
pub struct ElfHeaderIdentifier {
    /// The magic value indicating that this is an ELF file (0x7F, 'E', 'L', 'F' in ASCII)
    pub magic: [ElfByte; 4],
    /// The file's class. See [ElfClass].
    pub class: ElfClass,
    /// The file's data encoding. See [ElfDataEncoding].
    pub data_encoding: ElfDataEncoding,
    /// The file's version. See [ElfIdentifierVersion].
    pub version: ElfIdentifierVersion,
    /// The file's OS/ABI. See [ElfOSABI].
    pub os_abi: ElfOSABI,
    /// The ABI version
    ///
    /// Identifies the version of the ABI to which the object is targeted. This field is
    /// used to distinguish among incompatible versions of an ABI. The interpretation of
    /// this version number is dependent on the ABI identified by the EI_OSABI field. If no
    /// values are specified for the EI_OSABI field by the processor supplement or no
    /// version values are specified for the ABI determined by a particular value of the
    /// EI_OSABI byte, the value 0 shall be used for the EI_ABIVERSION byte; it indicates
    /// unspecified.
    pub abi_version: ElfByte,
    /// Marks the beginning of the unused bytes in the identifier. These bytes are
    /// reserved and set to zero; programs that read object ﬁles should ignore them. The
    /// value of EI_PAD will change in the future if currently unused bytes are given
    /// meanings.
    pub pad: [ElfByte; 7],
}

impl<R> FromReader<R> for ElfHeaderIdentifier
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let magic = [
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
        ];
        let class = ElfClass::from_reader_with(reader, config)?;
        let data_encoding = ElfDataEncoding::from_reader_with(reader, config)?;
        let version = ElfIdentifierVersion::from_reader_with(reader, config)?;
        let os_abi = ElfOSABI::from_reader_with(reader, config)?;
        let abi_version = ElfByte::from_reader_with(reader, config)?;
        let pad = [
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
            ElfByte::from_reader_with(reader, config)?,
        ];

        Ok(Self {
            magic,
            class,
            data_encoding,
            version,
            os_abi,
            abi_version,
            pad,
        })
    }
}

impl<W> ToWriter<W> for ElfHeaderIdentifier
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        self.magic[0].to_writer(writer)?;
        self.magic[1].to_writer(writer)?;
        self.magic[2].to_writer(writer)?;
        self.magic[3].to_writer(writer)?;
        self.class.to_writer(writer)?;
        self.data_encoding.to_writer(writer)?;
        self.version.to_writer(writer)?;
        self.os_abi.to_writer(writer)?;
        self.abi_version.to_writer(writer)?;
        self.pad[0].to_writer(writer)?;
        self.pad[1].to_writer(writer)?;
        self.pad[2].to_writer(writer)?;
        self.pad[3].to_writer(writer)?;
        self.pad[4].to_writer(writer)?;
        self.pad[5].to_writer(writer)?;
        self.pad[6].to_writer(writer)?;
        Ok(())
    }
}

impl HasWrittenSize for ElfHeaderIdentifier {
    const SIZE: usize = size_of::<ElfByte>() * 16;
}

impl std::fmt::Display for ElfHeaderIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "magic: {}",
            self.magic
                .iter()
                .map(|m| format!("{:#02x}", m.0))
                .collect::<Vec<_>>()
                .join(" ")
        )?;
        writeln!(f, "class: {:#02x} ({:?})", self.class as u8, self.class)?;
        writeln!(
            f,
            "data_encoding: {:#02x} ({:?})",
            self.data_encoding as u8, self.data_encoding
        )?;
        writeln!(
            f,
            "version: {:#02x} ({:?})",
            self.version as u8, self.version
        )?;
        writeln!(f, "os_abi: {:#02x} ({:?})", self.os_abi as u8, self.os_abi)?;
        writeln!(
            f,
            "abi_version: {:#02x} ({:?})",
            { self.abi_version.0 },
            self.abi_version.0
        )?;
        writeln!(
            f,
            "pad: {}",
            self.pad
                .iter()
                .map(|m| format!("{:#02x}", m.0))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Only need one test for identifier, it is not class/encoding dependent
    fn test_elf_identifier() {
        let mut bytes = &[
            // Magic
            0x7f, 0x45, 0x4c, 0x46, // Class (32)
            0x01, // Data encoding (LE)
            0x01, // Version (Current)
            0x01, // OS ABI (SystemV)
            0x00, // ABI Version
            0x00, // Padding
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let id = ElfHeaderIdentifier::from_reader(&mut std::io::Cursor::new(&mut bytes)).unwrap();

        let mut bytes_out = Vec::new();
        id.to_writer(&mut bytes_out).unwrap();
        assert_eq!(bytes, bytes_out.as_slice());
        assert_eq!(
            id,
            ElfHeaderIdentifier {
                magic: [ElfByte(0x7f), ElfByte(0x45), ElfByte(0x4c), ElfByte(0x46)],
                class: ElfClass::Elf32,
                data_encoding: ElfDataEncoding::LittleEndian,
                version: ElfIdentifierVersion::Current,
                os_abi: ElfOSABI::NoneSystemV,
                abi_version: ElfByte(0),
                pad: [ElfByte(0); 7],
            }
        );
    }
}
