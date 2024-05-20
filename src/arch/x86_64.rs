//! Architecture specific definitions for x86_64

// NOTE: x86_64 defines no e_flags values

use crate::{base::ElfWord, error::Error, header::elf::ElfMachine, TryFromWithConfig};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionHeaderTypeX86_64 {
    /// Unwind Table
    Unwind = Self::UNWIND,
}

impl ElfSectionHeaderTypeX86_64 {
    /// Constant value for [ElfSectionheaderTypeI386::Unwind]
    pub const UNWIND: u32 = 0x70000001;
}

impl<const EC: u8, const ED: u8> From<ElfSectionHeaderTypeX86_64> for ElfWord<EC, ED> {
    fn from(value: ElfSectionHeaderTypeX86_64) -> Self {
        Self(value as u32)
    }
}

impl<const EC: u8, const ED: u8> From<&ElfSectionHeaderTypeX86_64> for ElfWord<EC, ED> {
    fn from(value: &ElfSectionHeaderTypeX86_64) -> Self {
        Self(*value as u32)
    }
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfSectionHeaderTypeX86_64 {
    type Error = Error;

    fn try_from_with(
        value: ElfWord<EC, ED>,
        config: &mut crate::Config,
    ) -> Result<Self, Self::Error> {
        if !matches!(config.machine, Some(ElfMachine::X86_64)) {
            return Err(Error::InvalidMachineForSectionHeaderType {
                machine: config.machine,
                expected_machines: vec![ElfMachine::X86_64],
                value: value.0,
            });
        }

        if value.0 == Self::UNWIND {
            Ok(Self::Unwind)
        } else {
            Err(Error::InvalidSectionHeaderType {
                machine: config.machine,
                value: value.0,
            })
        }
    }
}
