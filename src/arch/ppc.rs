//! Architecture specific definitions for PowerPC

// NOTE: No architecture-specific ELF Header flags for PPC

use crate::{base::ElfWord, error::Error, header::elf::ElfMachine, TryFromWithConfig};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionHeaderTypePPC {
    /// Link editor is to sort the entries in this section based on the address
    /// specified in the associated symbol table entry
    Ordered = Self::ORDERED,
}

impl ElfSectionHeaderTypePPC {
    /// Link editor is to sort the entries in this section based on the address
    /// specified in the associated symbol table entry
    pub const ORDERED: u32 = 0x7FFFFFFF;
}

impl<const EC: u8, const ED: u8> From<ElfSectionHeaderTypePPC> for ElfWord<EC, ED> {
    fn from(value: ElfSectionHeaderTypePPC) -> Self {
        Self(value as u32)
    }
}

impl<const EC: u8, const ED: u8> From<&ElfSectionHeaderTypePPC> for ElfWord<EC, ED> {
    fn from(value: &ElfSectionHeaderTypePPC) -> Self {
        Self(*value as u32)
    }
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfSectionHeaderTypePPC {
    type Error = Error;

    fn try_from_with(
        value: ElfWord<EC, ED>,
        config: &mut crate::Config,
    ) -> Result<Self, Self::Error> {
        if !matches!(config.machine, Some(ElfMachine::PPC)) {
            return Err(Error::InvalidMachineForSectionHeaderType {
                machine: config.machine,
                expected_machines: vec![ElfMachine::PPC],
                value: value.0,
            });
        }

        if value.0 == Self::ORDERED {
            Ok(Self::Ordered)
        } else {
            Err(Error::InvalidSectionHeaderType {
                machine: config.machine,
                value: value.0,
            })
        }
    }
}
