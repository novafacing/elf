//! Definitions for ELF Files

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use error::Error;
use header::elf::{
    identification::{
        ElfClass, ElfDataEncoding, ElfHeaderIdentifier, ElfOSABI, ELF_CLASS_DEFAULT,
        ELF_DATA_ENCODING_DEFAULT,
    },
    ElfHeader, ElfMachine,
};
use std::{
    collections::HashSet,
    io::{Read, Seek, SeekFrom, Write},
};
use typed_builder::TypedBuilder;

pub mod arch;
pub mod base;
pub mod error;
pub mod header;
pub mod os;

#[macro_export]
/// Add the ability to convert a primitive to an enum
macro_rules! from_primitive {
    (
        $(#[$enum_attr:meta])*
        $(visibility:vis)? enum $enum_name:ident <$(const $trait_param:ident : $trait_bound:tt),*> {
            $(
                $(#[$variant_attr:meta])*
                $variant_name:ident = $variant_value:expr,
            )*
        }
    ) => {
        $(#[$enum_attr])*
        pub enum $enum_name <$(const $trait_param : $trait_bound),*> {
            $(
                $(#[$variant_attr])*
                $variant_name = $variant_value,
            )*
        }

        impl <$(const $trait_param : $trait_bound),*> num_traits::FromPrimitive for $enum_name <$($trait_param),*> {
            fn from_i64(n: i64) -> Option<Self> {
                match n {
                    $(
                        $variant_value => Some($enum_name::$variant_name),
                    )*
                    _ => None,
                }
            }

            fn from_u64(n: u64) -> Option<Self> {
                match n {
                    $(
                        $variant_value => Some($enum_name::$variant_name),
                    )*
                    _ => None,
                }
            }
        }
    };
}

/// Decode an owned instance of a type from a reader
pub trait FromReader<R>
where
    R: Read + Seek,
    Self: Sized,
{
    /// The error type for this operation
    type Error;

    /// Decode an instance of this type from a reader
    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error>;

    /// Decode an instance of this type from a reader
    fn from_reader(reader: &mut R) -> Result<Self, Self::Error> {
        Self::from_reader_with(reader, &mut Config::default())
    }
}

/// Encode an instance of a type to a writer
pub trait ToWriter<W>
where
    W: Write,
    Self: Sized,
{
    /// The error type for this operation
    type Error;

    /// Encode an instance of this type to a writer
    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error>;
}

/// A type which always has a known size when written to a writer
pub trait HasWrittenSize {
    /// The size when written
    const SIZE: usize;
}

/// Attempt to convert a value from one type to another type, possibly
/// fallibly, and use a configuration
pub trait TryFromWithConfig<T>: Sized {
    /// The error type
    type Error;

    /// Try to convert from value with `config`
    fn try_from_with(value: T, config: &mut Config) -> Result<Self, Self::Error>;
}

#[derive(Debug, Default, TypedBuilder)]
/// A configuration for the object file handler. Primarily configures errors which should
/// be ignored.
pub struct Config {
    #[builder(default = ElfClass::default())]
    /// The default class to use if no valid class is found
    default_class: ElfClass,
    #[builder(default = ElfDataEncoding::default())]
    /// The default data encoding to use if no valid class is found
    default_encoding: ElfDataEncoding,
    #[builder(setter(into))]
    /// Ignored errors. Each error is handled somewhat differently when it is ignored,
    /// and the implementation for each is located where the error would have been raised.
    /// For example, an invalid data encoding may be inferred from the machine field.
    ignore: HashSet<Error>,
    #[builder(default, setter(into, strip_option))]
    /// The machine type of the ELF object currently being decoded
    machine: Option<ElfMachine<ELF_CLASS_DEFAULT, ELF_DATA_ENCODING_DEFAULT>>,
    #[builder(default, setter(into, strip_option))]
    /// The OS ABI of the ELF object currently being decoded
    os_abi: Option<ElfOSABI>,
}

impl Config {
    pub(crate) fn default_elf_kind<R>(&mut self, reader: &mut R) -> Result<ElfKind, Error>
    where
        R: Read + Seek,
    {
        match (self.default_class, self.default_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                Ok(ElfKind::Elf32LE(Elf::<
                    { ElfClass::ELF_CLASS_32 },
                    { ElfDataEncoding::ELF_DATA_ENCODING_LITTLE_ENDIAN },
                >::from_reader_with(
                    reader, self
                )?))
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => {
                Ok(ElfKind::Elf32LE(Elf::<
                    { ElfClass::ELF_CLASS_32 },
                    { ElfDataEncoding::ELF_DATA_ENCODING_LITTLE_ENDIAN },
                >::from_reader_with(
                    reader, self
                )?))
            }
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                Ok(ElfKind::Elf32LE(Elf::<
                    { ElfClass::ELF_CLASS_32 },
                    { ElfDataEncoding::ELF_DATA_ENCODING_LITTLE_ENDIAN },
                >::from_reader_with(
                    reader, self
                )?))
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => {
                Ok(ElfKind::Elf32LE(Elf::<
                    { ElfClass::ELF_CLASS_32 },
                    { ElfDataEncoding::ELF_DATA_ENCODING_LITTLE_ENDIAN },
                >::from_reader_with(
                    reader, self
                )?))
            }
            (c, e) => Err(Error::InvalidClassEncodingPair {
                class: c,
                encoding: e,
            }),
        }
    }
}

#[derive(Debug, Clone)]
/// An ELF object file
pub struct Elf<const EC: u8, const ED: u8> {
    /// The ELF object file header
    pub header: ElfHeader<EC, ED>,
}

impl<R, const EC: u8, const ED: u8> FromReader<R> for Elf<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        Ok(Self {
            header: ElfHeader::<EC, ED>::from_reader_with(reader, config)?,
        })
    }
}

#[derive(Debug, Clone)]
/// An ELF object file which may be of any class or any data encoding
pub enum ElfKind {
    /// A 32-bit, Little Endian ELF object file
    Elf32LE(Elf<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>),
    /// A 32-bit, Big Endian ELF object file
    Elf32BE(Elf<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>),
    /// A 64-bit, Little Endian ELF object file
    Elf64LE(Elf<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>),
    /// A 64-bit, Big Endian ELF object file
    Elf64BE(Elf<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>),
}

impl<R> FromReader<R> for ElfKind
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        reader
            .seek(SeekFrom::Start(0))
            .map_err(|e| Error::Io { kind: e.kind() })?;

        let ident = ElfHeaderIdentifier::from_reader_with(reader, config)?;

        reader
            .seek(SeekFrom::Start(0))
            .map_err(|e| Error::Io { kind: e.kind() })?;

        match (ident.class, ident.data_encoding) {
            (ElfClass::Elf32, ElfDataEncoding::LittleEndian) => {
                Ok(Self::Elf32LE(Elf::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >::from_reader_with(
                    reader, config
                )?))
            }
            (ElfClass::Elf32, ElfDataEncoding::BigEndian) => Ok(Self::Elf32BE(Elf::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >::from_reader_with(
                reader, config
            )?)),
            (ElfClass::Elf64, ElfDataEncoding::LittleEndian) => {
                Ok(Self::Elf64LE(Elf::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >::from_reader_with(
                    reader, config
                )?))
            }
            (ElfClass::Elf64, ElfDataEncoding::BigEndian) => Ok(Self::Elf64BE(Elf::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >::from_reader_with(
                reader, config
            )?)),
            (ElfClass::None, e) => {
                let err = Error::InvalidClassEncodingPair {
                    class: ElfClass::None,
                    encoding: e,
                };

                if config.ignore.contains(&err) {
                    config.default_elf_kind(reader)
                } else {
                    Err(err)
                }
            }
            (c, ElfDataEncoding::None) => {
                let err = Error::InvalidClassEncodingPair {
                    class: c,
                    encoding: ElfDataEncoding::None,
                };

                if config.ignore.contains(&err) {
                    config.default_elf_kind(reader)
                } else {
                    Err(err)
                }
            }
        }
    }
}

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::unwrap_used
)]
#[cfg(test)]
mod test {
    use std::io::ErrorKind;

    use self::error::ErrorContext;

    use super::*;
    use paste::paste;

    macro_rules! file_test {
        ($file:expr, $name:ident) => {
            paste! {
                pub const [<TEST_ $name:upper>]: &[u8] = include_bytes!(concat!("../", $file));
                #[test]
                fn [<test_ $name:lower>]() {
                        let mut test = Vec::from([<TEST_ $name:upper>]);
                        let _k = ElfKind::from_reader(&mut std::io::Cursor::new(&mut test)).unwrap();
                        println!("{}: {:#?}", $file, _k);
                }
            }
        };
    }

    file_test!("tests/corpus/elf/0pack", PACK0);

    file_test!("tests/corpus/elf/7786-utf16le", _7786_utf16le);

    file_test!("tests/corpus/elf/_Exit (42)", _Exit_42_);

    file_test!("tests/corpus/elf/a6ppc.out", a6ppc_out);

    file_test!("tests/corpus/elf/abcde-qt32", abcde_qt32);

    file_test!("tests/corpus/elf/ada_test_dwarf", ada_test_dwarf);

    file_test!("tests/corpus/elf/allxmm", allxmm);

    file_test!(
        "tests/corpus/elf/analysis/001.make.elf.x86_64",
        _001_make_elf_x86_64
    );

    file_test!(
        "tests/corpus/elf/analysis/6921737e-08e3-11e6-998c-a8ddd566ab1c.jpg",
        _6921737e_08e3_11e6_998c_a8ddd566ab1c_jpg
    );

    file_test!("tests/corpus/elf/analysis/a.out.asan", a_out_asan);

    file_test!("tests/corpus/elf/analysis/a.out.cpp", a_out_cpp);

    file_test!("tests/corpus/elf/analysis/a.out.fullrel", a_out_fullrel);

    file_test!(
        "tests/corpus/elf/analysis/a.out.partialrel",
        a_out_partialrel
    );

    file_test!("tests/corpus/elf/analysis/arm-ls", arm_ls);

    file_test!("tests/corpus/elf/analysis/arm-relocs", arm_relocs);

    file_test!("tests/corpus/elf/analysis/arm_32_flags0", arm_32_flags0);

    file_test!("tests/corpus/elf/analysis/armcall", armcall);

    file_test!("tests/corpus/elf/analysis/bigswitch_x86", bigswitch_x86);

    file_test!(
        "tests/corpus/elf/analysis/bigswitch_x86_64",
        bigswitch_x86_64
    );

    file_test!("tests/corpus/elf/analysis/bug-it-bb", bug_it_bb);

    file_test!(
        "tests/corpus/elf/analysis/bugurtos-avr.elf",
        bugurtos_avr_elf
    );

    file_test!("tests/corpus/elf/analysis/busybox-mips", busybox_mips);

    file_test!(
        "tests/corpus/elf/analysis/busybox-mips-phdr",
        busybox_mips_phdr
    );

    file_test!("tests/corpus/elf/analysis/busybox.m68k", busybox_m68k);

    file_test!("tests/corpus/elf/analysis/callback.elf", callback_elf);

    file_test!("tests/corpus/elf/analysis/calls_x64", calls_x64);

    file_test!("tests/corpus/elf/analysis/candypop", candypop);

    file_test!("tests/corpus/elf/analysis/ch2.bin", ch2_bin);

    file_test!("tests/corpus/elf/analysis/ch23.bin", ch23_bin);

    file_test!("tests/corpus/elf/analysis/clark", clark);

    file_test!("tests/corpus/elf/analysis/class_dlang", class_dlang);

    file_test!("tests/corpus/elf/analysis/class_pascal", class_pascal);

    file_test!("tests/corpus/elf/analysis/classes_Polygon", classes_Polygon);

    file_test!("tests/corpus/elf/analysis/compiled.elf", compiled_elf);

    file_test!("tests/corpus/elf/analysis/core.1159", core_1159);

    file_test!("tests/corpus/elf/analysis/crackmips", crackmips);

    file_test!("tests/corpus/elf/analysis/cris-dosfsck", cris_dosfsck);

    file_test!("tests/corpus/elf/analysis/custom_ldscript", custom_ldscript);

    file_test!(
        "tests/corpus/elf/analysis/dummy_secnames.elf",
        dummy_secnames_elf
    );

    file_test!(
        "tests/corpus/elf/analysis/dummy_secvals.elf",
        dummy_secvals_elf
    );

    file_test!("tests/corpus/elf/analysis/dummy_shnum.elf", dummy_shnum_elf);

    file_test!("tests/corpus/elf/analysis/dwarf_load", dwarf_load);

    file_test!("tests/corpus/elf/analysis/dwarftest", dwarftest);

    file_test!("tests/corpus/elf/analysis/dynamic-poffset", dynamic_poffset);

    file_test!("tests/corpus/elf/analysis/dynimports", dynimports);

    file_test!("tests/corpus/elf/analysis/elf-nx", elf_nx);

    file_test!(
        "tests/corpus/elf/analysis/elf-ppc-execstack",
        elf_ppc_execstack
    );

    file_test!("tests/corpus/elf/analysis/elf-relro", elf_relro);

    file_test!(
        "tests/corpus/elf/analysis/elf-sparc-execstack",
        elf_sparc_execstack
    );

    file_test!(
        "tests/corpus/elf/analysis/elf-virtualtable",
        elf_virtualtable
    );

    file_test!("tests/corpus/elf/analysis/elf-xnorelro", elf_xnorelro);

    file_test!("tests/corpus/elf/analysis/elf_overlapped", elf_overlapped);

    file_test!("tests/corpus/elf/analysis/example.elf", example_elf);

    file_test!("tests/corpus/elf/analysis/fast", fast);

    file_test!(
        "tests/corpus/elf/analysis/fedora_35_x86_64bit_ls",
        fedora_35_x86_64bit_ls
    );

    file_test!(
        "tests/corpus/elf/analysis/filetime.c-clang-x64-O0.o",
        filetime_c_clang_x64_O0_o
    );

    file_test!("tests/corpus/elf/analysis/go_stripped", go_stripped);

    file_test!("tests/corpus/elf/analysis/go_stripped2", go_stripped2);

    file_test!(
        "tests/corpus/elf/analysis/graalvm-example-truncated",
        graalvm_example_truncated
    );

    file_test!(
        "tests/corpus/elf/analysis/guess-number-riscv64",
        guess_number_riscv64
    );

    file_test!(
        "tests/corpus/elf/analysis/hello-android-arm",
        hello_android_arm
    );

    file_test!(
        "tests/corpus/elf/analysis/hello-android-mips",
        hello_android_mips
    );

    file_test!("tests/corpus/elf/analysis/hello-arm32", hello_arm32);

    file_test!(
        "tests/corpus/elf/analysis/hello-freebsd-x86_64",
        hello_freebsd_x86_64
    );

    file_test!("tests/corpus/elf/analysis/hello-hpux-ia64", hello_hpux_ia64);

    file_test!(
        "tests/corpus/elf/analysis/hello-linux-i386-pie",
        hello_linux_i386_pie
    );

    file_test!(
        "tests/corpus/elf/analysis/hello-linux-i386nold",
        hello_linux_i386nold
    );

    file_test!(
        "tests/corpus/elf/analysis/hello-linux-x86_64",
        hello_linux_x86_64
    );

    file_test!(
        "tests/corpus/elf/analysis/hello-netbsd-x86_64",
        hello_netbsd_x86_64
    );

    file_test!(
        "tests/corpus/elf/analysis/hello-objc-linux",
        hello_objc_linux
    );

    file_test!(
        "tests/corpus/elf/analysis/hello-openbsd-x86_64",
        hello_openbsd_x86_64
    );

    file_test!("tests/corpus/elf/analysis/hello-swift", hello_swift);

    file_test!("tests/corpus/elf/analysis/hello-utf-16", hello_utf_16);

    file_test!("tests/corpus/elf/analysis/hello-utf-16le", hello_utf_16le);

    file_test!("tests/corpus/elf/analysis/hello-utf-32", hello_utf_32);

    file_test!("tests/corpus/elf/analysis/hello-utf-32le", hello_utf_32le);

    file_test!(
        "tests/corpus/elf/analysis/hexagon-hello-loop",
        hexagon_hello_loop
    );

    file_test!("tests/corpus/elf/analysis/libsimplejni.so", libsimplejni_so);

    file_test!(
        "tests/corpus/elf/analysis/libstagefright_soft_g711dec.so",
        libstagefright_soft_g711dec_so
    );

    file_test!(
        "tests/corpus/elf/analysis/linux_x64_endbr64",
        linux_x64_endbr64
    );

    file_test!("tests/corpus/elf/analysis/longsym", longsym);

    file_test!("tests/corpus/elf/analysis/ls-alxchk", ls_alxchk);

    file_test!("tests/corpus/elf/analysis/ls-fedora29", ls_fedora29);

    file_test!(
        "tests/corpus/elf/analysis/ls-linux-x86_64-zlul",
        ls_linux_x86_64_zlul
    );

    file_test!("tests/corpus/elf/analysis/ls-linux64", ls_linux64);

    file_test!("tests/corpus/elf/analysis/ls-ppc-debian", ls_ppc_debian);

    file_test!("tests/corpus/elf/analysis/ls2", ls2);

    file_test!("tests/corpus/elf/analysis/ls_main_bug", ls_main_bug);

    file_test!("tests/corpus/elf/analysis/main", main);

    file_test!("tests/corpus/elf/analysis/main-g", main_g);

    file_test!("tests/corpus/elf/analysis/main_nosect", main_nosect);

    file_test!("tests/corpus/elf/analysis/main_structure", main_structure);

    file_test!("tests/corpus/elf/analysis/main_wrong_sect", main_wrong_sect);

    file_test!("tests/corpus/elf/analysis/mips-hello", mips_hello);

    file_test!("tests/corpus/elf/analysis/mips-main", mips_main);

    file_test!("tests/corpus/elf/analysis/mips.elf", mips_elf);

    file_test!(
        "tests/corpus/elf/analysis/mips64r2-busybox",
        mips64r2_busybox
    );

    file_test!(
        "tests/corpus/elf/analysis/mips64r2-busybox-loongson",
        mips64r2_busybox_loongson
    );

    file_test!("tests/corpus/elf/analysis/mips64r2-cc1", mips64r2_cc1);

    file_test!(
        "tests/corpus/elf/analysis/mips64r2-ld-2.28.so",
        mips64r2_ld_2_28_so
    );

    file_test!("tests/corpus/elf/analysis/mipsbe-busybox", mipsbe_busybox);

    file_test!("tests/corpus/elf/analysis/mipsbe-ip", mipsbe_ip);

    file_test!("tests/corpus/elf/analysis/mipsbe-ubusd", mipsbe_ubusd);

    file_test!(
        "tests/corpus/elf/analysis/mobile_bank.45115ff5f655d94fc26cb5244928b3fc",
        mobile_bank_45115ff5f655d94fc26cb5244928b3fc
    );

    file_test!("tests/corpus/elf/analysis/movfuscator", movfuscator);

    file_test!("tests/corpus/elf/analysis/no_sechdr.elf", no_sechdr_elf);

    file_test!("tests/corpus/elf/analysis/no_sections.elf", no_sections_elf);

    file_test!("tests/corpus/elf/analysis/noreturn", analysis_noreturn);

    file_test!("tests/corpus/elf/analysis/phdr-override", phdr_override);

    file_test!("tests/corpus/elf/analysis/pid_stripped", pid_stripped);

    file_test!("tests/corpus/elf/analysis/pie", pie);

    file_test!("tests/corpus/elf/analysis/pie-main", pie_main);

    file_test!(
        "tests/corpus/elf/analysis/rawmem.c-gcc-x64-O3.o",
        rawmem_c_gcc_x64_O3_o
    );

    file_test!("tests/corpus/elf/analysis/reference.out", reference_out);

    file_test!("tests/corpus/elf/analysis/regdump.elf", regdump_elf);

    file_test!(
        "tests/corpus/elf/analysis/risky-hitcon-riscv64",
        risky_hitcon_riscv64
    );

    file_test!("tests/corpus/elf/analysis/rust_full", rust_full);

    file_test!(
        "tests/corpus/elf/analysis/self-ref-typedef",
        self_ref_typedef
    );

    file_test!("tests/corpus/elf/analysis/serial", serial);

    file_test!("tests/corpus/elf/analysis/simple.elf", simple_elf);

    file_test!("tests/corpus/elf/analysis/simple64.elf", simple64_elf);

    file_test!("tests/corpus/elf/analysis/simpleARM.elf", simpleARM_elf);

    file_test!("tests/corpus/elf/analysis/simpleARM2.elf", simpleARM2_elf);

    file_test!("tests/corpus/elf/analysis/spurious-relocs", spurious_relocs);

    file_test!("tests/corpus/elf/analysis/standard.elf", standard_elf);

    file_test!("tests/corpus/elf/analysis/symtrash", symtrash);

    file_test!("tests/corpus/elf/analysis/test.obj", test_obj);

    file_test!(
        "tests/corpus/elf/analysis/test_hex_search_issues",
        test_hex_search_issues
    );

    file_test!("tests/corpus/elf/analysis/thumb", thumb);

    file_test!("tests/corpus/elf/analysis/tiny-crackme", tiny_crackme);

    file_test!(
        "tests/corpus/elf/analysis/tiny-crackme-vm-x86_64",
        tiny_crackme_vm_x86_64
    );

    file_test!("tests/corpus/elf/analysis/tiny1", tiny1);

    file_test!(
        "tests/corpus/elf/analysis/tiny1@invalid_addr",
        tiny1_invalid_addr
    );

    file_test!("tests/corpus/elf/analysis/true32", analysis_true32);

    file_test!("tests/corpus/elf/analysis/unoriginal", unoriginal);

    file_test!("tests/corpus/elf/analysis/x64-fork-test", x64_fork_test);

    file_test!("tests/corpus/elf/analysis/x64-loop", x64_loop);

    file_test!("tests/corpus/elf/analysis/x64-rep-stosq", x64_rep_stosq);

    file_test!("tests/corpus/elf/analysis/x64-simple", x64_simple);

    file_test!(
        "tests/corpus/elf/analysis/x86-helloworld-clang",
        x86_helloworld_clang
    );

    file_test!(
        "tests/corpus/elf/analysis/x86-helloworld-gcc",
        x86_helloworld_gcc
    );

    file_test!(
        "tests/corpus/elf/analysis/x86-helloworld-phdr",
        x86_helloworld_phdr
    );

    file_test!("tests/corpus/elf/analysis/x86-jmpeax", x86_jmpeax);

    file_test!("tests/corpus/elf/analysis/x86-simple", x86_simple);

    file_test!(
        "tests/corpus/elf/analysis/x86_cfg_node_details_test",
        x86_cfg_node_details_test
    );

    file_test!("tests/corpus/elf/analysis/x86_cfg_test", x86_cfg_test);

    file_test!(
        "tests/corpus/elf/analysis/x86_icfg_malloc_test",
        x86_icfg_malloc_test
    );

    file_test!("tests/corpus/elf/analysis/x86_icfg_test", x86_icfg_test);

    file_test!("tests/corpus/elf/analysis/xrefpic", xrefpic);

    file_test!("tests/corpus/elf/analysis/zigs", zigs);

    file_test!("tests/corpus/elf/analysis/zigs_stripped", zigs_stripped);

    file_test!("tests/corpus/elf/arch-x86_64-ls", arch_x86_64_ls);

    file_test!("tests/corpus/elf/arg", arg);

    file_test!("tests/corpus/elf/arg_down_32", arg_down_32);

    file_test!("tests/corpus/elf/arg_down_prop", arg_down_prop);

    file_test!("tests/corpus/elf/arm-init", arm_init);

    file_test!("tests/corpus/elf/arm1.bin", arm1_bin);

    file_test!("tests/corpus/elf/arm32esilcrash", arm32esilcrash);

    file_test!("tests/corpus/elf/avr-sbrx-rjmp.elf", avr_sbrx_rjmp_elf);

    file_test!("tests/corpus/elf/back1", back1);

    file_test!("tests/corpus/elf/bash", bash);

    file_test!(
        "tests/corpus/elf/bashbot.arm.gcc.O0.elf",
        bashbot_arm_gcc_O0_elf
    );

    file_test!(
        "tests/corpus/elf/bashbot.x86_64.O0.elf",
        bashbot_x86_64_O0_elf
    );

    file_test!("tests/corpus/elf/before-after-main", before_after_main);

    file_test!("tests/corpus/elf/boa-mips", boa_mips);

    file_test!("tests/corpus/elf/bomb", bomb);

    file_test!("tests/corpus/elf/busybox-phdr-shdr", busybox_phdr_shdr);

    file_test!("tests/corpus/elf/busybox-powerpc", busybox_powerpc);

    file_test!("tests/corpus/elf/calc.file", calc_file);

    file_test!(
        "tests/corpus/elf/calculate-freebsd-x64",
        calculate_freebsd_x64
    );

    file_test!("tests/corpus/elf/ch25.bin", ch25_bin);

    file_test!("tests/corpus/elf/class_test", class_test);

    file_test!("tests/corpus/elf/constr_type", constr_type);

    file_test!("tests/corpus/elf/core/core-linux-arm32", core_linux_arm32);

    file_test!("tests/corpus/elf/core/core-linux-arm64", core_linux_arm64);

    file_test!("tests/corpus/elf/core/core-linux-x86", core_linux_x86);

    file_test!("tests/corpus/elf/core/core-linux-x86_64", core_linux_x86_64);

    file_test!("tests/corpus/elf/core/crash-linux-arm32", crash_linux_arm32);

    file_test!("tests/corpus/elf/core/crash-linux-arm64", crash_linux_arm64);

    file_test!("tests/corpus/elf/core/crash-linux-x86", crash_linux_x86);

    file_test!(
        "tests/corpus/elf/core/crash-linux-x86_64",
        crash_linux_x86_64
    );

    file_test!("tests/corpus/elf/crackme", crackme);

    file_test!("tests/corpus/elf/crackme0x00b", crackme0x00b);

    file_test!("tests/corpus/elf/crackme0x05", crackme0x05);

    file_test!("tests/corpus/elf/dectest32", dectest32);

    file_test!("tests/corpus/elf/dectest64", dectest64);

    file_test!("tests/corpus/elf/demangle-test-cpp", demangle_test_cpp);

    file_test!("tests/corpus/elf/double_ptr", double_ptr);

    file_test!("tests/corpus/elf/dwarf/static_var", static_var);

    file_test!(
        "tests/corpus/elf/dwarf2_many_comp_units.elf",
        dwarf2_many_comp_units_elf
    );

    file_test!("tests/corpus/elf/dwarf3_c.elf", dwarf3_c_elf);

    file_test!(
        "tests/corpus/elf/dwarf3_c.elf.patched0",
        dwarf3_c_elf_patched0
    );

    file_test!("tests/corpus/elf/dwarf3_cpp.elf", dwarf3_cpp_elf);

    file_test!(
        "tests/corpus/elf/dwarf3_cpp.elf.patched0",
        dwarf3_cpp_elf_patched0
    );

    file_test!(
        "tests/corpus/elf/dwarf3_many_comp_units.elf",
        dwarf3_many_comp_units_elf
    );

    file_test!(
        "tests/corpus/elf/dwarf4_many_comp_units.elf",
        dwarf4_many_comp_units_elf
    );

    file_test!(
        "tests/corpus/elf/dwarf4_multidir_comp_units",
        dwarf4_multidir_comp_units
    );

    file_test!("tests/corpus/elf/dwarf_attr_check", dwarf_attr_check);

    file_test!(
        "tests/corpus/elf/dwarf_fuzzed_abbrev_empty",
        dwarf_fuzzed_abbrev_empty
    );

    file_test!("tests/corpus/elf/dwarf_go_tree", dwarf_go_tree);

    file_test!("tests/corpus/elf/dwarf_rust_bubble", dwarf_rust_bubble);

    file_test!(
        "tests/corpus/elf/dwarf_test_func_patched",
        dwarf_test_func_patched
    );

    file_test!("tests/corpus/elf/echo", echo);

    file_test!("tests/corpus/elf/echo-bin", echo_bin);

    file_test!(
        "tests/corpus/elf/elf-Linux-SparcV8-bash",
        elf_Linux_SparcV8_bash
    );

    file_test!(
        "tests/corpus/elf/elf-solaris-sparc-ls",
        elf_solaris_sparc_ls
    );

    file_test!("tests/corpus/elf/elf_one_symbol_shdr", elf_one_symbol_shdr);

    file_test!(
        "tests/corpus/elf/elf_one_symbol_shdr1",
        elf_one_symbol_shdr1
    );

    file_test!("tests/corpus/elf/elf_stripped", elf_stripped);

    file_test!("tests/corpus/elf/emulateme.arm32", emulateme_arm32);

    file_test!("tests/corpus/elf/emulateme.arm64", emulateme_arm64);

    file_test!("tests/corpus/elf/emulateme.x86", emulateme_x86);

    file_test!("tests/corpus/elf/emulateme_vfp.arm32", emulateme_vfp_arm32);

    file_test!("tests/corpus/elf/endbr-main", endbr_main);

    file_test!("tests/corpus/elf/errno", errno);

    file_test!("tests/corpus/elf/ezpz", ezpz);

    file_test!("tests/corpus/elf/fcn_in_test.elf", fcn_in_test_elf);

    file_test!("tests/corpus/elf/fedora_32_bin_ls", fedora_32_bin_ls);

    file_test!(
        "tests/corpus/elf/find_x86-sok_gcc_m32_O2",
        find_x86_sok_gcc_m32_O2
    );

    file_test!("tests/corpus/elf/flagspace", flagspace);

    file_test!("tests/corpus/elf/float_ex1/float_ex1_arm", float_ex1_arm);

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_arm_clang.dw.zlib",
        float_ex1_arm_clang_dw_zlib
    );

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_arm_clang.dw.zstd",
        float_ex1_arm_clang_dw_zstd
    );

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_arm_nodebug",
        float_ex1_arm_nodebug
    );

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_arm_stripped",
        float_ex1_arm_stripped
    );

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_buildid",
        float_ex1_buildid
    );

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_hightec",
        float_ex1_hightec
    );

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_split-dwarf",
        float_ex1_split_dwarf
    );

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_split-dwarf.debug",
        float_ex1_split_dwarf_debug
    );

    file_test!(
        "tests/corpus/elf/float_ex1/float_ex1_tricore_gcc",
        float_ex1_tricore_gcc
    );

    file_test!("tests/corpus/elf/float_point", float_point);

    file_test!("tests/corpus/elf/follow_ptr", follow_ptr);

    file_test!("tests/corpus/elf/format", format);

    file_test!("tests/corpus/elf/forward1", forward1);

    file_test!("tests/corpus/elf/game_of_thrones", game_of_thrones);

    file_test!("tests/corpus/elf/glibc-heap-2.27", glibc_heap_2_27);

    file_test!("tests/corpus/elf/glibc-heap-2.31", glibc_heap_2_31);

    file_test!("tests/corpus/elf/glibc-heap-2.32", glibc_heap_2_32);

    file_test!(
        "tests/corpus/elf/graphascii.c-clang-arm64-O0.o",
        graphascii_c_clang_arm64_O0_o
    );

    file_test!(
        "tests/corpus/elf/graphascii.c-gcc-arm64-O0.o",
        graphascii_c_gcc_arm64_O0_o
    );

    file_test!("tests/corpus/elf/hello-freebsd-x64", hello_freebsd_x64);

    file_test!("tests/corpus/elf/hello.ppc", hello_ppc);

    file_test!("tests/corpus/elf/hello.sysz", hello_sysz);

    file_test!("tests/corpus/elf/hello_world", hello_world);

    file_test!("tests/corpus/elf/hello_world32", hello_world32);

    file_test!("tests/corpus/elf/hexagon/hexagon-plt", hexagon_plt);

    file_test!("tests/corpus/elf/hexagon/relocs", relocs);

    file_test!("tests/corpus/elf/hexagon/rzil/brev", brev);

    file_test!("tests/corpus/elf/hexagon/rzil/circ", circ);

    file_test!("tests/corpus/elf/hexagon/rzil/dual_stores", dual_stores);

    file_test!("tests/corpus/elf/hexagon/rzil/first", first);

    file_test!("tests/corpus/elf/hexagon/rzil/float_convd", float_convd);

    file_test!("tests/corpus/elf/hexagon/rzil/float_convs", float_convs);

    file_test!("tests/corpus/elf/hexagon/rzil/float_madds", float_madds);

    file_test!("tests/corpus/elf/hexagon/rzil/fpstuff", fpstuff);

    file_test!("tests/corpus/elf/hexagon/rzil/hex_sigsegv", hex_sigsegv);

    file_test!("tests/corpus/elf/hexagon/rzil/hvx_histogram", hvx_histogram);

    file_test!("tests/corpus/elf/hexagon/rzil/hvx_misc", hvx_misc);

    file_test!("tests/corpus/elf/hexagon/rzil/invalid-slots", invalid_slots);

    file_test!("tests/corpus/elf/hexagon/rzil/linux-madvise", linux_madvise);

    file_test!("tests/corpus/elf/hexagon/rzil/linux-test", linux_test);

    file_test!("tests/corpus/elf/hexagon/rzil/load_align", load_align);

    file_test!("tests/corpus/elf/hexagon/rzil/load_unpack", load_unpack);

    file_test!("tests/corpus/elf/hexagon/rzil/mem_noshuf", mem_noshuf);

    file_test!(
        "tests/corpus/elf/hexagon/rzil/mem_noshuf_exception",
        mem_noshuf_exception
    );

    file_test!("tests/corpus/elf/hexagon/rzil/misc", misc);

    file_test!("tests/corpus/elf/hexagon/rzil/multi_result", multi_result);

    file_test!("tests/corpus/elf/hexagon/rzil/overflow", overflow);

    file_test!("tests/corpus/elf/hexagon/rzil/preg_alias", preg_alias);

    file_test!(
        "tests/corpus/elf/hexagon/rzil/read_write_overlap",
        read_write_overlap
    );

    file_test!("tests/corpus/elf/hexagon/rzil/reg_mut", reg_mut);

    file_test!(
        "tests/corpus/elf/hexagon/rzil/scatter_gather",
        scatter_gather
    );

    file_test!("tests/corpus/elf/hexagon/rzil/sha1", sha1);

    file_test!("tests/corpus/elf/hexagon/rzil/sha512", sha512);

    file_test!("tests/corpus/elf/hexagon/rzil/sigbus", sigbus);

    file_test!(
        "tests/corpus/elf/hexagon/rzil/signal_context",
        signal_context
    );

    file_test!("tests/corpus/elf/hexagon/rzil/signals", signals);

    file_test!("tests/corpus/elf/hexagon/rzil/test-mmap", test_mmap);

    file_test!("tests/corpus/elf/hexagon/rzil/test-vma", test_vma);

    file_test!("tests/corpus/elf/hexagon/rzil/test_abs", test_abs);

    file_test!("tests/corpus/elf/hexagon/rzil/test_bitcnt", test_bitcnt);

    file_test!("tests/corpus/elf/hexagon/rzil/test_bitsplit", test_bitsplit);

    file_test!("tests/corpus/elf/hexagon/rzil/test_call", test_call);

    file_test!("tests/corpus/elf/hexagon/rzil/test_clobber", test_clobber);

    file_test!("tests/corpus/elf/hexagon/rzil/test_cmp", test_cmp);

    file_test!("tests/corpus/elf/hexagon/rzil/test_dotnew", test_dotnew);

    file_test!("tests/corpus/elf/hexagon/rzil/test_ext", test_ext);

    file_test!(
        "tests/corpus/elf/hexagon/rzil/test_fibonacci",
        test_fibonacci
    );

    file_test!("tests/corpus/elf/hexagon/rzil/test_hl", test_hl);

    file_test!("tests/corpus/elf/hexagon/rzil/test_hwloops", test_hwloops);

    file_test!("tests/corpus/elf/hexagon/rzil/test_jmp", test_jmp);

    file_test!("tests/corpus/elf/hexagon/rzil/test_lsr", test_lsr);

    file_test!("tests/corpus/elf/hexagon/rzil/test_mpyi", test_mpyi);

    file_test!("tests/corpus/elf/hexagon/rzil/test_packet", test_packet);

    file_test!("tests/corpus/elf/hexagon/rzil/test_reorder", test_reorder);

    file_test!("tests/corpus/elf/hexagon/rzil/test_round", test_round);

    file_test!("tests/corpus/elf/hexagon/rzil/test_vavgw", test_vavgw);

    file_test!("tests/corpus/elf/hexagon/rzil/test_vcmpb", test_vcmpb);

    file_test!("tests/corpus/elf/hexagon/rzil/test_vcmpw", test_vcmpw);

    file_test!("tests/corpus/elf/hexagon/rzil/test_vlsrw", test_vlsrw);

    file_test!("tests/corpus/elf/hexagon/rzil/test_vmaxh", test_vmaxh);

    file_test!("tests/corpus/elf/hexagon/rzil/test_vminh", test_vminh);

    file_test!("tests/corpus/elf/hexagon/rzil/test_vpmpyh", test_vpmpyh);

    file_test!("tests/corpus/elf/hexagon/rzil/test_vspliceb", test_vspliceb);

    file_test!("tests/corpus/elf/hexagon/rzil/usr", usr);

    file_test!("tests/corpus/elf/hexagon/rzil/v68_hvx", v68_hvx);

    file_test!("tests/corpus/elf/hexagon/rzil/v68_scalar", v68_scalar);

    file_test!("tests/corpus/elf/hexagon/rzil/v69_hvx", v69_hvx);

    file_test!("tests/corpus/elf/hexagon/rzil/v73_scalar", v73_scalar);

    file_test!(
        "tests/corpus/elf/hexagon/rzil/vector_add_int",
        vector_add_int
    );

    file_test!("tests/corpus/elf/ifunc_rel64", ifunc_rel64);

    file_test!("tests/corpus/elf/infinite-loop.bf", infinite_loop_bf);

    file_test!("tests/corpus/elf/ioli/crackme0x00", crackme0x00);

    file_test!("tests/corpus/elf/ioli/crackme0x01", crackme0x01);

    file_test!("tests/corpus/elf/ioli/crackme0x02", crackme0x02);

    file_test!("tests/corpus/elf/ioli/crackme0x03", crackme0x03);

    file_test!("tests/corpus/elf/ioli/crackme0x04", crackme0x04);

    file_test!("tests/corpus/elf/ioli/crackme0x05", ioli_crackme0x05);

    file_test!("tests/corpus/elf/ioli/crackme0x06", crackme0x06);

    file_test!("tests/corpus/elf/ioli/crackme0x07", crackme0x07);

    file_test!("tests/corpus/elf/ioli/crackme0x08", crackme0x08);

    file_test!("tests/corpus/elf/ioli/crackme0x09", crackme0x09);

    file_test!("tests/corpus/elf/ip-riscv", ip_riscv);

    file_test!("tests/corpus/elf/jni/jniO0-arm64", jniO0_arm64);

    file_test!("tests/corpus/elf/jni/jniO0-mips", jniO0_mips);

    file_test!("tests/corpus/elf/jni/jniO0-x86", jniO0_x86);

    file_test!("tests/corpus/elf/jni/jniO2-arm64", jniO2_arm64);

    file_test!("tests/corpus/elf/jni/jniO2-mips", jniO2_mips);

    file_test!("tests/corpus/elf/jni/jniO2-x86", jniO2_x86);

    file_test!("tests/corpus/elf/l2rbin", l2rbin);

    file_test!("tests/corpus/elf/lab1B", lab1B);

    file_test!("tests/corpus/elf/lab2", lab2);

    file_test!("tests/corpus/elf/ld-2.27.so", ld_2_27_so);

    file_test!("tests/corpus/elf/ld-2.31.so", ld_2_31_so);

    file_test!("tests/corpus/elf/ld-2.32.so", ld_2_32_so);

    file_test!(
        "tests/corpus/elf/ld-uClibc-0.9.33.2.so",
        ld_uClibc_0_9_33_2_so
    );

    file_test!("tests/corpus/elf/libarm64.so", libarm64_so);

    file_test!("tests/corpus/elf/libc-2.27.so", libc_2_27_so);

    file_test!("tests/corpus/elf/libc-2.31.so", libc_2_31_so);

    file_test!("tests/corpus/elf/libc-2.32.so", libc_2_32_so);

    file_test!("tests/corpus/elf/libc.so.0", libc_so_0);

    file_test!("tests/corpus/elf/libc.so.6", libc_so_6);

    file_test!("tests/corpus/elf/libexploit.so", libexploit_so);

    file_test!("tests/corpus/elf/libmagic.so", libmagic_so);

    file_test!(
        "tests/corpus/elf/libmemalloc-dump-mem",
        libmemalloc_dump_mem
    );

    file_test!("tests/corpus/elf/librsjni_androix.so", librsjni_androix_so);

    file_test!(
        "tests/corpus/elf/libshella-2.10.3.1.so",
        libshella_2_10_3_1_so
    );

    file_test!("tests/corpus/elf/libstdc++.so.6", libstdc___so_6);

    file_test!("tests/corpus/elf/libverifyPass.so", libverifyPass_so);

    file_test!("tests/corpus/elf/license_1.out", license_1_out);

    file_test!(
        "tests/corpus/elf/linux-example-x86-32.ko",
        linux_example_x86_32_ko
    );

    file_test!("tests/corpus/elf/long-symbol.elf", long_symbol_elf);

    file_test!("tests/corpus/elf/ls", ls);

    file_test!("tests/corpus/elf/ls-cet", ls_cet);

    file_test!("tests/corpus/elf/ls.odd", ls_odd);

    file_test!("tests/corpus/elf/mentalminer", mentalminer);

    file_test!("tests/corpus/elf/mips-mozi", mips_mozi);

    file_test!("tests/corpus/elf/mipsloop", mipsloop);

    file_test!("tests/corpus/elf/mosquito-ppc64le", mosquito_ppc64le);

    file_test!("tests/corpus/elf/movstr", movstr);

    file_test!("tests/corpus/elf/msp430.elf", msp430_elf);

    file_test!("tests/corpus/elf/mtk-su", mtk_su);

    file_test!(
        "tests/corpus/elf/netbsd-calculate-x64",
        netbsd_calculate_x64
    );

    file_test!("tests/corpus/elf/netbsd-hello-x64", netbsd_hello_x64);

    file_test!("tests/corpus/elf/noreturn", noreturn);

    file_test!(
        "tests/corpus/elf/openbsd-arm64-nobtcfi",
        openbsd_arm64_nobtcfi
    );

    file_test!(
        "tests/corpus/elf/openbsd-calculate-x64",
        openbsd_calculate_x64
    );

    file_test!("tests/corpus/elf/openbsd-hello-x64", openbsd_hello_x64);

    file_test!("tests/corpus/elf/overlapped-segment", overlapped_segment);

    file_test!("tests/corpus/elf/padding_in_func", padding_in_func);

    file_test!("tests/corpus/elf/pngrutil_o", pngrutil_o);

    file_test!(
        "tests/corpus/elf/powerpc-linux-gnu-symexec-palindrome",
        powerpc_linux_gnu_symexec_palindrome
    );

    file_test!("tests/corpus/elf/ppc/asm_tests", asm_tests);

    file_test!("tests/corpus/elf/ppc/emulateme-ppc32be", emulateme_ppc32be);

    file_test!("tests/corpus/elf/ppc/emulateme-ppc32le", emulateme_ppc32le);

    file_test!("tests/corpus/elf/ppc/emulateme-ppc64be", emulateme_ppc64be);

    file_test!("tests/corpus/elf/ppc/emulateme-ppc64le", emulateme_ppc64le);

    file_test!("tests/corpus/elf/ppc/ppc32be_uplifted", ppc32be_uplifted);

    file_test!("tests/corpus/elf/ppc/ppc64le_uplifted", ppc64le_uplifted);

    file_test!("tests/corpus/elf/ppc/ppc_insn_tests", ppc_insn_tests);

    file_test!("tests/corpus/elf/ppc/pseudo_fuzz_tests", pseudo_fuzz_tests);

    file_test!("tests/corpus/elf/ppc64_sudoku_dwarf", ppc64_sudoku_dwarf);

    file_test!("tests/corpus/elf/ppc_classes", ppc_classes);

    file_test!("tests/corpus/elf/r2-dynstr-format", r2_dynstr_format);

    file_test!("tests/corpus/elf/r2pay-arm32.so", r2pay_arm32_so);

    file_test!("tests/corpus/elf/r2pay-arm64.so", r2pay_arm64_so);

    file_test!("tests/corpus/elf/radare2.c.obj", radare2_c_obj);

    file_test!("tests/corpus/elf/redpill", redpill);

    file_test!("tests/corpus/elf/retpoline", retpoline);

    file_test!("tests/corpus/elf/rust", rust);

    file_test!("tests/corpus/elf/sht_null_symbols", sht_null_symbols);

    file_test!("tests/corpus/elf/signed_test", signed_test);

    file_test!(
        "tests/corpus/elf/simple-hello-world-with-wrong-rela-section-name",
        simple_hello_world_with_wrong_rela_section_name
    );

    file_test!(
        "tests/corpus/elf/simple_malloc_x86_64",
        simple_malloc_x86_64
    );

    file_test!("tests/corpus/elf/smallstrings.elf", smallstrings_elf);

    file_test!("tests/corpus/elf/socket-syscall", socket_syscall);

    file_test!(
        "tests/corpus/elf/special-sym-with-dot.bin",
        special_sym_with_dot_bin
    );

    file_test!("tests/corpus/elf/sse2-add", sse2_add);

    file_test!("tests/corpus/elf/static-glibc-2.27", static_glibc_2_27);

    file_test!("tests/corpus/elf/strenc", strenc);

    file_test!("tests/corpus/elf/strenc-ctrlchars", strenc_ctrlchars);

    file_test!(
        "tests/corpus/elf/strenc-guess-utf32le",
        strenc_guess_utf32le
    );

    file_test!("tests/corpus/elf/struct64", struct64);

    file_test!("tests/corpus/elf/struct_2", struct_2);

    file_test!("tests/corpus/elf/struct_sample", struct_sample);

    file_test!(
        "tests/corpus/elf/switch-hello-world.elf",
        switch_hello_world_elf
    );

    file_test!("tests/corpus/elf/sym_version", sym_version);

    file_test!("tests/corpus/elf/syscall_mips", syscall_mips);

    file_test!("tests/corpus/elf/syscall_x86", syscall_x86);

    file_test!("tests/corpus/elf/tcache", tcache);

    file_test!("tests/corpus/elf/tcache-2.27", tcache_2_27);

    file_test!("tests/corpus/elf/test.ko", test_ko);

    file_test!("tests/corpus/elf/test_app2/test_app2.elf", test_app2_elf);

    file_test!("tests/corpus/elf/textile_hitcon2017", textile_hitcon2017);

    file_test!("tests/corpus/elf/tie-test", tie_test);

    file_test!("tests/corpus/elf/true", true);

    file_test!(
        "tests/corpus/elf/true-invalid-section-offset",
        true_invalid_section_offset
    );

    file_test!("tests/corpus/elf/true32", true32);

    file_test!("tests/corpus/elf/ts3server", ts3server);

    file_test!("tests/corpus/elf/two-words", two_words);

    file_test!("tests/corpus/elf/union_sample", union_sample);

    file_test!("tests/corpus/elf/utfbe&bom", utfbe_bom);

    file_test!(
        "tests/corpus/elf/vars-complex-x86_64-bp",
        vars_complex_x86_64_bp
    );

    file_test!(
        "tests/corpus/elf/vars-complex-x86_64-sp",
        vars_complex_x86_64_sp
    );

    file_test!("tests/corpus/elf/vars-mips-bp", vars_mips_bp);

    file_test!("tests/corpus/elf/vars-mips-sp", vars_mips_sp);

    file_test!("tests/corpus/elf/vars-x86_64-bp", vars_x86_64_bp);

    file_test!("tests/corpus/elf/vars-x86_64-sp", vars_x86_64_sp);

    file_test!("tests/corpus/elf/vars_args/example.x64", example_x64);

    file_test!("tests/corpus/elf/varsub", varsub);

    file_test!("tests/corpus/elf/varsub_2", varsub_2);

    file_test!("tests/corpus/elf/vim", vim);

    file_test!("tests/corpus/elf/ymm", ymm);

    pub const TEST_TINY_ELF: &[u8] = include_bytes!("../tests/corpus/elf/analysis/tiny.elf");

    #[test]
    fn test_tiny_elf() {
        let mut test = Vec::from(TEST_TINY_ELF);
        let _ = ElfKind::from_reader_with(
            &mut std::io::Cursor::new(&mut test),
            &mut Config::builder()
                .default_class(ElfClass::Elf32)
                .default_encoding(ElfDataEncoding::LittleEndian)
                .ignore([
                    Error::InvalidClassEncodingPair {
                        class: ElfClass::Elf32,
                        encoding: ElfDataEncoding::None,
                    },
                    Error::InvalidVersion {
                        context: ErrorContext::builder().offset(20).build(),
                    },
                    Error::Io {
                        kind: ErrorKind::UnexpectedEof,
                    },
                ])
                .build(),
        )
        .unwrap();
    }
}
