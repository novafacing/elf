//! Architecture specific definitions for aarch64

// NOTE: aarch64 defines no e_flags values

use num_derive::FromPrimitive;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
#[non_exhaustive]
/// Type flags for an ELF Section Header, which may contain processor and OS-specific
/// flags.
pub enum ElfSectionHeaderTypeAARCH64 {
    /// Reserved for Object file compatibility attributes
    Attributes = Self::ATTRIBUTES,
}

impl ElfSectionHeaderTypeAARCH64 {
    /// Constant value for [ElfSectionHeaderTypeAARCH64::Attributes]
    pub const ATTRIBUTES: u32 = 0x70000003;
}
