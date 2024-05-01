//! Implementation of the ELF section header

use num_traits::FromPrimitive;
use std::{
    io::{Read, Seek, Write},
    mem::size_of,
};
use typed_builder::TypedBuilder;

use crate::{
    arch::{
        aarch64::ElfSectionHeaderTypeAARCH64, arm32::ElfSectionHeaderTypeARM32,
        i386::ElfSectionHeaderTypeI386, mips::ElfSectionHeaderTypeMIPS,
        parisc::ElfSectionHeaderTypePARISC, ppc::ElfSectionHeaderTypePPC,
        riscv::ElfSectionHeaderTypeRISCV, x86_64::ElfSectionHeaderTypeX86_64,
    },
    base::{ElfAddress, ElfExtendedWord, ElfOffset, ElfWord},
    error::{Error, ErrorContext},
    from_primitive,
    os::{gnu::ElfSectionHeaderTypeGNU, sun::ElfSectionHeaderTypeSUN},
    Config, FromReader, HasWrittenSize, ToWriter,
};

use super::elf::identification::ElfClass;

#[derive(Debug, Clone, PartialEq, Eq)]
/// The name of an ELF section
pub struct ElfSectionHeaderName<const ED: u8> {
    /// The name of the section, which is obtained by indexing into the section header
    /// table string table
    pub name: String,
    /// The raw section header name
    pub value: ElfWord<{ ElfClass::Elf32 as u8 }, ED>,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// The type of an ELF section
enum ElfSectionHeaderType<const EC: u8, const ED: u8> {
    /// Marks the section header as inactive; it does not have an associated
    /// section. Other members of the section header have undefined values.
    NullUndefined = 0,
    /// Holds information deﬁned by the program, whose format and meaning are
    /// determined solely by the program.
    ProgramBits = 1,
    /// Hold a symbol table. Currently, an object file may have only one section of
    /// each type (`SymbolTable` and `DynamicSymbol`), but this restriction may be
    /// relaxed in the future.  Typically, SHT_SYMTAB provides symbols for link
    /// editing, though it may also be used for dynamic linking. As a complete
    /// symbol table, it may contain many symbols unnecessary for dynamic linking.
    /// Consequently, an object file may also contain a SHT_DYNSYM section, which
    /// holds a minimal set of dynamic linking symbols, to save space.
    ///
    ///
    SymbolTable = 2,
    /// The section holds a string table. An object ﬁle may have multiple
    /// string table sections. See ``String Table'' below for details.
    StringTable = 3,
    /// Holds relocation entries with explicit addends, such as type Elf32_Rela for
    /// the 32-bit class of object ﬁles or type Elf64_Rela for the 64-bit class of
    /// object ﬁles. An object ﬁle may have multiple relocation sections.
    /// ``Relocation'' below for details.
    RelocationExplicit = 4,
    /// Holds a symbol hash table. Currently, an object ﬁle may have only one hash
    /// table, but this restriction may be relaxed in the future. See ``Hash Table''
    /// in the Chapter 5 for details.
    Hash = 5,
    /// The section holds information for dynamic linking. Currently, an object ﬁle
    /// may have only one dynamic section, but this restriction may be relaxed in
    /// the future. See ``Dynamic Section'' in Chapter 5 for details.
    Dynamic = 6,
    /// The section holds information that marks the ﬁle in some way. See ``Note
    /// Section'' in Chapter 5 for details.
    Note = 7,
    /// A section of this type occupies no space in the ﬁle but otherwise resembles
    /// SHT_PROGBITS. Although this section contains no bytes, the sh_oﬀset member
    /// contains the conceptual ﬁle oﬀset.
    NoBits = 8,
    /// The section holds relocation entries without explicit addends, such as type
    /// Elf32_Rel for the 32-bit class of object ﬁles or type Elf64_Rel for the
    /// 64-bit class of object ﬁles. An object ﬁle may have multiple relocation
    /// sections. See ``Relocation'' below for details.
    RelocationImplicit = 9,
    /// This section type is reserved but has unspecified semantics
    SectionHeaderLibrary = 10,
    /// Hold a symbol table. Currently, an object file may have only one section of
    /// each type, but this restriction may be relaxed in the future.  Typically,
    /// SHT_SYMTAB provides symbols for link editing, though it may also be used for
    /// dynamic linking. As a complete symbol table, it may contain many symbols
    /// unnecessary for dynamic linking.  Consequently, an object file may also
    /// contain a SHT_DYNSYM section, which holds a minimal set of dynamic linking
    /// symbols, to save space.  See ``Symbol Table'' below for details.
    DynamicSymbol = 11,
    /// This section contains an array of pointers to initialization functions, as
    /// described in ``Initialization and Termination Functions'' in Chapter 5. Each
    /// pointer in the array is taken as a parameterless procedure with a void
    /// return.
    InitializerArray = 14,
    /// This section contains an array of pointers to termination functions, as
    /// described in ``Initialization and Termination Functions'' in Chapter 5.
    /// Each pointer in the array is taken as a parameterless procedure with a void
    /// return
    FinalizerArray = 15,
    /// This section contains an array of pointers to functions that are invoked
    /// before all other initialization functions, as described in ``Initialization
    /// and Termination Functions'' in Chapter 5. Each pointer in the array is taken
    /// as a parameterless procedure with a void return.
    PreInitializerArray = 16,
    /// This section deﬁnes a section group. A section group is a set of sections
    /// that are related and that must be treated specially by the linker (see below
    /// for further details). Sections of type SHT_GROUP may appear only in
    /// relocatable objects (objects with the ELF header e_type member set to
    /// ET_REL). The section header table entry for a group section must appear in
    /// the section header table before the entries for any of the sections that are
    /// members of the group.
    Group = 17,
    /// This section is associated with a symbol table section and is required if
    /// any of the section header indexes referenced by that symbol table contain
    /// the escape value SHN_XINDEX. The section is an array of Elf32_Word values.
    /// Each value corresponds one to one with a symbol table entry and appear in
    /// the same order as those entries. The values represent the section header
    /// indexes against which the symbol table entries are deﬁned. Only if the
    /// corresponding symbol table entry's st_shndx ﬁeld contains the escape value
    /// SHN_XINDEX will the matching Elf32_Word hold the actual section header
    /// index; otherwise, the entry must be SHN_UNDEF (0).
    SymbolTableSectionHeaderIndex = 18,
    /// RELR Relative Relocations
    RelR = 19,
    // /// Values in this inclusive range are reserved for operating system- speciﬁc
    // /// semantics.
    // LowOperatingSystem = 0x60000000,
    // /// Values in this inclusive range are reserved for operating system-
    // /// speciﬁc semantics.
    // HighOperatingSystem = 0x6fffffff,
    // /// Values in this inclusive range are reserved for processor specific
    // /// speciﬁc semantics.
    // LowProcessorSpecific = 0x70000000,
    // /// Values in this inclusive range are reserved for processor specific
    // /// speciﬁc semantics.
    // HighProcessorSpecific = 0x7fffffff,
    // /// Values in this inclusive range are resserved for application programs
    // LowUserDefined = 0x80000000,
    // // /// Values in this inclusive range are resserved for application programs
    // // HighUserDefined = 0xffffffff
    /// AARCH64-specific
    AARCH64(ElfSectionHeaderTypeAARCH64),
    /// ARM-specific
    ARM(ElfSectionHeaderTypeARM32),
    /// I386-specific
    I386(ElfSectionHeaderTypeI386),
    /// MIPS-specific
    MIPS(ElfSectionHeaderTypeMIPS),
    /// PA-RISC-specific
    PARISC(ElfSectionHeaderTypePARISC),
    /// PPC-specific
    PPC(ElfSectionHeaderTypePPC),
    /// RISC-V-specific
    RISCV(ElfSectionHeaderTypeRISCV),
    /// X86_64-Specific
    X86_64(ElfSectionHeaderTypeX86_64),
    /// GNU-Specific
    GNU(ElfSectionHeaderTypeGNU),
    /// SUN-Specific
    SUN(ElfSectionHeaderTypeSUN),
}

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfSectionHeaderType<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let r#type = ElfWord::<EC, ED>::from_reader_with(reader, config)?;

        if let Some(r#type) = match r#type.0 {} {
            Ok(r#type)
        } else {
            Err(Error::InvalidElfSectionHeaderType {
                context: ErrorContext::from_reader(reader, size_of::<ElfWord<EC, ED>>())
                    .map_err(Error::from)?,
            })
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfSectionHeaderType<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfWord::<EC, ED>((*self as u32).to_le()).to_writer(writer)
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfSectionHeaderType<EC, ED> {
    const SIZE: usize = size_of::<ElfWord<EC, ED>>();
}

from_primitive! {
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[non_exhaustive]
    /// The flags of an ELF section
    enum Elf32SectionHeaderFlags<const ED: u8> {
        /// The section contains data that should be writable during process execution
        Write = 0x1,
        /// The section occupies memory during process execution.  Some control sections
        /// do not reside in the memory image of an object file; this attribute is off
        /// for those sections.
        Allocated = 0x2,
        /// Contains executable machine instructions
        ExecutableInstructions = 0x4,
        /// The data in the section may be merged to eliminate duplication.  Unless the
        /// SHF_STRINGS flag is also set, the data elements in the section are of a
        /// uniform size. The size of each element is specified in the section header's
        /// sh_entsize field. If the SHF_STRINGS flag is also set, the data elements
        /// consist of null-terminated character strings. The size of each character is
        /// specified in the section header's sh_entsize field.  Each element in the
        /// section is compared against other elements in sections with the same name,
        /// type and flags. Elements that would have identical values at program
        /// run-time may be merged.  Relocations referencing elements of such sections
        /// must be resolved to the merged locations of the referenced values. Note that
        /// any relocatable values, including values that would result in run-time
        /// relocations, must be analyzed to determine whether the run-time values would
        /// actually be identical. An ABI-conforming object file may not depend on
        /// specific elements being merged, and an ABI- conforming link editor may
        /// choose not to merge specific elements.
        Merge = 0x10,
        /// The data elements in the section consist of null-terminated character
        /// strings. The size of each character is specified in the section header's
        /// sh_entsize field.
        Strings = 0x20,
        /// The sh_info field of this section header holds a section header table
        /// index.
        InfoLink = 0x40,
        /// This flag adds special ordering requirements for link editors. The
        /// requirements apply if the sh_link field of this section's header references
        /// another section (the linked-to section). If this section is combined with
        /// other sections in the output file, it must appear in the same relative order
        /// with respect to those sections, as the linked-to section appears with
        /// respect to sections the linked-to section is combined with.
        ///
        /// A typical use of this flag is to build a table that references text
        /// or data sections in address order.
        LinkOrder = 0x80,
        /// This section requires special OS-specific processing (beyond the standard
        /// linking rules) to avoid incorrect behavior. If this section has either an
        /// sh_type value or contains sh_flags bits in the OS-specific ranges for those
        /// fields, and a link editor processing this section does not recognize those
        /// values, then the link editor should reject the object file containing this
        /// section with an error.
        OsNonConforming = 0x100,
        /// This section is a member (perhaps the only one) of a section group.  The
        /// section must be referenced by a section of type SHT_GROUP. The SHF_GROUP flag
        /// may be set only for sections contained in relocatable objects (objects with
        /// the ELF header e_type member set to ET_REL). See below for further details.
        Group = 0x200,
        /// This section holds Thread-Local Storage, meaning that each separate
        /// execution flow has its own distinct instance of this data.  Implementations
        /// need not support this flag.
        ThreadLocalStorage = 0x400,
        /// This flag identifies a section containing compressed data.
        /// SHF_COMPRESSED applies only to non-allocable sections, and cannot
        /// be used in conjunction with SHF_ALLOC. In addition,
        /// SHF_COMPRESSED cannot be applied to sections of type SHT_NOBITS.
        /// All relocations to a compressed section specifiy oﬀsets to the
        /// uncompressed section data. It is therefore necessary to decompress
        /// the section data before relocations can be applied. Each compressed
        /// section specifies the algorithm independently. It is permissible for
        /// diﬀerent sections in a given ELF object to employ diﬀerent
        /// compression algorithms.
        /// Compressed sections begin with a compression header structure that
        /// identifies the compression algorithm.
        Compressed = 0x800,
        // Maskos = 0x0ff00000
        // Maskproc = 0xf0000000
    }
}

impl<R, const ED: u8> FromReader<R> for Elf32SectionHeaderFlags<ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let flags = ElfWord::<{ ElfClass::Elf32 as u8 }, ED>::from_reader_with(reader, config)?;

        if let Some(flags) = Self::from_u32(flags.0) {
            Ok(flags)
        } else {
            Err(Error::InvalidElfSectionHeaderFlags {
                context: ErrorContext::from_reader(
                    reader,
                    size_of::<ElfWord<{ ElfClass::Elf32 as u8 }, ED>>(),
                )
                .map_err(Error::from)?,
            })
        }
    }
}

impl<W, const ED: u8> ToWriter<W> for Elf32SectionHeaderFlags<ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfWord::<{ ElfClass::Elf32 as u8 }, ED>((*self as u32).to_le()).to_writer(writer)
    }
}

impl<const ED: u8> HasWrittenSize for Elf32SectionHeaderFlags<ED> {
    const SIZE: usize = size_of::<ElfWord<{ ElfClass::Elf32 as u8 }, ED>>();
}

from_primitive! {
    #[repr(u64)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[non_exhaustive]
    /// The flags of an ELF section
    enum Elf64SectionHeaderFlags<const ED: u8> {
        /// The section contains data that should be writable during process execution
        Write = 0x1,
        /// The section occupies memory during process execution.  Some control sections
        /// do not reside in the memory image of an object file; this attribute is off
        /// for those sections.
        Allocated = 0x2,
        /// Contains executable machine instructions
        ExecutableInstructions = 0x4,
        /// The data in the section may be merged to eliminate duplication.  Unless the
        /// SHF_STRINGS flag is also set, the data elements in the section are of a
        /// uniform size. The size of each element is specified in the section header's
        /// sh_entsize field. If the SHF_STRINGS flag is also set, the data elements
        /// consist of null-terminated character strings. The size of each character is
        /// specified in the section header's sh_entsize field.  Each element in the
        /// section is compared against other elements in sections with the same name,
        /// type and flags. Elements that would have identical values at program
        /// run-time may be merged.  Relocations referencing elements of such sections
        /// must be resolved to the merged locations of the referenced values. Note that
        /// any relocatable values, including values that would result in run-time
        /// relocations, must be analyzed to determine whether the run-time values would
        /// actually be identical. An ABI-conforming object file may not depend on
        /// specific elements being merged, and an ABI- conforming link editor may
        /// choose not to merge specific elements.
        Merge = 0x10,
        /// The data elements in the section consist of null-terminated character
        /// strings. The size of each character is specified in the section header's
        /// sh_entsize field.
        Strings = 0x20,
        /// The sh_info field of this section header holds a section header table
        /// index.
        InfoLink = 0x40,
        /// This flag adds special ordering requirements for link editors. The
        /// requirements apply if the sh_link field of this section's header references
        /// another section (the linked-to section). If this section is combined with
        /// other sections in the output file, it must appear in the same relative order
        /// with respect to those sections, as the linked-to section appears with
        /// respect to sections the linked-to section is combined with.
        ///
        /// A typical use of this flag is to build a table that references text
        /// or data sections in address order.
        LinkOrder = 0x80,
        /// This section requires special OS-specific processing (beyond the standard
        /// linking rules) to avoid incorrect behavior. If this section has either an
        /// sh_type value or contains sh_flags bits in the OS-specific ranges for those
        /// fields, and a link editor processing this section does not recognize those
        /// values, then the link editor should reject the object file containing this
        /// section with an error.
        OsNonConforming = 0x100,
        /// This section is a member (perhaps the only one) of a section group.  The
        /// section must be referenced by a section of type SHT_GROUP. The SHF_GROUP flag
        /// may be set only for sections contained in relocatable objects (objects with
        /// the ELF header e_type member set to ET_REL). See below for further details.
        Group = 0x200,
        /// This section holds Thread-Local Storage, meaning that each separate
        /// execution flow has its own distinct instance of this data.  Implementations
        /// need not support this flag.
        ThreadLocalStorage = 0x400,
        /// This flag identifies a section containing compressed data.
        /// SHF_COMPRESSED applies only to non-allocable sections, and cannot
        /// be used in conjunction with SHF_ALLOC. In addition,
        /// SHF_COMPRESSED cannot be applied to sections of type SHT_NOBITS.
        /// All relocations to a compressed section specifiy oﬀsets to the
        /// uncompressed section data. It is therefore necessary to decompress
        /// the section data before relocations can be applied. Each compressed
        /// section specifies the algorithm independently. It is permissible for
        /// diﬀerent sections in a given ELF object to employ diﬀerent
        /// compression algorithms.
        /// Compressed sections begin with a compression header structure that
        /// identifies the compression algorithm.
        Compressed = 0x800,
        // Maskos = 0x0ff00000
        // Maskproc = 0xf0000000
    }
}

impl<R, const ED: u8> FromReader<R> for Elf64SectionHeaderFlags<ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let flags =
            ElfExtendedWord::<{ ElfClass::Elf64 as u8 }, ED>::from_reader_with(reader, config)?;

        if let Some(flags) = Self::from_u64(flags.0) {
            Ok(flags)
        } else {
            Err(Error::InvalidElfSectionHeaderFlags {
                context: ErrorContext::from_reader(
                    reader,
                    size_of::<ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>>(),
                )
                .map_err(Error::from)?,
            })
        }
    }
}

impl<W, const ED: u8> ToWriter<W> for Elf64SectionHeaderFlags<ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfExtendedWord::<{ ElfClass::Elf64 as u8 }, ED>((*self as u64).to_le()).to_writer(writer)
    }
}

impl<const ED: u8> HasWrittenSize for Elf64SectionHeaderFlags<ED> {
    const SIZE: usize = size_of::<ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>>();
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
/// ELF Section Header
pub struct Elf32SectionHeader<const ED: u8> {
    /// The name of the section. Its value is an index into the section header string
    /// table section giving the location of a null-terminated string
    name: ElfSectionHeaderName<ED>,
    /// The section's contents and semantics
    r#type: ElfSectionHeaderType<{ ElfClass::Elf32 as u8 }, ED>,
    /// Bit-flags that describe miscellaneous attributes
    flags: Elf32SectionHeaderFlags<ED>,
    /// If the section will appear in the memory image of a process, this member gives
    /// the address at which the section's first byte should reside. Otherwise, the
    /// member contains 0.
    address: Option<ElfAddress<{ ElfClass::Elf32 as u8 }, ED>>,
    /// This member's value gives the byte offset from the beginning of the file to the
    /// first byte in the section. One section type, SHT_NOBITS described below, occupies
    /// no space in the file, and its sh_offset member locates the conceptual placement in
    /// the file.
    offset: ElfOffset<{ ElfClass::Elf32 as u8 }, ED>,
    /// This member gives the section's size in bytes. Unless the section type is
    /// SHT_NOBITS, the section occupies sh_size bytes in the file. A section of type
    /// SHT_NOBITS may have a non-zero size, but it occupies no space in the file.
    size: ElfWord<{ ElfClass::Elf32 as u8 }, ED>,
    /// This member holds a section header table index link, whose interpretation
    /// depends on the section type. A table below describes the values.
    ///
    /// * SHT_DYNAMIC:
    ///     * link: The section header index of the string table used by entries
    ///       in the section
    ///     * info: 0
    /// * SHT_HASH:
    ///     * link: The section header index of the symbol table to which the
    ///       hash table applies
    ///     * info: 0
    /// * SHT_REL/SHT_RELA:
    ///     * link: The section header index of the associated symbol table
    ///     * info: The section header index of the section to which the relocation
    ///       applies
    /// * SHT_SYMTAB/SHT_DYNSYM:
    ///     * link: The section header index of the associated string table
    ///     * info: One greater than the symbol table index of the last local symbol
    ///       (binding STB_LOCAL)
    /// * SHT_GROUP:
    ///     * link: The section header index of the associated symbol table
    ///     * info: The symbol table index of an entry in the associated symbol table.
    ///       The name of the specified symbol table entry provides a signature for the
    ///       section group.
    /// * SHT_SYMTAB_SHNDX:
    ///     * link: The section header index of the associated symbol table section
    ///     * info: 0
    link: ElfWord<{ ElfClass::Elf32 as u8 }, ED>,
    /// This member holds extra information, whose interpretation depends on the section
    /// type. See `link` for the table describing meanings. If the sh_flags field for
    /// this section header includes the attribute SHF_INFO_LINK, then this member
    /// represents a section header table index.
    info: ElfWord<{ ElfClass::Elf32 as u8 }, ED>,
    /// Some sections have address alignment constraints. For example, if a section
    /// holds a doubleword, the system must ensure doubleword alignment for the entire
    /// section. The value of sh_addr must be congruent to 0, modulo the value of
    /// sh_addralign. Currently, only 0 and positive integral powers of two are allowed.
    /// Values 0 and 1 mean the section has no alignment constraints.
    address_align: ElfWord<{ ElfClass::Elf32 as u8 }, ED>,
    /// Some sections hold a table of fixed-size entries, such as a symbol table.  For
    /// such a section, this member gives the size in bytes of each entry.  The member
    /// contains 0 if the section does not hold a table of fixed- size entries
    entry_size: ElfWord<{ ElfClass::Elf32 as u8 }, ED>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
/// ELF Section Header
pub struct Elf64SectionHeader<const ED: u8> {
    /// The name of the section. Its value is an index into the section header string
    /// table section giving the location of a null-terminated string
    name: ElfWord<{ ElfClass::Elf64 as u8 }, ED>,
    /// The section's contents and semantics
    r#type: ElfSectionHeaderType<{ ElfClass::Elf64 as u8 }, ED>,
    /// Bit-flags that describe miscellaneous attributes
    flags: Elf64SectionHeaderFlags<ED>,
    /// If the section will appear in the memory image of a process, this member gives
    /// the address at which the section's first byte should reside. Otherwise, the
    /// member contains 0.
    address: ElfAddress<{ ElfClass::Elf64 as u8 }, ED>,
    /// This member's value gives the byte offset from the beginning of the file to the
    /// ﬁrst byte in the section. One section type, SHT_NOBITS described below, occupies
    /// no space in the file, and its sh_offset member locates the conceptual placement in
    /// the file.
    offset: ElfOffset<{ ElfClass::Elf64 as u8 }, ED>,
    /// This member gives the section's size in bytes. Unless the section type is
    /// SHT_NOBITS, the section occupies sh_size bytes in the file. A section of type
    /// SHT_NOBITS may have a non-zero size, but it occupies no space in the file.
    size: ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>,
    /// This member holds a section header table index link, whose interpretation
    /// depends on the section type. A table below describes the values.
    ///
    /// * SHT_DYNAMIC:
    ///     * link: The section header index of the string table used by entries
    ///       in the section
    ///     * info: 0
    /// * SHT_HASH:
    ///     * link: The section header index of the symbol table to which the
    ///       hash table applies
    ///     * info: 0
    /// * SHT_REL/SHT_RELA:
    ///     * link: The section header index of the associated symbol table
    ///     * info: The section header index of the section to which the relocation
    ///       applies
    /// * SHT_SYMTAB/SHT_DYNSYM:
    ///     * link: The section header index of the associated string table
    ///     * info: One greater than the symbol table index of the last local symbol
    ///       (binding STB_LOCAL)
    /// * SHT_GROUP:
    ///     * link: The section header index of the associated symbol table
    ///     * info: The symbol table index of an entry in the associated symbol table.
    ///       The name of the specified symbol table entry provides a signature for the
    ///       section group.
    /// * SHT_SYMTAB_SHNDX:
    ///     * link: The section header index of the associated symbol table section
    ///     * info: 0
    link: ElfWord<{ ElfClass::Elf64 as u8 }, ED>,
    /// This member holds extra information, whose interpretation depends on the section
    /// type. See `link` for the table describing meanings. If the sh_flags field for
    /// this section header includes the attribute SHF_INFO_LINK, then this member
    /// represents a section header table index.
    info: ElfWord<{ ElfClass::Elf64 as u8 }, ED>,
    /// Some sections have address alignment constraints. For example, if a section
    /// holds a doubleword, the system must ensure doubleword alignment for the entire
    /// section. The value of sh_addr must be congruent to 0, modulo the value of
    /// sh_addralign. Currently, only 0 and positive integral powers of two are allowed.
    /// Values 0 and 1 mean the section has no alignment constraints.
    address_align: ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>,
    /// Some sections hold a table of fixed-size entries, such as a symbol table.  For
    /// such a section, this member gives the size in bytes of each entry.  The member
    /// contains 0 if the section does not hold a table of fixed- size entries
    entry_size: ElfExtendedWord<{ ElfClass::Elf64 as u8 }, ED>,
}
