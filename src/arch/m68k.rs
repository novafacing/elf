//! Architecture specific definitions for m68k

use crate::{base::ElfWord, error::Error, Config, TryFromWithConfig};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagM68K {
    /// This object file runs on CPU32 version M68K family processors
    Cpu32 = Self::CPU32,
}

impl ElfHeaderFlagM68K {
    /// Constant value for [ElfHeaderFlagM68K::Cpu32]
    pub const CPU32: u32 = 0x00810000;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A set of semantically useful flags retrieved from the set of flags in the ELF header
pub struct ElfHeaderFlagsM68K<const EC: u8, const ED: u8> {
    flags: Vec<ElfHeaderFlagM68K>,
    value: ElfWord<EC, ED>,
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfHeaderFlagsM68K<EC, ED> {
    type Error = Error;

    fn try_from_with(value: ElfWord<EC, ED>, _config: &mut Config) -> Result<Self, Self::Error> {
        let mut flags = Vec::new();

        if value.0 & ElfHeaderFlagM68K::CPU32 != 0 {
            flags.push(ElfHeaderFlagM68K::Cpu32);
        }

        Ok(Self { flags, value })
    }
}
