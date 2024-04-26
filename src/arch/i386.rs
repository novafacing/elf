//! Architecture specific definitions for i386

// NOTE: i386 defines no e_flags values

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionheaderTypeI386 {
    /// Unwind Table
    Unwind = Self::UNWIND,
}

impl ElfSectionheaderTypeI386 {
    /// Constant value for [ElfSectionheaderTypeI386::Unwind]
    pub const UNWIND: u32 = 0x70000001;
}
