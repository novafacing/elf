//! GNU-specific definitions

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Section Header Types
pub enum ElfSectionHeaderTypeGNU {
    /// Incremental build data
    IncrementalInputs = Self::INCREMENTAL_INPUTS,
    /// Object attributes
    Attributes = Self::ATTRIBUTES,
    /// GNU style symbol hash table
    Hash = Self::HASH,
    /// List of prelink dependencies
    LibList = Self::LIBLIST,
    /// Versions defined by file
    VerDef = Self::VERDEF,
    /// Versions needed by file
    VerNeed = Self::VERNEED,
    /// Symbol versions
    VerSym = Self::VERSYM,
}

impl ElfSectionHeaderTypeGNU {
    /// Incremental build data
    pub const INCREMENTAL_INPUTS: u32 = 0x6fff4700;
    /// Object attributes
    pub const ATTRIBUTES: u32 = 0x6ffffff5;
    /// GNU style symbol hash table
    pub const HASH: u32 = 0x6ffffff6;
    /// List of prelink dependencies
    pub const LIBLIST: u32 = 0x6ffffff7;
    /// Versions defined by file
    pub const VERDEF: u32 = 0x6ffffffd;
    /// Versions needed by file
    pub const VERNEED: u32 = 0x6ffffffe;
    /// Symbol versions
    pub const VERSYM: u32 = 0x6fffffff;
}
