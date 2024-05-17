//! Architecture specific definitions for parisc

use std::io::Write;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;

use crate::{
    base::ElfWord, error::Error, header::elf::ElfMachine, Config, ToWriter, TryFromWithConfig,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagPARISCArchitectureVersion {
    /// PA-RISC 1.0
    PaRisc10 = Self::PARISC_1_0,
    /// PA-RISC 1.1
    PaRisc11 = Self::PARISC_1_1,
    /// PA-RISC 2.0
    PaRisc20 = Self::PARISC_2_0,
}

impl ElfHeaderFlagPARISCArchitectureVersion {
    /// PA-RISC 1.0
    pub const PARISC_1_0: u32 = 0x020b;
    /// PA-RISC 1.1
    pub const PARISC_1_1: u32 = 0x0210;
    /// PA-RISC 2.0
    pub const PARISC_2_0: u32 = 0x0214;
    /// Mask
    pub const MASK: u32 = 0xffff;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagPARISC {
    /// Trap nil pointer dereferences
    TrapNil = Self::TRAP_NIL,
    /// Program uses architecture extensions. If set, there is a .PARISC.archext section
    /// with at least one 32-bit word which identifies the extensions required by the object file:
    ///
    /// 0000 0001 PA 7000 Quadword store instruction
    /// 0000 0002 PA 7100 Floating-point loads and stores to
    ///                  I/O space
    /// 0000 0004 PA 7000 Reciprocal square root instruction
    /// 0000 0008 PA 7000, 7100 FDC instruction includes graphics
    ///                       flushes
    /// 0000 0010 PA 7100LC, 8000 Halfword parallel add/subtract/
    ///                         average instructions
    /// 0000 0020 PA 7100LC, 8000 Halfword parallel shift-and-add
    ///                         instructions
    /// 0000 0040 PA 7100LC Byte-swapping stores
    /// 0000 0080 PA 7200, 8000 Data prefetch via load to GR 0
    Extensions = Self::EXTENSIONS,
    /// Program expects little endian mode
    LittleEndianMode = Self::LITTLE_ENDIAN_MODE,
    /// Program expects wide mode
    WideMode = Self::WIDE_MODE,
    /// Disallow kernel assisted branch prediction
    NoKernelAssistedBranchPrediction = Self::NO_KERNEL_ASSISTED_BRANCH_PREDICTION,
    /// Allow lazy swap for dynamically allocated program segments
    LazySwap = Self::LAZY_SWAP,
    /// Architecture version
    ArchitectureVersion(ElfHeaderFlagPARISCArchitectureVersion),
}

impl ElfHeaderFlagPARISC {
    /// Trap nil pointer dereferences
    pub const TRAP_NIL: u32 = 0x00010000;
    /// Program uses architecture extensions. If set, a .PARISC.archext section is present
    /// with at least one 32-bit word which identifies the extensions required by the object file
    /// (see `ElfHeaderFlagPARISCArchitectureExtension`).
    pub const EXTENSIONS: u32 = 0x00020000;
    /// Program expects little endian mode
    pub const LITTLE_ENDIAN_MODE: u32 = 0x00040000;
    /// Program expects wide mode
    pub const WIDE_MODE: u32 = 0x00080000;
    /// Disallow kernel assisted branch prediction
    pub const NO_KERNEL_ASSISTED_BRANCH_PREDICTION: u32 = 0x00100000;
    /// Allow lazy swap for dynamically allocated program segments
    pub const LAZY_SWAP: u32 = 0x00400000;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A set of semantically useful flags retrieved from the set of flags in the ELF header
pub struct ElfHeaderFlagsPARISC<const EC: u8, const ED: u8> {
    flags: Vec<ElfHeaderFlagPARISC>,
    value: ElfWord<EC, ED>,
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>>
    for ElfHeaderFlagsPARISC<EC, ED>
{
    type Error = Error;

    fn try_from_with(value: ElfWord<EC, ED>, config: &mut Config) -> Result<Self, Self::Error> {
        let mut flags = Vec::new();

        if value.0 & ElfHeaderFlagPARISC::TRAP_NIL != 0 {
            flags.push(ElfHeaderFlagPARISC::TrapNil);
        }

        if value.0 & ElfHeaderFlagPARISC::EXTENSIONS != 0 {
            flags.push(ElfHeaderFlagPARISC::Extensions);
        }

        if value.0 & ElfHeaderFlagPARISC::LITTLE_ENDIAN_MODE != 0 {
            flags.push(ElfHeaderFlagPARISC::LittleEndianMode);
        }

        if value.0 & ElfHeaderFlagPARISC::WIDE_MODE != 0 {
            flags.push(ElfHeaderFlagPARISC::WideMode);
        }

        if value.0 & ElfHeaderFlagPARISC::NO_KERNEL_ASSISTED_BRANCH_PREDICTION != 0 {
            flags.push(ElfHeaderFlagPARISC::NoKernelAssistedBranchPrediction);
        }

        if value.0 & ElfHeaderFlagPARISC::LAZY_SWAP != 0 {
            flags.push(ElfHeaderFlagPARISC::LazySwap);
        }

        if value.0 & ElfHeaderFlagPARISCArchitectureVersion::MASK != 0 {
            flags.push(ElfHeaderFlagPARISC::ArchitectureVersion(
                ElfHeaderFlagPARISCArchitectureVersion::from_u32(
                    value.0 & ElfHeaderFlagPARISCArchitectureVersion::MASK,
                )
                .ok_or(Error::InvalidHeaderFlagForMachine {
                    machine: config.machine,
                    value: value.0,
                })?,
            ))
        }

        Ok(Self { flags, value })
    }
}

impl<const EC: u8, const ED: u8, W> ToWriter<W> for ElfHeaderFlagsPARISC<EC, ED>
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
pub enum ElfSectionHeaderTypePARISC {
    /// Section contains product-specific extension bits
    PariscExt = Self::PARISC_EXT,
    /// Section contains nwind table entries
    PariscUnwind = Self::PARISC_UNWIND,
    /// Section contains debug information for optimized code
    PariscDoc = Self::PARISC_DOC,
    /// Section contains code annotations
    PariscAnnot = Self::PARISC_ANNOT,
    /// DKLM special section
    PariscDlkm = Self::PARISC_DLKM,
    /// Optional section holding argument location/relocation info
    PariscSymextn = Self::PARISC_SYMEXTN,
    /// Option section for linker stubs
    PariscStubs = Self::PARISC_STUBS,
    /// HP specific section
    HpOvlbits = Self::HP_OVLBITS,
    /// HP specific section
    HpDlkm = Self::HP_DLKM,
    /// HP specific section
    HpComdat = Self::HP_COMDAT,
    /// HP specific section
    HpObjdict = Self::HP_OBJDICT,
    /// HP specific section
    HpAnnot = Self::HP_ANNOT,
}

impl ElfSectionHeaderTypePARISC {
    /// Section contains product-specific extension bits
    pub const PARISC_EXT: u32 = 0x70000000;
    /// Section contains nwind table entries
    pub const PARISC_UNWIND: u32 = 0x70000001;
    /// Section contains debug information for optimized code
    pub const PARISC_DOC: u32 = 0x70000002;
    /// Section contains code annotations
    pub const PARISC_ANNOT: u32 = 0x70000003;
    /// DKLM special section
    pub const PARISC_DLKM: u32 = 0x70000004;
    /// Optional section holding argument location/relocation info
    pub const PARISC_SYMEXTN: u32 = 0x70000008;
    /// Option section for linker stubs
    pub const PARISC_STUBS: u32 = 0x70000009;
    /// HP specific section
    pub const HP_OVLBITS: u32 = 0x60000000;
    /// HP specific section
    pub const HP_DLKM: u32 = 0x60000001;
    /// HP specific section
    pub const HP_COMDAT: u32 = 0x60000002;
    /// HP specific section
    pub const HP_OBJDICT: u32 = 0x60000003;
    /// HP specific section
    pub const HP_ANNOT: u32 = 0x60000004;
}

impl<const EC: u8, const ED: u8> From<ElfSectionHeaderTypePARISC> for ElfWord<EC, ED> {
    fn from(value: ElfSectionHeaderTypePARISC) -> Self {
        Self(value as u32)
    }
}

impl<const EC: u8, const ED: u8> From<&ElfSectionHeaderTypePARISC> for ElfWord<EC, ED> {
    fn from(value: &ElfSectionHeaderTypePARISC) -> Self {
        Self(*value as u32)
    }
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfSectionHeaderTypePARISC {
    type Error = Error;

    fn try_from_with(value: ElfWord<EC, ED>, config: &mut Config) -> Result<Self, Self::Error> {
        if !matches!(config.machine, Some(ElfMachine::PARISC)) {
            return Err(Error::InvalidMachineForSectionHeaderType {
                machine: config.machine,
                expected_machines: vec![ElfMachine::PARISC],
                value: value.0,
            });
        }

        match value.0 {
            Self::PARISC_EXT => Ok(Self::PariscExt),
            Self::PARISC_UNWIND => Ok(Self::PariscUnwind),
            Self::PARISC_DOC => Ok(Self::PariscDoc),
            Self::PARISC_ANNOT => Ok(Self::PariscAnnot),
            Self::PARISC_DLKM => Ok(Self::PariscDlkm),
            Self::PARISC_SYMEXTN => Ok(Self::PariscSymextn),
            Self::PARISC_STUBS => Ok(Self::PariscStubs),
            Self::HP_OVLBITS => Ok(Self::HpOvlbits),
            Self::HP_DLKM => Ok(Self::HpDlkm),
            Self::HP_COMDAT => Ok(Self::HpComdat),
            Self::HP_OBJDICT => Ok(Self::HpObjdict),
            Self::HP_ANNOT => Ok(Self::HpAnnot),
            _ => Err(Error::InvalidSectionHeaderType {
                machine: config.machine,
                value: value.0,
            }),
        }
    }
}
