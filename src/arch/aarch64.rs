//! Architecture specific definitions for aarch64

// NOTE: aarch64 defines no e_flags values

use num_derive::FromPrimitive;

use crate::{base::ElfWord, error::Error, TryFromWithConfig};

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

impl<const EC: u8, const ED: u8> TryFromWithConfig<ElfWord<EC, ED>>
    for ElfSectionHeaderTypeAARCH64
{
    type Error = Error;

    fn try_from_with(
        value: ElfWord<EC, ED>,
        _config: &mut crate::Config,
    ) -> Result<Self, Self::Error> {
        if value.0 == Self::Attributes as u32 {
            Ok(Self::Attributes)
        } else {
            Err(Error::InvalidSectionHeaderTypeAARCH64 { value: value.0 })
        }
    }
}
