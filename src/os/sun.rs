//! SUN-specific definitions

use crate::{
    base::ElfWord, error::Error, header::elf::identification::ElfOSABI, TryFromWithConfig,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionHeaderTypeSUN {
    /// Versions defined by file
    VerDef = Self::VERDEF,
    /// Versions needed by file
    VerNeed = Self::VERNEED,
    /// Symbol versions
    VerSym = Self::VERSYM,
}

impl ElfSectionHeaderTypeSUN {
    /// Versions defined by file
    pub const VERDEF: u32 = 0x6ffffffd;
    /// Versions needed by file
    pub const VERNEED: u32 = 0x6ffffffe;
    /// Symbol versions
    pub const VERSYM: u32 = 0x6fffffff;
}

impl<const EC: u8, const ED: u8> From<ElfSectionHeaderTypeSUN> for ElfWord<EC, ED> {
    fn from(value: ElfSectionHeaderTypeSUN) -> Self {
        Self(value as u32)
    }
}

impl<const EC: u8, const ED: u8> From<&ElfSectionHeaderTypeSUN> for ElfWord<EC, ED> {
    fn from(value: &ElfSectionHeaderTypeSUN) -> Self {
        Self(*value as u32)
    }
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfSectionHeaderTypeSUN {
    type Error = Error;

    fn try_from_with(
        value: ElfWord<EC, ED>,
        config: &mut crate::Config,
    ) -> Result<Self, Self::Error> {
        if !matches!(config.os_abi, Some(ElfOSABI::Solaris)) {
            return Err(Error::InvalidOsAbiForSectionHeaderType {
                os_abi: config.os_abi,
                expected_os_abis: vec![ElfOSABI::Solaris],
                value: value.0,
            });
        }

        match value.0 {
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
