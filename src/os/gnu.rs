//! GNU-specific definitions

use crate::{
    base::ElfWord, error::Error, header::elf::identification::ElfOSABI, TryFromWithConfig,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionHeaderTypeGNU {
    /// Incremental build data
    IncrementalInputs = Self::INCREMENTAL_INPUTS,
    /// Object attributes
    Attributes = Self::ATTRIBUTES,
    /// GNU style symbol hash table
    Hash = Self::HASH,
    /// List of prelink dependencies
    LibList = Self::LIBLIST,
    /// Versions defined by file
    VerDef = Self::VERDEF,
    /// Versions needed by file
    VerNeed = Self::VERNEED,
    /// Symbol versions
    VerSym = Self::VERSYM,
}

impl ElfSectionHeaderTypeGNU {
    /// Incremental build data
    pub const INCREMENTAL_INPUTS: u32 = 0x6fff4700;
    /// Object attributes
    pub const ATTRIBUTES: u32 = 0x6ffffff5;
    /// GNU style symbol hash table
    pub const HASH: u32 = 0x6ffffff6;
    /// List of prelink dependencies
    pub const LIBLIST: u32 = 0x6ffffff7;
    /// Versions defined by file
    pub const VERDEF: u32 = 0x6ffffffd;
    /// Versions needed by file
    pub const VERNEED: u32 = 0x6ffffffe;
    /// Symbol versions
    pub const VERSYM: u32 = 0x6fffffff;
}

impl<const EC: u8, const ED: u8> From<ElfSectionHeaderTypeGNU> for ElfWord<EC, ED> {
    fn from(value: ElfSectionHeaderTypeGNU) -> Self {
        Self(value as u32)
    }
}

impl<const EC: u8, const ED: u8> From<&ElfSectionHeaderTypeGNU> for ElfWord<EC, ED> {
    fn from(value: &ElfSectionHeaderTypeGNU) -> Self {
        Self(*value as u32)
    }
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfSectionHeaderTypeGNU {
    type Error = Error;

    fn try_from_with(
        value: ElfWord<EC, ED>,
        config: &mut crate::Config,
    ) -> Result<Self, Self::Error> {
        if !matches!(config.os_abi, Some(ElfOSABI::GnuLinux)) {
            return Err(Error::InvalidOsAbiForSectionHeaderType {
                os_abi: config.os_abi,
                expected_os_abis: vec![ElfOSABI::GnuLinux],
                value: value.0,
            });
        }
        match value.0 {
            Self::INCREMENTAL_INPUTS => Ok(Self::IncrementalInputs),
            Self::ATTRIBUTES => Ok(Self::Attributes),
            Self::HASH => Ok(Self::Hash),
            Self::LIBLIST => Ok(Self::LibList),
            Self::VERDEF => Ok(Self::VerDef),
            Self::VERNEED => Ok(Self::VerNeed),
            Self::VERSYM => Ok(Self::VerSym),
            _ => Err(Error::InvalidSectionHeaderType {
                machine: config.machine,
                value: value.0,
            }),
        }
    }
}
