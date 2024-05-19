use crate::{ElfByte, Error, FromReader, FromReaderHost, Result};
use std::io::Read;
use typed_builder::TypedBuilder;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ElfClass {
    None = Self::NONE,
    Elf32 = Self::ELF32,
    Elf64 = Self::ELF64,
}

impl ElfClass {
    pub const NONE: u8 = 0;
    pub const ELF32: u8 = 1;
    pub const ELF64: u8 = 2;
    #[cfg(target_pointer_width = "32")]
    pub const HOST: u8 = Self::ELF32;
    #[cfg(target_pointer_width = "64")]
    pub const HOST: u8 = Self::ELF64;
}

impl TryFrom<u8> for ElfClass {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            Self::NONE => Ok(Self::None),
            Self::ELF32 => Ok(Self::Elf32),
            Self::ELF64 => Ok(Self::Elf64),
            o => Err(Error::InvalidElfClass { value: o }),
        }
    }
}

impl<const EC: u8, const ED: u8, R> FromReader<EC, ED, R> for ElfClass
where
    R: Read,
{
    fn from_reader(reader: &mut R) -> Result<Self> {
        let value = <ElfByte as FromReader<EC, ED, _>>::from_reader(reader)?;
        Self::try_from(value.0)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ElfDataEncoding {
    None = Self::NONE,
    Lsb = Self::LSB,
    Msb = Self::MSB,
}

impl ElfDataEncoding {
    pub const NONE: u8 = 0;
    pub const LSB: u8 = 1;
    pub const MSB: u8 = 2;
    #[cfg(target_endian = "little")]
    pub const HOST: u8 = Self::LSB;
    #[cfg(target_endian = "big")]
    pub const HOST: u8 = Self::MSB;
}

impl TryFrom<u8> for ElfDataEncoding {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            Self::NONE => Ok(Self::None),
            Self::LSB => Ok(Self::Lsb),
            Self::MSB => Ok(Self::Msb),
            o => Err(Error::InvalidElfDataEncoding { value: o }),
        }
    }
}

impl<const EC: u8, const ED: u8, R> FromReader<EC, ED, R> for ElfDataEncoding
where
    R: Read,
{
    fn from_reader(reader: &mut R) -> Result<Self> {
        let value = <ElfByte as FromReader<EC, ED, _>>::from_reader(reader)?;
        Self::try_from(value.0)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ElfIdentifierVersion {
    None = Self::NONE,
    Current = Self::CURRENT,
}

impl ElfIdentifierVersion {
    pub const NONE: u8 = 0;
    pub const CURRENT: u8 = 1;
}

impl TryFrom<u8> for ElfIdentifierVersion {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            Self::NONE => Ok(Self::None),
            Self::CURRENT => Ok(Self::Current),
            o => Err(Error::InvalidElfIdentifierVersion { value: o }),
        }
    }
}

impl<const EC: u8, const ED: u8, R> FromReader<EC, ED, R> for ElfIdentifierVersion
where
    R: Read,
{
    fn from_reader(reader: &mut R) -> Result<Self> {
        let value = <ElfByte as FromReader<EC, ED, _>>::from_reader(reader)?;
        Self::try_from(value.0)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    NoneSystemV = Self::NONE_SYSTEM_V,
    /// HP-UX
    Hpux = Self::HPUX,
    /// NetBSD
    NetBSD = Self::NETBSD,
    /// Object uses GNU ELF extensions.
    GnuLinux = Self::GNU_LINUX,
    /// SUN Solaris
    Solaris = Self::SOLARIS,
    /// IBM AIX
    Aix = Self::AIX,
    /// SGI Irix
    Irix = Self::IRIX,
    /// FreeBSD
    FreeBSD = Self::FREEBSD,
    /// Compaq TRU64 UNIX
    Tru64 = Self::TRU64,
    /// Novell Modesto
    NovellModesto = Self::NOVELL_MODESTO,
    /// OpenBSD
    OpenBSD = Self::OPENBSD,
    /// Open Virtual Memory System
    OpenVMS = Self::OPEN_VMS,
    /// NSK Non-Stop Kernel
    NonStopKernel = Self::NON_STOP_KERNEL,
    /// Amiga Research OS
    Aros = Self::AROS,
    /// FenixOS Highly scalable multi-core OS
    FenixOS = Self::FENIX_OS,
    /// Nuxi CloudABI
    CloudABI = Self::CLOUD_ABI,
    /// Stratus Technologies OpenVOS
    OpenVOS = Self::OPEN_VOS,
    /// ARM EABI (the object file contains symbol versioning extensions as described
    /// in the aaelf32 documentation)
    ///
    /// NOTE: This value is specified by the the ARM ABI processor supplement.
    ///
    /// NOTE: This value is overloaded, it also means C6000 ELFABI (Bare-metal TMS320C60000),
    /// and AMDGPU HSA Runtime
    ArmExtendedApplicationBinaryInterface = Self::ARM_EXTENDED_APPLICATION_BINARY_INTERFACE,
    /// FDPIC ELF for either XTensa or ARM, depending on the detected machine. For ARM, this
    /// is described in the fdpic document.
    ///
    /// NOTE: This value is specified by the the ARM ABI processor supplement and the
    /// XTensa ABI processor supplement, respectively, depending on the detected machine.
    ///
    /// NOTE: This value is overloaded, it also means C6000 Linux (TMS320C6000 Linux),
    /// and AMDGPU PAL Runtime.
    ArmXTensaFunctionDescriptorPositionIndependentCode =
        Self::ARM_XTENSA_FUNCTION_DESCRIPTOR_POSITION_INDEPENDENT_CODE,
    /// AMDGPU MESA3D Runtime
    AmdGpuMesa3DRuntime = Self::AMD_GPU_MESA3D_RUNTIME,
    /// ARM (non-EABI)
    Arm = Self::ARM,
    /// Standalone system
    Standalone = Self::STANDALONE,
}

impl ElfOSABI {
    pub const NONE_SYSTEM_V: u8 = 0;
    pub const HPUX: u8 = 1;
    pub const NETBSD: u8 = 2;
    pub const GNU_LINUX: u8 = 3;
    pub const SOLARIS: u8 = 6;
    pub const AIX: u8 = 7;
    pub const IRIX: u8 = 8;
    pub const FREEBSD: u8 = 9;
    pub const TRU64: u8 = 10;
    pub const NOVELL_MODESTO: u8 = 11;
    pub const OPENBSD: u8 = 12;
    pub const OPEN_VMS: u8 = 13;
    pub const NON_STOP_KERNEL: u8 = 14;
    pub const AROS: u8 = 15;
    pub const FENIX_OS: u8 = 16;
    pub const CLOUD_ABI: u8 = 17;
    pub const OPEN_VOS: u8 = 18;
    pub const ARM_EXTENDED_APPLICATION_BINARY_INTERFACE: u8 = 64;
    pub const ARM_XTENSA_FUNCTION_DESCRIPTOR_POSITION_INDEPENDENT_CODE: u8 = 65;
    pub const AMD_GPU_MESA3D_RUNTIME: u8 = 66;
    pub const ARM: u8 = 97;
    pub const STANDALONE: u8 = 255;
}

impl TryFrom<u8> for ElfOSABI {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            Self::NONE_SYSTEM_V => Ok(Self::NoneSystemV),
            Self::HPUX => Ok(Self::Hpux),
            Self::NETBSD => Ok(Self::NetBSD),
            Self::GNU_LINUX => Ok(Self::GnuLinux),
            Self::SOLARIS => Ok(Self::Solaris),
            Self::AIX => Ok(Self::Aix),
            Self::IRIX => Ok(Self::Irix),
            Self::FREEBSD => Ok(Self::FreeBSD),
            Self::TRU64 => Ok(Self::Tru64),
            Self::NOVELL_MODESTO => Ok(Self::NovellModesto),
            Self::OPENBSD => Ok(Self::OpenBSD),
            Self::OPEN_VMS => Ok(Self::OpenVMS),
            Self::NON_STOP_KERNEL => Ok(Self::NonStopKernel),
            Self::AROS => Ok(Self::Aros),
            Self::FENIX_OS => Ok(Self::FenixOS),
            Self::CLOUD_ABI => Ok(Self::CloudABI),
            Self::OPEN_VOS => Ok(Self::OpenVOS),
            Self::ARM_EXTENDED_APPLICATION_BINARY_INTERFACE => {
                Ok(Self::ArmExtendedApplicationBinaryInterface)
            }
            Self::ARM_XTENSA_FUNCTION_DESCRIPTOR_POSITION_INDEPENDENT_CODE => {
                Ok(Self::ArmXTensaFunctionDescriptorPositionIndependentCode)
            }
            Self::AMD_GPU_MESA3D_RUNTIME => Ok(Self::AmdGpuMesa3DRuntime),
            Self::ARM => Ok(Self::Arm),
            Self::STANDALONE => Ok(Self::Standalone),
            o => Err(Error::InvalidElfOsAbi { value: o }),
        }
    }
}

impl<const EC: u8, const ED: u8, R> FromReader<EC, ED, R> for ElfOSABI
where
    R: Read,
{
    fn from_reader(reader: &mut R) -> Result<Self> {
        let value = <ElfByte as FromReader<EC, ED, _>>::from_reader(reader)?;
        Self::try_from(value.0)
    }
}

#[repr(C)]
#[derive(TypedBuilder, Debug, Clone, PartialEq, Eq, Hash)]
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

impl<R> FromReader<{ ElfClass::HOST }, { ElfDataEncoding::HOST }, R> for ElfHeaderIdentifier
where
    R: Read,
{
    fn from_reader(reader: &mut R) -> Result<Self> {
        Ok(Self {
            magic: [
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
            ],
            class: <ElfClass as FromReaderHost<_>>::from_reader_host(reader)?,
            data_encoding: <ElfDataEncoding as FromReaderHost<_>>::from_reader_host(reader)?,
            version: <ElfIdentifierVersion as FromReaderHost<_>>::from_reader_host(reader)?,
            os_abi: <ElfOSABI as FromReaderHost<_>>::from_reader_host(reader)?,
            abi_version: <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
            pad: [
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
                <ElfByte as FromReaderHost<_>>::from_reader_host(reader)?,
            ],
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_elf_identifier() {
        use super::*;
        use std::io::Cursor;

        let data = [
            0x7F, 0x45, 0x4C, 0x46, // Magic
            0x02, // Class
            0x01, // Data Encoding
            0x01, // Version
            0x00, // OS/ABI
            0x00, // ABI Version
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
        ];

        let mut cursor = Cursor::new(&data);
        let ident = ElfHeaderIdentifier::from_reader_host(&mut cursor).unwrap();
        assert_eq!(
            ident.magic,
            [ElfByte(0x7F), ElfByte(0x45), ElfByte(0x4C), ElfByte(0x46)]
        );
        assert_eq!(ident.class, ElfClass::Elf64);
        assert_eq!(ident.data_encoding, ElfDataEncoding::Lsb);
        assert_eq!(ident.version, ElfIdentifierVersion::Current);
        assert_eq!(ident.os_abi, ElfOSABI::NoneSystemV);
        assert_eq!(ident.abi_version.0, 0);
        assert_eq!(ident.pad, [ElfByte(0); 7]);
    }
}
