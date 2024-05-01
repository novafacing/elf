//! Platform-specific structures for the ARM32 architecture

use std::io::Write;

use crate::{base::ElfWord, error::Error, Config, ToWriter, TryFromWithConfig};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlagARM32 {
    /// Set in executable file headers to note explicitly that the executable
    /// was built to conform to the software floating point procedure call (base)
    /// standard. Note that if `FloatSoft` and `FloatHard` are both not set,
    /// `FloatSoft` is implied.
    FloatSoft = Self::FLOAT_SOFT,
    /// Set in executable file headers to note that the executable file was built to
    /// conform to the hardwaare floating point procedure call standard.
    FloatHard = Self::FLOAT_HARD,
    /// The ELF file contains BE-8 Code, suitable for execution on an arm v6 processor
    Be8 = Self::BE8,
    /// An 8-bit version number, the version of the ABI to which this file conforms. The
    /// current ABI is version 5, a value of 5 denotes unknown conformance.
    AbiVersion {
        /// The ABI version
        version: u8,
    },
    /// Extra flags used by GCC
    Gcc {
        /// Extra flags used by GCC
        flags: u32,
    },
}

impl ElfHeaderFlagARM32 {
    /// Constant value for [ElfHeaderFlagsARM32::FloatSoft]
    pub const FLOAT_SOFT: u32 = 0x00000200;
    /// Constant value for [ElfHeaderFlagsARM32::FloatHard]
    pub const FLOAT_HARD: u32 = 0x00000400;
    /// Constant value for [ElfHeaderFlagsARM32::Be8]
    pub const BE8: u32 = 0x00800000;
    /// Mask for ABI version number
    pub const ABIMASK: u32 = 0xff000000;
    /// Mask for legacy GCC information
    pub const GCCMASK: u32 = 0x00400fff;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A set of semantically useful flags retrieved from the set of flags in the ELF header
pub struct ElfHeaderFlagsARM32<const EC: u8, const ED: u8> {
    flags: Vec<ElfHeaderFlagARM32>,
    value: ElfWord<EC, ED>,
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>>
    for ElfHeaderFlagsARM32<EC, ED>
{
    type Error = Error;

    fn try_from_with(value: ElfWord<EC, ED>, _config: &mut Config) -> Result<Self, Self::Error> {
        let mut flags = Vec::new();

        flags.push(ElfHeaderFlagARM32::AbiVersion {
            version: (((value.0 & ElfHeaderFlagARM32::ABIMASK) >> 24) as u8),
        });

        flags.push(ElfHeaderFlagARM32::Gcc {
            flags: value.0 & ElfHeaderFlagARM32::GCCMASK,
        });

        if value.0 & ElfHeaderFlagARM32::FLOAT_SOFT != 0 {
            flags.push(ElfHeaderFlagARM32::FloatSoft);
        }

        if value.0 & ElfHeaderFlagARM32::FLOAT_HARD != 0 {
            flags.push(ElfHeaderFlagARM32::FloatHard);
        }

        if value.0 & ElfHeaderFlagARM32::BE8 != 0 {
            flags.push(ElfHeaderFlagARM32::Be8);
        }

        Ok(Self { flags, value })
    }
}

impl<const EC: u8, const ED: u8, W> ToWriter<W> for ElfHeaderFlagsARM32<EC, ED>
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
pub enum ElfSectionHeaderTypeARM32 {
    /// Exception Index Table
    ExIdx = Self::EXIDX,
    /// BPABI DLL dynamic linking pre-emption map
    PreemptMap = Self::PREEMPTMAP,
    /// Object file compatibility attributes
    Attributes = Self::ATTRIBUTES,
    /// Debug Overlay
    DebugOverlay = Self::DEBUGOVERLAY,
    /// Overlay Section
    Overlay = Self::OVERLAY,
}

impl ElfSectionHeaderTypeARM32 {
    /// Constant value for [ElfSectionheaderTypeARM32::ExIdx]
    pub const EXIDX: u32 = 0x70000001;
    /// Constant value for [ElfSectionheaderTypeARM32::PreemptMap]
    pub const PREEMPTMAP: u32 = 0x70000002;
    /// Constant value for [ElfSectionheaderTypeARM32::Attributes]
    pub const ATTRIBUTES: u32 = 0x70000003;
    /// Constant value for [ElfSectionheaderTypeARM32::DebugOverlay]
    pub const DEBUGOVERLAY: u32 = 0x70000004;
    /// Constant value for [ElfSectionheaderTypeARM32::Overlay]
    pub const OVERLAY: u32 = 0x70000005;
}

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>> for ElfSectionHeaderTypeARM32 {
    type Error = Error;

    fn try_from_with(
        value: ElfWord<EC, ED>,
        _config: &mut crate::Config,
    ) -> Result<Self, Self::Error> {
        if value.0 == Self::ExIdx as u32 {
            Ok(Self::ExIdx)
        } else if value.0 == Self::PreemptMap as u32 {
            Ok(Self::PreemptMap)
        } else if value.0 == Self::Attributes as u32 {
            Ok(Self::Attributes)
        } else if value.0 == Self::DebugOverlay as u32 {
            Ok(Self::DebugOverlay)
        } else if value.0 == Self::Overlay as u32 {
            Ok(Self::Overlay)
        } else {
            Err(Error::InvalidSectionHeaderTypeARM32 { value: value.0 })
        }
    }
}
