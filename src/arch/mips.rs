//! Architecture specific definitions for mips

use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;

use crate::{base::ElfWord, error::Error, Config, TryFromWithConfig};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagMIPSArchitecture {
    /// MIPS1 code
    Mips1 = Self::MIPS_ARCH_1,
    /// MIPS2 code
    Mips2 = Self::MIPS_ARCH_2,
    /// MIPS3 code
    Mips3 = Self::MIPS_ARCH_3,
    /// MIPS4 code
    Mips4 = Self::MIPS_ARCH_4,
    /// MIPS5 code
    Mips5 = Self::MIPS_ARCH_5,
    /// MIPS32 code
    Mips32 = Self::MIPS_ARCH_32,
    /// MIPS64 code
    Mips64 = Self::MIPS_ARCH_64,
    /// MIPS32r2 code
    Mips32R2 = Self::MIPS_ARCH_32_R2,
    /// MIPS64r2 code
    Mips64R2 = Self::MIPS_ARCH_64_R2,
    /// MIPS32r2 code
    Mips32R6 = Self::MIPS_ARCH_32_R6,
    /// MIPS64r2 code
    Mips64R6 = Self::MIPS_ARCH_64_R6,
}

impl ElfHeaderFlagMIPSArchitecture {
    /// MIPS1 code
    pub const MIPS_ARCH_1: u32 = 0x00000000;
    /// MIPS2 code
    pub const MIPS_ARCH_2: u32 = 0x10000000;
    /// MIPS3 code
    pub const MIPS_ARCH_3: u32 = 0x20000000;
    /// MIPS4 code
    pub const MIPS_ARCH_4: u32 = 0x30000000;
    /// MIPS5 code
    pub const MIPS_ARCH_5: u32 = 0x40000000;
    /// MIPS32 code
    pub const MIPS_ARCH_32: u32 = 0x50000000;
    /// MIPS64 code
    pub const MIPS_ARCH_64: u32 = 0x60000000;
    /// MIPS32r2 code
    pub const MIPS_ARCH_32_R2: u32 = 0x70000000;
    /// MIPS64r2 code
    pub const MIPS_ARCH_64_R2: u32 = 0x80000000;
    /// MIPS32r6 code
    pub const MIPS_ARCH_32_R6: u32 = 0x90000000;
    /// MIPS64r6 code
    pub const MIPS_ARCH_64_R6: u32 = 0xa0000000;
    /// Mask
    pub const MASK: u32 = 0xf0000000;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagMIPSArchitectureExtension {
    /// Use MDMX multimedia extensions
    Mdmx = Self::MDMX,
    /// Use MIPS-16 ISA exctensions
    Mips16 = Self::MIPS16,
    /// Use MICROMIPS ISA extensions
    Micromips = Self::MICROMIPS,
}

impl ElfHeaderFlagMIPSArchitectureExtension {
    /// Use MDMX multimedia extensions
    pub const MDMX: u32 = 0x08000000;
    /// Use MIPS-16 ISA exctensions
    pub const MIPS16: u32 = 0x04000000;
    /// Use MICROMIPS ISA extensions
    pub const MICROMIPS: u32 = 0x02000000;
    /// Mask
    pub const MASK: u32 = 0x0f000000;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagMIPSABI {
    /// O32 ABI
    O32 = Self::ABI_O32,
    /// O32 extended for 64-bit
    O64 = Self::ABI_O64,
    /// EABI in 32-bit mode
    EABI32 = Self::ABI_EABI32,
    /// EABI in 64-bit mode
    EABI64 = Self::ABI_EABI64,
}

impl ElfHeaderFlagMIPSABI {
    /// O32 ABI
    pub const ABI_O32: u32 = 0x00001000;
    /// O32 extended for 64-bit
    pub const ABI_O64: u32 = 0x00002000;
    /// EABI in 32-bit mode
    pub const ABI_EABI32: u32 = 0x00003000;
    /// EABI in 64-bit mode
    pub const ABI_EABI64: u32 = 0x00004000;
    /// Mask
    pub const MASK: u32 = 0x0000f000;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagMIPSMachine {
    /// 3900
    Machine3900 = Self::MACHINE_3900,
    /// 4010
    Machine4010 = Self::MACHINE_4010,
    /// 4100
    Machine4100 = Self::MACHINE_4100,
    /// ALLEGREX
    MachineALLEGREX = Self::MACHINE_ALLEGREX,
    /// 4650
    Machine4650 = Self::MACHINE_4650,
    /// 4120
    Machine4120 = Self::MACHINE_4120,
    /// 4111
    Machine4111 = Self::MACHINE_4111,
    /// SB1
    MachineSB1 = Self::MACHINE_SB1,
    /// OCTEON
    MachineOCTEON = Self::MACHINE_OCTEON,
    /// XLR
    MachineXLR = Self::MACHINE_XLR,
    /// OCTEON2
    MachineOCTEON2 = Self::MACHINE_OCTEON2,
    /// OCTEON3
    MachineOCTEON3 = Self::MACHINE_OCTEON3,
    /// 5400
    Machine5400 = Self::MACHINE_5400,
    /// 5900
    Machine5900 = Self::MACHINE_5900,
    /// IAMR2
    MachineIAMR2 = Self::MACHINE_IAMR2,
    /// 5500
    Machine5500 = Self::MACHINE_5500,
    /// 9000
    Machine9000 = Self::MACHINE_9000,
    /// LS2E
    MachineLS2E = Self::MACHINE_LS2E,
    /// LS2F
    MachineLS2F = Self::MACHINE_LS2F,
    /// GS464
    MachineGS464 = Self::MACHINE_GS464,
    /// GS464E
    MachineGS464E = Self::MACHINE_GS464E,
    /// GS264E
    MachineGS264E = Self::MACHINE_GS264E,
}

impl ElfHeaderFlagMIPSMachine {
    /// 3900
    pub const MACHINE_3900: u32 = 0x00810000;
    /// 4010
    pub const MACHINE_4010: u32 = 0x00820000;
    /// 4100
    pub const MACHINE_4100: u32 = 0x00830000;
    /// ALLEGREX
    pub const MACHINE_ALLEGREX: u32 = 0x00840000;
    /// 4650
    pub const MACHINE_4650: u32 = 0x00850000;
    /// 4120
    pub const MACHINE_4120: u32 = 0x00870000;
    /// 4111
    pub const MACHINE_4111: u32 = 0x00880000;
    /// SB1
    pub const MACHINE_SB1: u32 = 0x008a0000;
    /// OCTEON
    pub const MACHINE_OCTEON: u32 = 0x008b0000;
    /// XLR
    pub const MACHINE_XLR: u32 = 0x008c0000;
    /// OCTEON2
    pub const MACHINE_OCTEON2: u32 = 0x008d0000;
    /// OCTEON3
    pub const MACHINE_OCTEON3: u32 = 0x008e0000;
    /// 5400
    pub const MACHINE_5400: u32 = 0x00910000;
    /// 5900
    pub const MACHINE_5900: u32 = 0x00920000;
    /// IAMR2
    pub const MACHINE_IAMR2: u32 = 0x00930000;
    /// 5500
    pub const MACHINE_5500: u32 = 0x00980000;
    /// 9000
    pub const MACHINE_9000: u32 = 0x00990000;
    /// LS2E
    pub const MACHINE_LS2E: u32 = 0x00a00000;
    /// LS2F
    pub const MACHINE_LS2F: u32 = 0x00a10000;
    /// GS464
    pub const MACHINE_GS464: u32 = 0x00a20000;
    /// GS464E
    pub const MACHINE_GS464E: u32 = 0x00a30000;
    /// GS264E
    pub const MACHINE_GS264E: u32 = 0x00a40000;
    /// MASK Value
    pub const MACHINE_MASK: u32 = 0x00ff0000;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagMIPS {
    /// No reordering (.noreorder) was used
    NoReorder = Self::NO_REORDER,
    /// Position independent code
    Pic = Self::PIC,
    /// PIC Calling sequence
    CPic = Self::CPIC,
    /// Extended global offset table
    XGot = Self::XGOT,
    /// Stanford Ucode
    UCode = Self::UCODE,
    /// ABI 2
    Abi2 = Self::ABI2,
    /// ABI O/N 32
    AbiOn32 = Self::ABI_ON32,
    /// .MIPS.options section processed first by LD
    OptionsFirst = Self::OPTIONS_FIRST,
    /// Code compiled for a 64-bit machine in 32-bit mode (32-bit regs)
    BitMode32 = Self::BITMODE_32,
    /// Uses fp64 (12 callee-saved) abi
    FloatingPoint64 = Self::FP64,
    /// Uses IEEE 754-2008 NaN encoding
    NotANumber2008 = Self::NAN_2008,
    /// Architecture
    Architecture(ElfHeaderFlagMIPSArchitecture),
    /// Architecture Extensions
    Extension(ElfHeaderFlagMIPSArchitectureExtension),
    /// ABI
    Abi(ElfHeaderFlagMIPSABI),
    /// Machine
    Machine(ElfHeaderFlagMIPSMachine),
}

impl ElfHeaderFlagMIPS {
    /// Do not reorder
    pub const NO_REORDER: u32 = 1;
    /// Use PIC
    pub const PIC: u32 = 2;
    /// PIC calling sequence
    pub const CPIC: u32 = 4;
    /// Extended GOT
    pub const XGOT: u32 = 8;
    /// Stanford Ucode
    pub const UCODE: u32 = 16;
    /// ABI 2
    pub const ABI2: u32 = 32;
    /// ABI O/N 32
    pub const ABI_ON32: u32 = 64;
    /// Options first
    pub const OPTIONS_FIRST: u32 = 0x80;
    /// 32-bit mode
    pub const BITMODE_32: u32 = 0x100;
    /// Floating point 64
    pub const FP64: u32 = 512;
    /// Use IEEE 754 2008 NaN encoding
    pub const NAN_2008: u32 = 1024;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A set of semantically useful flags retrieved from the set of flags in the ELF header
pub struct ElfHeaderFlagsMIPS<const EC: u8, const ED: u8> {
    flags: Vec<ElfHeaderFlagMIPS>,
    value: ElfWord<EC, ED>,
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfHeaderFlagsMIPS<EC, ED> {
    type Error = Error;

    fn try_from_with(value: ElfWord<EC, ED>, _config: &mut Config) -> Result<Self, Self::Error> {
        let mut flags = Vec::new();

        // Base flags
        if value.0 & ElfHeaderFlagMIPS::NO_REORDER != 0 {
            flags.push(ElfHeaderFlagMIPS::NoReorder);
        }

        if value.0 & ElfHeaderFlagMIPS::PIC != 0 {
            flags.push(ElfHeaderFlagMIPS::Pic);
        }

        if value.0 & ElfHeaderFlagMIPS::CPIC != 0 {
            flags.push(ElfHeaderFlagMIPS::CPic);
        }

        if value.0 & ElfHeaderFlagMIPS::XGOT != 0 {
            flags.push(ElfHeaderFlagMIPS::XGot);
        }

        if value.0 & ElfHeaderFlagMIPS::UCODE != 0 {
            flags.push(ElfHeaderFlagMIPS::UCode);
        }

        if value.0 & ElfHeaderFlagMIPS::ABI2 != 0 {
            flags.push(ElfHeaderFlagMIPS::Abi2);
        }

        if value.0 & ElfHeaderFlagMIPS::ABI_ON32 != 0 {
            flags.push(ElfHeaderFlagMIPS::AbiOn32);
        }

        if value.0 & ElfHeaderFlagMIPS::OPTIONS_FIRST != 0 {
            flags.push(ElfHeaderFlagMIPS::OptionsFirst);
        }

        if value.0 & ElfHeaderFlagMIPS::BITMODE_32 != 0 {
            flags.push(ElfHeaderFlagMIPS::BitMode32);
        }

        if value.0 & ElfHeaderFlagMIPS::FP64 != 0 {
            flags.push(ElfHeaderFlagMIPS::FloatingPoint64);
        }

        if value.0 & ElfHeaderFlagMIPS::NAN_2008 != 0 {
            flags.push(ElfHeaderFlagMIPS::NotANumber2008);
        }

        if value.0 & ElfHeaderFlagMIPSArchitecture::MASK != 0 {
            flags.push(ElfHeaderFlagMIPS::Architecture(
                ElfHeaderFlagMIPSArchitecture::from_u32(
                    value.0 & ElfHeaderFlagMIPSArchitecture::MASK,
                )
                .ok_or(Error::InvalidHeaderFlagMipsArchitecture { value: value.0 })?,
            ))
        }

        if value.0 & ElfHeaderFlagMIPSArchitectureExtension::MASK != 0 {
            flags.push(ElfHeaderFlagMIPS::Extension(
                ElfHeaderFlagMIPSArchitectureExtension::from_u32(
                    value.0 & ElfHeaderFlagMIPSArchitectureExtension::MASK,
                )
                .ok_or(Error::InvalidHeaderFlagMipsArchitectureExtension { value: value.0 })?,
            ))
        }

        if value.0 & ElfHeaderFlagMIPSABI::MASK != 0 {
            flags.push(ElfHeaderFlagMIPS::Abi(
                ElfHeaderFlagMIPSABI::from_u32(value.0 & ElfHeaderFlagMIPSABI::MASK)
                    .ok_or(Error::InvalidHeaderFlagMipsAbi { value: value.0 })?,
            ))
        }

        if value.0 & ElfHeaderFlagMIPSMachine::MACHINE_MASK != 0 {
            flags.push(ElfHeaderFlagMIPS::Machine(
                ElfHeaderFlagMIPSMachine::from_u32(
                    value.0 & ElfHeaderFlagMIPSMachine::MACHINE_MASK,
                )
                .ok_or(Error::InvalidHeaderFlagMipsMachine { value: value.0 })?,
            ));
        }

        Ok(Self { flags, value })
    }
}
