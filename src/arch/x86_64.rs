//! Architecture specific definitions for x86_64

// NOTE: x86_64 defines no e_flags values

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionheaderTypeX86_64 {
    /// Unwind Table
    Unwind = Self::UNWIND,
}

impl ElfSectionheaderTypeX86_64 {
    /// Constant value for [ElfSectionheaderTypeI386::Unwind]
    pub const UNWIND: u32 = 0x70000001;
}
