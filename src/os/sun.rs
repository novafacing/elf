//! SUN-specific definitions

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionheaderTypeSUN {
    /// Versions defined by file
    VerDef = Self::VERDEF,
    /// Versions needed by file
    VerNeed = Self::VERNEED,
    /// Symbol versions
    VerSym = Self::VERSYM,
}

impl ElfSectionheaderTypeSUN {
    /// Versions defined by file
    pub const VERDEF: u32 = 0x6ffffffd;
    /// Versions needed by file
    pub const VERNEED: u32 = 0x6ffffffe;
    /// Symbol versions
    pub const VERSYM: u32 = 0x6fffffff;
}
