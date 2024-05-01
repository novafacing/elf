//! Architecture specific definitions for PowerPC

// NOTE: No architecture-specific ELF Header flags for PPC

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
