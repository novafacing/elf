//! Architecture specific definitions for RISC-V

use std::io::Write;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;

use crate::{base::ElfWord, error::Error, header::elf::ElfMachine, ToWriter, TryFromWithConfig};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// RVC bits
pub enum ElfHeaderFlagRISCVRVC {
    /// The binary targets some other ABI
    NoRvc = Self::NO_RVC,
    /// The binary targets the C ABI, which allows instructions to be aligned to 16- bit
    /// boundaries (the base RV32 and RV64 ISAs only allow 32-bit instruction
    /// alignment). When linking  objects  which  specify  EF_RISCV_RVC,  the  linker
    /// is  permitted  to  use  RVC  instructions such as C.JAL in the linker relaxation
    /// process
    Rvc = Self::RVC,
}

impl ElfHeaderFlagRISCVRVC {
    /// The binary targets some other ABI
    pub const NO_RVC: u32 = 0x00000000;
    /// The binary targets the C ABI, which allows instructions to be aligned to 16- bit
    /// boundaries (the base RV32 and RV64 ISAs only allow 32-bit instruction
    /// alignment). When linking  objects  which  specify  EF_RISCV_RVC,  the  linker
    /// is  permitted  to  use  RVC  instructions such as C.JAL in the linker relaxation
    /// process
    pub const RVC: u32 = 0x00000001;
    /// Mask
    pub const MASK: u32 = 0x00000001;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// Float ABI bits
pub enum ElfHeaderFlagRISCVFloatAbi {
    /// The binary targets the soft-float ABI
    Soft = Self::SOFT,
    /// The single precision ABI
    Single = Self::SINGLE,
    /// The double precision ABI
    Double = Self::DOUBLE,
    /// The quadruple precision ABI
    Quad = Self::QUAD,
}

impl ElfHeaderFlagRISCVFloatAbi {
    /// The binary targets the soft-float ABI
    pub const SOFT: u32 = 0x00000000;
    /// The single precision ABI
    pub const SINGLE: u32 = 0x00000002;
    /// The double precision ABI
    pub const DOUBLE: u32 = 0x00000004;
    /// The quadruple precision ABI
    pub const QUAD: u32 = 0x00000006;
    /// Mask
    pub const MASK: u32 = 0x00000006;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// RVE (E ABI bits)
pub enum ElfHeaderFlagRISCVEAbi {
    /// The binary targets the base ISA
    Base = Self::BASE,
    /// The binary targets the E ISA
    EIsa = Self::E_ISA,
}

impl ElfHeaderFlagRISCVEAbi {
    /// The binary targets the base ISA
    pub const BASE: u32 = 0x00000000;
    /// The binary targets the E ISA
    pub const E_ISA: u32 = 0x00000008;
    /// Mask
    pub const MASK: u32 = 0x00000008;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// This bit is set when the binary requires the RVTSO memory consistency model
pub enum ElfHeaderFlagRISCVMemoryModel {
    /// The binary requires the base memory consistency model
    Base = Self::BASE,
    /// The binary requires the RVTSO memory consistency model
    RvtsO = Self::RVTSO,
}

impl ElfHeaderFlagRISCVMemoryModel {
    /// The binary requires the base memory consistency model
    pub const BASE: u32 = 0x00000000;
    /// The binary requires the RVTSO memory consistency model
    pub const RVTSO: u32 = 0x00000010;
    /// Mask
    pub const MASK: u32 = 0x00000010;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagRISCV {
    /// Rvc
    Rvc(ElfHeaderFlagRISCVRVC),
    /// Float ABI
    FloatAbi(ElfHeaderFlagRISCVFloatAbi),
    /// RVE (E ABI bits)
    EAbi(ElfHeaderFlagRISCVEAbi),
    /// Memory model
    MemoryModel(ElfHeaderFlagRISCVMemoryModel),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub struct ElfHeaderFlagsRISCV<const EC: u8, const ED: u8> {
    flags: Vec<ElfHeaderFlagRISCV>,
    value: ElfWord<EC, ED>,
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>>
    for ElfHeaderFlagsRISCV<EC, ED>
{
    type Error = Error;

    fn try_from_with(
        value: ElfWord<EC, ED>,
        config: &mut crate::Config,
    ) -> Result<Self, Self::Error> {
        let mut flags = Vec::new();

        if value.0 & ElfHeaderFlagRISCVRVC::MASK != 0 {
            flags.push(ElfHeaderFlagRISCV::Rvc(
                ElfHeaderFlagRISCVRVC::from_u32(value.0 & ElfHeaderFlagRISCVRVC::MASK).ok_or(
                    Error::InvalidHeaderFlagForMachine {
                        machine: config.machine,
                        value: value.0,
                    },
                )?,
            ));
        }

        if value.0 & ElfHeaderFlagRISCVFloatAbi::MASK != 0 {
            flags.push(ElfHeaderFlagRISCV::FloatAbi(
                ElfHeaderFlagRISCVFloatAbi::from_u32(value.0 & ElfHeaderFlagRISCVFloatAbi::MASK)
                    .ok_or(Error::InvalidHeaderFlagForMachine {
                        machine: config.machine,
                        value: value.0,
                    })?,
            ));
        }

        if value.0 & ElfHeaderFlagRISCVEAbi::MASK != 0 {
            flags.push(ElfHeaderFlagRISCV::EAbi(
                ElfHeaderFlagRISCVEAbi::from_u32(value.0 & ElfHeaderFlagRISCVEAbi::MASK).ok_or(
                    Error::InvalidHeaderFlagForMachine {
                        machine: config.machine,
                        value: value.0,
                    },
                )?,
            ));
        }

        if value.0 & ElfHeaderFlagRISCVMemoryModel::MASK != 0 {
            flags.push(ElfHeaderFlagRISCV::MemoryModel(
                ElfHeaderFlagRISCVMemoryModel::from_u32(
                    value.0 & ElfHeaderFlagRISCVMemoryModel::MASK,
                )
                .ok_or(Error::InvalidHeaderFlagForMachine {
                    machine: config.machine,
                    value: value.0,
                })?,
            ));
        }

        Ok(Self { flags, value })
    }
}

impl<const EC: u8, const ED: u8, W> ToWriter<W> for ElfHeaderFlagsRISCV<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        self.value.to_writer(writer)?;
        Ok(())
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionHeaderTypeRISCV {
    /// This section contains RISC-V ELF attributes
    Attributes = Self::ATTRIBUTES,
}

impl ElfSectionHeaderTypeRISCV {
    /// This section contains RISC-V ELF attributes
    pub const ATTRIBUTES: u32 = 0x70000003;
}

impl<const EC: u8, const ED: u8> From<ElfSectionHeaderTypeRISCV> for ElfWord<EC, ED> {
    fn from(value: ElfSectionHeaderTypeRISCV) -> Self {
        Self(value as u32)
    }
}

impl<const EC: u8, const ED: u8> From<&ElfSectionHeaderTypeRISCV> for ElfWord<EC, ED> {
    fn from(value: &ElfSectionHeaderTypeRISCV) -> Self {
        Self(*value as u32)
    }
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfSectionHeaderTypeRISCV {
    type Error = Error;

    fn try_from_with(
        value: ElfWord<EC, ED>,
        config: &mut crate::Config,
    ) -> Result<Self, Self::Error> {
        if !matches!(config.machine, Some(ElfMachine::Riscv)) {
            return Err(Error::InvalidMachineForSectionHeaderType {
                machine: config.machine,
                expected_machines: vec![ElfMachine::Riscv],
                value: value.0,
            });
        }

        if value.0 == Self::ATTRIBUTES {
            Ok(Self::Attributes)
        } else {
            Err(Error::InvalidSectionHeaderType {
                machine: config.machine,
                value: value.0,
            })
        }
    }
}
