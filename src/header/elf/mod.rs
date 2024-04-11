//! Implementation of the ELF header. This header is located at the beginning
//! of the ELF object file and is the only header type in the ELF specification
//! which must be located at a specific offset (0) in the file.

use num_traits::FromPrimitive;
use std::{
    io::{Read, Seek, Write},
    mem::size_of,
};
use typed_builder::TypedBuilder;

use crate::{
    arch::arm32::ElfHeaderFlagsARM32,
    base::{ElfAddress, ElfByte, ElfHalfWord, ElfOffset, ElfWord},
    error::{Error, ErrorContext},
    from_primitive, Config, FromReader, HasWrittenSize, ToWriter,
};

use self::identification::ElfHeaderIdentifier;

pub mod identification;

from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[non_exhaustive]
    /// The ELF object type
    ///
    /// The following operating systems define no additional values
    /// for the e_type field:
    ///
    /// - Linux
    ///
    /// The following Processors define no additional values for
    /// the e_type field:
    ///
    /// - ARM32
    /// - AARCH64
    /// - i386
    /// - m68k
    /// - MIPS
    /// - PA-RISC
    /// - PPC
    /// - PPC64
    /// - RISC-V
    /// - S390
    /// - S390X
    /// - SPARC
    /// - x86_64
    ///
    /// Therefore, it is possible to have an undefined flag, but is unlikely in a
    /// well-formed ELF object file.  The OS-specific range of types is [0xfe00, 0xfeff]
    /// and the processor-specific range of types is [0xff00, 0xffff].
    enum ElfType<const EC: u8, const ED: u8> {
        /// No file type
        None = 0,
        /// Relocatable file type
        Relocatable = 1,
        /// Executable file type
        Executable = 2,
        /// Shared object file type
        Dynamic = 3,
        /// Core file
        Core = 4,
    }
}

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfType<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let ty = ElfHalfWord::<EC, ED>::from_reader_with(reader, config)?;

        if let Some(ty) = Self::from_u16(ty.0) {
            Ok(ty)
        } else {
            Err(Error::InvalidType {
                context: ErrorContext::from_reader(reader, size_of::<ElfHalfWord<EC, ED>>())
                    .map_err(Error::from)?,
            })
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfType<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfHalfWord::<EC, ED>((*self as u16).to_le()).to_writer(writer)
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfType<EC, ED> {
    const SIZE: usize = size_of::<ElfHalfWord<EC, ED>>();
}

from_primitive! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[non_exhaustive]
    /// The ELF object's machine
    enum ElfMachine<const EC: u8, const ED: u8> {
        /// No machine
        NONE = 0,
        /// AT&T WE 32100
        M32 = 1,
        /// SPARC
        SPARC = 2,
        /// Intel 80386
        I386 = 3,
        /// Motorola 68000
        M68K = 4,
        /// Motorola 88000
        M88K = 5,
        /// Intel MCU
        IAMCU = 6,
        /// Intel 80860
        I860 = 7,
        /// MIPS I Architecture
        MIPS = 8,
        /// IBM System/370 Processor
        S370 = 9,
        /// MIPS RS3000 Little-endian
        MIPS_RS3_LE = 10,
        // reserved	11-14	Reserved for future use
        /// Hewlett-Packard PA-RISC
        PARISC = 15,
        // reserved	16	Reserved for future use
        /// Fujitsu VPP500
        VPP500 = 17,
        /// Enhanced instruction set SPARC
        SPARC32PLUS = 18,
        /// Intel 80960
        I960 = 19,
        /// PowerPC
        PPC = 20,
        /// 64-bit PowerPC
        PPC64 = 21,
        /// IBM System/390 Processor
        S390 = 22,
        /// IBM SPU/SPC
        SPU = 23,
        // reserved	24-35	Reserved for future use
        /// NEC V800
        V800 = 36,
        /// Fujitsu FR20
        FR20 = 37,
        /// TRW RH-32
        RH32 = 38,
        /// Motorola RCE
        RCE = 39,
        /// ARM 32-bit architecture (AARCH32)
        ARM = 40,
        /// Digital Alpha
        ALPHA = 41,
        /// Hitachi SH
        SH = 42,
        /// SPARC Version 9
        SPARCV9 = 43,
        /// Siemens TriCore embedded processor
        TRICORE = 44,
        /// Argonaut RISC Core, Argonaut Technologies Inc.
        ARC = 45,
        /// Hitachi H8/300
        H8_300 = 46,
        /// Hitachi H8/300H
        H8_300H = 47,
        /// Hitachi H8S
        H8S = 48,
        /// Hitachi H8/500
        H8_500 = 49,
        /// Intel IA-64 processor architecture
        IA_64 = 50,
        /// Stanford MIPS-X
        MIPS_X = 51,
        /// Motorola ColdFire
        COLDFIRE = 52,
        /// Motorola M68HC12
        M68HC12 = 53,
        /// Fujitsu MMA Multimedia Accelerator
        MMA = 54,
        /// Siemens PCP
        PCP = 55,
        /// Sony nCPU embedded RISC processor
        NCPU = 56,
        /// Denso NDR1 microprocessor
        NDR1 = 57,
        /// Motorola Star*Core processor
        STARCORE = 58,
        /// Toyota ME16 processor
        ME16 = 59,
        /// STMicroelectronics ST100 processor
        ST100 = 60,
        /// Advanced Logic Corp. TinyJ embedded processor family
        TINYJ = 61,
        /// AMD x86-64 architecture
        X86_64 = 62,
        /// Sony DSP Processor
        PDSP = 63,
        /// Digital Equipment Corp. PDP-10
        PDP10 = 64,
        /// Digital Equipment Corp. PDP-11
        PDP11 = 65,
        /// Siemens FX66 microcontroller
        FX66 = 66,
        /// STMicroelectronics ST9+ 8/16 bit microcontroller
        ST9PLUS = 67,
        /// STMicroelectronics ST7 8-bit microcontroller
        ST7 = 68,
        /// Motorola MC68HC16 Microcontroller
        M68HC16 = 69,
        /// Motorola MC68HC11 Microcontroller
        M68HC11 = 70,
        /// Motorola MC68HC08 Microcontroller
        M68HC08 = 71,
        /// Motorola MC68HC05 Microcontroller
        M68HC05 = 72,
        /// Silicon Graphics SVx
        SVX = 73,
        /// STMicroelectronics ST19 8-bit microcontroller
        ST19 = 74,
        /// Digital VAX
        VAX = 75,
        /// Axis Communications 32-bit embedded processor
        CRIS = 76,
        /// Infineon Technologies 32-bit embedded processor
        JAVELIN = 77,
        /// Element 14 64-bit DSP Processor
        FIREPATH = 78,
        /// LSI Logic 16-bit DSP Processor
        ZSP = 79,
        /// Donald Knuth's educational 64-bit processor
        MMIX = 80,
        /// Harvard University machine-independent object files
        HUANY = 81,
        /// SiTera Prism
        PRISM = 82,
        /// Atmel AVR 8-bit microcontroller
        AVR = 83,
        /// Fujitsu FR30
        FR30 = 84,
        /// Mitsubishi D10V
        D10V = 85,
        /// Mitsubishi D30V
        D30V = 86,
        /// NEC v850
        V850 = 87,
        /// Mitsubishi M32R
        M32R = 88,
        /// Matsushita MN10300
        MN10300 = 89,
        /// Matsushita MN10200
        MN10200 = 90,
        /// picoJava
        PJ = 91,
        /// OpenRISC 32-bit embedded processor
        OPENRISC = 92,
        /// ARC International ARCompact processor (old spelling/synonym: ARC_A5)
        ARC_COMPACT = 93,
        /// Tensilica Xtensa Architecture
        XTENSA = 94,
        /// Alphamosaic VideoCore processor
        VIDEOCORE = 95,
        /// Thompson Multimedia General Purpose Processor
        TMM_GPP = 96,
        /// National Semiconductor 32000 series
        NS32K = 97,
        /// Tenor Network TPC processor
        TPC = 98,
        /// Trebia SNP 1000 processor
        SNP1K = 99,
        /// STMicroelectronics (www.st.com) ST200 microcontroller
        ST200 = 100,
        /// Ubicom IP2xxx microcontroller family
        IP2K = 101,
        /// MAX Processor
        MAX = 102,
        /// National Semiconductor CompactRISC microprocessor
        CR = 103,
        /// Fujitsu F2MC16
        F2MC16 = 104,
        /// Texas Instruments embedded microcontroller msp430
        MSP430 = 105,
        /// Analog Devices Blackfin (DSP) processor
        BLACKFIN = 106,
        /// S1C33 Family of Seiko Epson processors
        SE_C33 = 107,
        /// Sharp embedded microprocessor
        SEP = 108,
        /// Arca RISC Microprocessor
        ARCA = 109,
        /// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University
        UNICORE = 110,
        /// eXcess: 16/32/64-bit configurable embedded CPU
        EXCESS = 111,
        /// Icera Semiconductor Inc. Deep Execution Processor
        DXP = 112,
        /// Altera Nios II soft-core processor
        ALTERA_NIOS2 = 113,
        /// National Semiconductor CompactRISC CRX microprocessor
        CRX = 114,
        /// Motorola XGATE embedded processor
        XGATE = 115,
        /// Infineon C16x/XC16x processor
        C166 = 116,
        /// Renesas M16C series microprocessors
        M16C = 117,
        /// Microchip Technology dsPIC30F Digital Signal Controller
        DSPIC30F = 118,
        /// Freescale Communication Engine RISC core
        CE = 119,
        /// Renesas M32C series microprocessors
        M32C = 120,
        // reserved	121-130	Reserved for future use
        /// Altium TSK3000 core
        TSK3000 = 131,
        /// Freescale RS08 embedded processor
        RS08 = 132,
        /// Analog Devices SHARC family of 32-bit DSP processors
        SHARC = 133,
        /// Cyan Technology eCOG2 microprocessor
        ECOG2 = 134,
        /// Sunplus S+core7 RISC processor
        SCORE7 = 135,
        /// New Japan Radio (NJR) 24-bit DSP Processor
        DSP24 = 136,
        /// Broadcom VideoCore III processor
        VIDEOCORE3 = 137,
        /// RISC processor for Lattice FPGA architecture
        LATTICEMICO32 = 138,
        /// Seiko Epson C17 family
        SE_C17 = 139,
        /// The Texas Instruments TMS320C6000 DSP family
        TI_C6000 = 140,
        /// The Texas Instruments TMS320C2000 DSP family
        TI_C2000 = 141,
        /// The Texas Instruments TMS320C55x DSP family
        TI_C5500 = 142,
        /// Texas Instruments Application Specific RISC Processor, 32bit fetch
        TI_ARP32 = 143,
        /// Texas Instruments Programmable Realtime Unit
        TI_PRU = 144,
        // reserved	145-159	Reserved for future use
        /// STMicroelectronics 64bit VLIW Data Signal Processor
        MMDSP_PLUS = 160,
        /// Cypress M8C microprocessor
        CYPRESS_M8C = 161,
        /// Renesas R32C series microprocessors
        R32C = 162,
        /// NXP Semiconductors TriMedia architecture family
        TRIMEDIA = 163,
        /// QUALCOMM DSP6 Processor
        QDSP6 = 164,
        /// Intel 8051 and variants
        I8051 = 165,
        /// STMicroelectronics STxP7x family of configurable and extensible RISC processors
        STXP7X = 166,
        /// Andes Technology compact code size embedded RISC processor family
        NDS32 = 167,
        /// Cyan Technology eCOG1X family
        ECOG1 = 168,
        /// Dallas Semiconductor MAXQ30 Core Micro-controllers
        MAXQ30 = 169,
        /// New Japan Radio (NJR) 16-bit DSP Processor
        XIMO16 = 170,
        /// M2000 Reconfigurable RISC Microprocessor
        MANIK = 171,
        /// Cray Inc. NV2 vector architecture
        CRAYNV2 = 172,
        /// Renesas RX family
        RX = 173,
        /// Imagination Technologies META processor architecture
        METAG = 174,
        /// MCST Elbrus general purpose hardware architecture
        MCST_ELBRUS = 175,
        /// Cyan Technology eCOG16 family
        ECOG16 = 176,
        /// National Semiconductor CompactRISC CR16 16-bit microprocessor
        CR16 = 177,
        /// Freescale Extended Time Processing Unit
        ETPU = 178,
        /// Infineon Technologies SLE9X core
        SLE9X = 179,
        /// Intel L10M
        L10M = 180,
        /// Intel K10M
        K10M = 181,
        // reserved	182	Reserved for future Intel use
        /// ARM 64-bit architecture (AARCH64)
        AARCH64 = 183,
        // reserved	184	Reserved for future ARM use
        /// Atmel Corporation 32-bit microprocessor family
        AVR32 = 185,
        /// STMicroeletronics STM8 8-bit microcontroller
        STM8 = 186,
        /// Tilera TILE64 multicore architecture family
        TILE64 = 187,
        /// Tilera TILEPro multicore architecture family
        TILEPRO = 188,
        /// Xilinx MicroBlaze 32-bit RISC soft processor core
        MICROBLAZE = 189,
        /// NVIDIA CUDA architecture
        CUDA = 190,
        /// Tilera TILE-Gx multicore architecture family
        TILEGX = 191,
        /// CloudShield architecture family
        CLOUDSHIELD = 192,
        /// KIPO-KAIST Core-A 1st generation processor family
        COREA_1ST = 193,
        /// KIPO-KAIST Core-A 2nd generation processor family
        COREA_2ND = 194,
        /// Synopsys ARCompact V2
        ARC_COMPACT2 = 195,
        /// Open8 8-bit RISC soft processor core
        OPEN8 = 196,
        /// Renesas RL78 family
        RL78 = 197,
        /// Broadcom VideoCore V processor
        VIDEOCORE5 = 198,
        /// Renesas 78KOR family
        R78KOR = 199,
        /// Freescale 56800EX Digital Signal Controller (DSC)
        F56800EX = 200,
        /// Beyond BA1 CPU architecture
        BA1 = 201,
        /// Beyond BA2 CPU architecture
        BA2 = 202,
        /// XMOS xCORE processor family
        XCORE = 203,
        /// Microchip 8-bit PIC(r) family
        MCHP_PIC = 204,
        /// Reserved by Intel
        INTEL205 = 205,
        /// Reserved by Intel
        INTEL206 = 206,
        /// Reserved by Intel
        INTEL207 = 207,
        /// Reserved by Intel
        INTEL208 = 208,
        /// Reserved by Intel
        INTEL209 = 209,
        /// KM211 KM32 32-bit processor
        KM32 = 210,
        /// KM211 KMX32 32-bit processor
        KMX32 = 211,
        /// KM211 KMX16 16-bit processor
        KMX16 = 212,
        /// KM211 KMX8 8-bit processor
        KMX8 = 213,
        /// KM211 KVARC processor
        KVARC = 214,
        /// Paneve CDP architecture family
        CDP = 215,
        /// Cognitive Smart Memory Processor
        COGE = 216,
        /// Bluechip Systems CoolEngine
        COOL = 217,
        /// Nanoradio Optimized RISC
        NORC = 218,
        /// CSR Kalimba architecture family
        CSR_KALIMBA = 219,
        /// Zilog Z80
        Z80 = 220,
        /// Controls and Data Services VISIUMcore processor
        VISIUM = 221,
        /// FTDI Chip FT32 high performance 32-bit RISC architecture
        FT32 = 222,
        /// Moxie processor family
        MOXIE = 223,
        /// AMD GPU architecture
        AMDGPU = 224,
        // 225 - 242 reserved
        /// RISC-V
        Riscv = 243,
        /// Linux BPF -- in-kernel virtual machine
        BPF = 247,
        /// C-SKY
        CSKY = 252,
        /// LoongArch
        LOONGARCH = 258,
    }
}

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfMachine<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let machine = ElfHalfWord::<EC, ED>::from_reader_with(reader, config)?;

        if let Some(machine) = Self::from_u16(machine.0) {
            Ok(machine)
        } else {
            Err(Error::InvalidMachine {
                context: ErrorContext::from_reader(reader, size_of::<ElfHalfWord<EC, ED>>())
                    .map_err(Error::from)?,
            })
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfMachine<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfHalfWord::<EC, ED>((*self as u16).to_le()).to_writer(writer)
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfMachine<EC, ED> {
    const SIZE: usize = size_of::<ElfHalfWord<EC, ED>>();
}

from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[non_exhaustive]
    /// The ELF object's version
    enum ElfVersion<const EC: u8, const ED: u8> {
        /// Invalid version
        None = 0,
        /// Current version
        Current = 1,
    }
}

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfVersion<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let version = ElfWord::<EC, ED>::from_reader_with(reader, config)?;

        if let Some(version) = Self::from_u32(version.0) {
            Ok(version)
        } else {
            let err = Error::InvalidVersion {
                context: ErrorContext::from_reader(reader, size_of::<ElfWord<EC, ED>>())
                    .map_err(Error::from)?,
            };

            if config.ignore.contains(&err) {
                Ok(Self::None)
            } else {
                Err(err)
            }
        }
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfVersion<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        ElfWord::<EC, ED>((*self as u32).to_le()).to_writer(writer)
    }
}

impl<const EC: u8, const ED: u8> HasWrittenSize for ElfVersion<EC, ED> {
    const SIZE: usize = size_of::<ElfWord<EC, ED>>();
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Flags for an ELF header, which may contain processor and OS-specific
/// flags.
pub enum ElfHeaderFlags<const EC: u8, const ED: u8> {
    /// Platform-specific flags for ARM32
    ARM32(ElfHeaderFlagsARM32<EC, ED>),
    /// Platform-specific flags for AARCH64
    ///
    /// AARCH64 defines no processor-specific flags and specifies this field
    /// shall be zero
    AARCH64 {
        /// The value of the flags field
        value: ElfWord<EC, ED>,
    },
    /// Platform-specific flags for i386
    ///
    /// i386 defines no processor-specific flags but does not specify the value
    /// of this field
    I386 {
        /// The value of the flags field
        value: ElfWord<EC, ED>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
/// The header for an ELF object. Resides at the beginning and holds a ``road map''
/// describing the file's organization
pub struct ElfHeader<const EC: u8, const ED: u8> {
    /// The file's identifier information, which marks the file as an object file
    /// and provide machine- independent data with which to decode and interpret the
    pub identifier: ElfHeaderIdentifier,
    /// The object file type
    pub r#type: ElfType<EC, ED>,
    /// The file's machine, which specifies the required architecture for this
    /// object file
    pub machine: ElfMachine<EC, ED>,
    /// The object file version
    pub version: ElfVersion<EC, ED>,
    /// The file's entrypoint. This is the virtual address to which the system
    /// first transfers control, thus starting the process. If the object has no
    /// associated entry point, this member is zero (absent).
    pub entrypoint: Option<ElfAddress<EC, ED>>,
    /// The program header table's file offset in bytes. If the file has no program
    /// header table, this member is zero (absent).
    pub program_header_offset: Option<ElfOffset<EC, ED>>,
    /// The section header table's file offset in bytes. If the file has no section
    /// header table, this member is zero (absent).
    pub section_header_offset: Option<ElfOffset<EC, ED>>,
    /// The processor-specific flags associated with the file.
    /// TODO: Make this a trait abstract over the various architectures' flags
    pub flags: ElfWord<EC, ED>,
    /// The ELF header's size in bytes
    pub header_size: ElfHalfWord<EC, ED>,
    /// The size in bytes of a program header table entry; all entries are the same
    /// size
    pub program_header_entry_size: ElfHalfWord<EC, ED>,
    /// The number of entries in the program header table. If the file has no
    /// program header table, this member is zero (absent).
    pub program_header_entry_count: ElfHalfWord<EC, ED>,
    /// The size in bytes of a section header table entry; all entries are the same
    /// size
    pub section_header_entry_size: ElfHalfWord<EC, ED>,
    /// The number of entries in the section header table.  Thus the product of
    /// e_shentsize and e_shnum gives the section header table's size in bytes. If a file
    /// has no section header table, e_shnum holds the value zero.  If the number of
    /// sections is greater than or equal to SHN_LORESERVE (0xﬀ00), this member has the
    /// value zero and the actual number of section header table entries is contained in
    /// the sh_size field of the section header at index 0. (Otherwise, the sh_size
    /// member of the initial entry contains 0.)
    pub section_header_entry_count: ElfHalfWord<EC, ED>,
    /// This member holds the section header table index of the entry associated with
    /// the section name string table. If the file has no section name string table, this
    /// member holds the value SHN_UNDEF. See ``Sections'' and ``String Table'' below
    /// for more information.  If the section name string table section index is greater
    /// than or equal to SHN_LORESERVE (0xﬀ00), this member has the value SHN_XINDEX
    /// (0xﬀﬀ) and the actual index of the section name string table section is
    /// contained in the sh_link field of the section header at index 0.  (Otherwise, the
    /// sh_link member of the initial entry contains 0.)
    pub section_name_string_table_index: ElfHalfWord<EC, ED>,
    /// Extra data in the elf header. The contents of this data are not specified by the ELF
    /// specification, but extra data may be part of the header as specified by
    /// `header_size`. The size of this data is equal to the `header_size` minus the size of
    /// the preceding fields.
    pub data: Vec<ElfByte>,
}

impl<const EC: u8, const ED: u8> ElfHeader<EC, ED> {
    /// The size of the ELF header structure in bytes, less the size of the `data` field
    pub const SIZE: usize = ElfHeaderIdentifier::SIZE
        + ElfType::<EC, ED>::SIZE
        + ElfMachine::<EC, ED>::SIZE
        + ElfVersion::<EC, ED>::SIZE
        + ElfAddress::<EC, ED>::SIZE
        + ElfOffset::<EC, ED>::SIZE
        + ElfOffset::<EC, ED>::SIZE
        + ElfWord::<EC, ED>::SIZE
        + (ElfHalfWord::<EC, ED>::SIZE * 6);
}

impl<R, const EC: u8, const ED: u8> FromReader<R> for ElfHeader<EC, ED>
where
    R: Read + Seek,
{
    type Error = Error;

    fn from_reader_with(reader: &mut R, config: &mut Config) -> Result<Self, Self::Error> {
        let identifier = ElfHeaderIdentifier::from_reader_with(reader, config)?;
        let r#type = ElfType::<EC, ED>::from_reader_with(reader, config)?;
        let machine = ElfMachine::<EC, ED>::from_reader_with(reader, config)?;
        let version = ElfVersion::<EC, ED>::from_reader_with(reader, config)?;
        let entrypoint = ElfAddress::<EC, ED>::from_reader_with(reader, config).ok();
        let program_header_offset = ElfOffset::<EC, ED>::from_reader_with(reader, config).ok();
        let section_header_offset = ElfOffset::<EC, ED>::from_reader_with(reader, config).ok();
        let flags = ElfWord::<EC, ED>::from_reader_with(reader, config)?;
        let header_size = ElfHalfWord::<EC, ED>::from_reader_with(reader, config)?;
        let program_header_entry_size = ElfHalfWord::<EC, ED>::from_reader_with(reader, config)?;
        let program_header_entry_count = ElfHalfWord::<EC, ED>::from_reader_with(reader, config)?;
        let section_header_entry_size = ElfHalfWord::<EC, ED>::from_reader_with(reader, config)?;
        let section_header_entry_count = ElfHalfWord::<EC, ED>::from_reader_with(reader, config)?;
        let section_name_string_table_index =
            ElfHalfWord::<EC, ED>::from_reader_with(reader, config)?;

        let data = {
            let mut data = vec![ElfByte(0); (header_size.0 as usize).saturating_sub(Self::SIZE)];
            data.iter_mut()
                .try_for_each(|b| ElfByte::from_reader_with(reader, config).map(|r| *b = r))?;
            data
        };

        Ok(Self {
            identifier,
            r#type,
            machine,
            version,
            entrypoint,
            program_header_offset,
            section_header_offset,
            flags,
            header_size,
            program_header_entry_size,
            program_header_entry_count,
            section_header_entry_size,
            section_header_entry_count,
            section_name_string_table_index,
            data,
        })
    }
}

impl<W, const EC: u8, const ED: u8> ToWriter<W> for ElfHeader<EC, ED>
where
    W: Write,
{
    type Error = Error;

    fn to_writer(&self, writer: &mut W) -> Result<(), Self::Error> {
        self.identifier.to_writer(writer)?;
        self.r#type.to_writer(writer)?;
        self.machine.to_writer(writer)?;
        self.version.to_writer(writer)?;
        if let Some(entrypoint) = self.entrypoint {
            entrypoint.to_writer(writer)?;
        } else {
            ElfAddress::<EC, ED>(0).to_writer(writer)?;
        }
        if let Some(program_header_offset) = self.program_header_offset {
            program_header_offset.to_writer(writer)?;
        } else {
            ElfOffset::<EC, ED>(0).to_writer(writer)?;
        }
        if let Some(section_header_offset) = self.section_header_offset {
            section_header_offset.to_writer(writer)?;
        } else {
            ElfOffset::<EC, ED>(0).to_writer(writer)?;
        }
        self.flags.to_writer(writer)?;
        self.header_size.to_writer(writer)?;
        self.program_header_entry_size.to_writer(writer)?;
        self.program_header_entry_count.to_writer(writer)?;
        self.section_header_entry_size.to_writer(writer)?;
        self.section_header_entry_count.to_writer(writer)?;
        self.section_name_string_table_index.to_writer(writer)?;
        Ok(())
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;
    use identification::*;

    #[test]
    fn test_elf_type() {
        let mut bytes_le = &[0x01, 0x00];
        let mut bytes_be = &[0x00, 0x01];

        let le32t = ElfType::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be32t = ElfType::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();
        let le64t = ElfType::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be64t = ElfType::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();

        assert_eq!(le32t, ElfType::Relocatable);
        assert_eq!(be32t, ElfType::Relocatable);
        assert_eq!(le64t, ElfType::Relocatable);
        assert_eq!(be64t, ElfType::Relocatable);

        let mut le32t_out = Vec::new();
        le32t.to_writer(&mut le32t_out).unwrap();
        assert_eq!(le32t_out, bytes_le);
        let mut be32t_out = Vec::new();
        be32t.to_writer(&mut be32t_out).unwrap();
        assert_eq!(be32t_out, bytes_be);
        let mut le64t_out = Vec::new();
        le64t.to_writer(&mut le64t_out).unwrap();
        assert_eq!(le64t_out, bytes_le);
        let mut be64t_out = Vec::new();
        be64t.to_writer(&mut be64t_out).unwrap();
        assert_eq!(be64t_out, bytes_be);
    }

    #[test]
    fn test_elf_machine() {
        let mut bytes_le = &[0x03, 0x00];
        let mut bytes_be = &[0x00, 0x03];

        let le32m = ElfMachine::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be32m = ElfMachine::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();
        let le64m = ElfMachine::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be64m = ElfMachine::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();

        assert_eq!(le32m, ElfMachine::I386);
        assert_eq!(be32m, ElfMachine::I386);
        assert_eq!(le64m, ElfMachine::I386);
        assert_eq!(be64m, ElfMachine::I386);

        let mut le32m_out = Vec::new();
        le32m.to_writer(&mut le32m_out).unwrap();
        assert_eq!(le32m_out, bytes_le);
        let mut be32m_out = Vec::new();
        be32m.to_writer(&mut be32m_out).unwrap();
        assert_eq!(be32m_out, bytes_be);
        let mut le64m_out = Vec::new();
        le64m.to_writer(&mut le64m_out).unwrap();
        assert_eq!(le64m_out, bytes_le);
        let mut be64m_out = Vec::new();
        be64m.to_writer(&mut be64m_out).unwrap();
    }

    #[test]
    fn test_elf_version() {
        let mut bytes_le = &[0x01, 0x00, 0x00, 0x00];
        let mut bytes_be = &[0x00, 0x00, 0x00, 0x01];

        let le32v = ElfVersion::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be32v = ElfVersion::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();
        let le64v = ElfVersion::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be64v = ElfVersion::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();

        assert_eq!(le32v, ElfVersion::Current);
        assert_eq!(be32v, ElfVersion::Current);
        assert_eq!(le64v, ElfVersion::Current);
        assert_eq!(be64v, ElfVersion::Current);

        let mut le32v_out = Vec::new();
        le32v.to_writer(&mut le32v_out).unwrap();
        assert_eq!(le32v_out, bytes_le);
        let mut be32v_out = Vec::new();
        be32v.to_writer(&mut be32v_out).unwrap();
        assert_eq!(be32v_out, bytes_be);
        let mut le64v_out = Vec::new();
        le64v.to_writer(&mut le64v_out).unwrap();
        assert_eq!(le64v_out, bytes_le);
        let mut be64v_out = Vec::new();
        be64v.to_writer(&mut be64v_out).unwrap();
        assert_eq!(be64v_out, bytes_be);
    }

    #[test]
    fn test_elf_entry() {
        let mut bytes_le = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut bytes_be = &[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01];

        let le32e = ElfAddress::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be32e = ElfAddress::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();
        let le64e = ElfAddress::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be64e = ElfAddress::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();

        assert_eq!(le32e.0, 0x04030201);
        assert_eq!(be32e.0, 0x08070605);
        assert_eq!(le64e.0, 0x0807060504030201);
        assert_eq!(be64e.0, 0x0807060504030201);

        let mut le32e_out = Vec::new();
        le32e.to_writer(&mut le32e_out).unwrap();
        assert_eq!(le32e_out, bytes_le[..4]);
        let mut be32e_out = Vec::new();
        be32e.to_writer(&mut be32e_out).unwrap();
        assert_eq!(be32e_out, bytes_be[..4]);
        let mut le64e_out = Vec::new();
        le64e.to_writer(&mut le64e_out).unwrap();
        assert_eq!(le64e_out, bytes_le);
        let mut be64e_out = Vec::new();
        be64e.to_writer(&mut be64e_out).unwrap();
        assert_eq!(be64e_out, bytes_be);
    }

    #[test]
    fn test_program_header_offset() {
        let mut bytes_le = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut bytes_be = &[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01];

        let le32o = ElfOffset::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be32o = ElfOffset::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();
        let le64o = ElfOffset::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be64o = ElfOffset::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();

        assert_eq!(le32o.0, 0x04030201);
        assert_eq!(be32o.0, 0x08070605);
        assert_eq!(le64o.0, 0x0807060504030201);
        assert_eq!(be64o.0, 0x0807060504030201);

        let mut le32o_out = Vec::new();
        le32o.to_writer(&mut le32o_out).unwrap();
        assert_eq!(le32o_out, bytes_le[..4]);
        let mut be32o_out = Vec::new();
        be32o.to_writer(&mut be32o_out).unwrap();
        assert_eq!(be32o_out, bytes_be[..4]);
        let mut le64o_out = Vec::new();
        le64o.to_writer(&mut le64o_out).unwrap();
        assert_eq!(le64o_out, bytes_le);
        let mut be64o_out = Vec::new();
        be64o.to_writer(&mut be64o_out).unwrap();
        assert_eq!(be64o_out, bytes_be);
    }

    #[test]
    fn test_section_header_offset() {
        let mut bytes_le = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut bytes_be = &[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01];

        let le32o = ElfOffset::<{ ElfClass::Elf32 as u8}, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be32o = ElfOffset::<{ ElfClass::Elf32 as u8}, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();
        let le64o = ElfOffset::<{ ElfClass::Elf64 as u8}, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be64o = ElfOffset::<{ ElfClass::Elf64 as u8}, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();

        assert_eq!(le32o.0, 0x04030201);
        assert_eq!(be32o.0, 0x08070605);
        assert_eq!(le64o.0, 0x0807060504030201);
        assert_eq!(be64o.0, 0x0807060504030201);

        let mut le32o_out = Vec::new();
        le32o.to_writer(&mut le32o_out).unwrap();
        assert_eq!(le32o_out, bytes_le[..4]);
        let mut be32o_out = Vec::new();
        be32o.to_writer(&mut be32o_out).unwrap();
        assert_eq!(be32o_out, bytes_be[..4]);
        let mut le64o_out = Vec::new();
        le64o.to_writer(&mut le64o_out).unwrap();
        assert_eq!(le64o_out, bytes_le);
        let mut be64o_out = Vec::new();
        be64o.to_writer(&mut be64o_out).unwrap();
        assert_eq!(be64o_out, bytes_be);
    }

    #[test]
    fn test_flags() {
        let mut bytes_le = &[0x01, 0x02, 0x03, 0x04];
        let mut bytes_be = &[0x04, 0x03, 0x02, 0x01];

        let le32f = ElfWord::<{ ElfClass::Elf32 as u8}, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be32f = ElfWord::<{ ElfClass::Elf32 as u8}, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();
        let le64f = ElfWord::<{ ElfClass::Elf64 as u8}, { ElfDataEncoding::LittleEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_le)).unwrap();
        let be64f = ElfWord::<{ ElfClass::Elf64 as u8}, { ElfDataEncoding::BigEndian as u8 }>::from_reader(&mut std::io::Cursor::new(&mut bytes_be)).unwrap();

        assert_eq!(le32f.0, 0x04030201);
        assert_eq!(be32f.0, 0x04030201);
        assert_eq!(le64f.0, 0x04030201);
        assert_eq!(be64f.0, 0x04030201);

        let mut le32f_out = Vec::new();
        le32f.to_writer(&mut le32f_out).unwrap();
        assert_eq!(le32f_out, bytes_le);
        let mut be32f_out = Vec::new();
        be32f.to_writer(&mut be32f_out).unwrap();
        assert_eq!(be32f_out, bytes_be);
        let mut le64f_out = Vec::new();
        le64f.to_writer(&mut le64f_out).unwrap();
        assert_eq!(le64f_out, bytes_le);
        let mut be64f_out = Vec::new();
        be64f.to_writer(&mut be64f_out).unwrap();
        assert_eq!(be64f_out, bytes_be);
    }

    #[test]
    fn test_elf_header_le32() {
        let le32_id = ElfHeaderIdentifier {
            magic: [ElfByte(0x7f), ElfByte(0x45), ElfByte(0x4c), ElfByte(0x46)],
            class: ElfClass::Elf32,
            data_encoding: ElfDataEncoding::LittleEndian,
            version: ElfIdentifierVersion::Current,
            os_abi: ElfOSABI::NoneSystemV,
            abi_version: ElfByte(0),
            pad: [ElfByte(0); 7],
        };

        let le32_hdr =
            ElfHeader::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }> {
                identifier: le32_id.clone(),
                r#type: ElfType::Executable,
                machine: ElfMachine::X86_64,
                version: ElfVersion::Current,
                entrypoint: Some(ElfAddress::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0)),
                program_header_offset: Some(ElfOffset::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0)),
                section_header_offset: Some(ElfOffset::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0)),
                flags: ElfWord::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>(
                    0,
                ),
                // NOTE: No extra size, ends at the section name string table index
                header_size:
                    ElfHalfWord::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>(
                        ElfHeader::<
                            { ElfClass::Elf32 as u8 },
                            { ElfDataEncoding::LittleEndian as u8 },
                        >::SIZE
                            .try_into()
                            .unwrap(),
                    ),
                program_header_entry_size: ElfHalfWord::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                program_header_entry_count: ElfHalfWord::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                section_header_entry_size: ElfHalfWord::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                section_header_entry_count: ElfHalfWord::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                section_name_string_table_index: ElfHalfWord::<
                    { ElfClass::Elf32 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                data: vec![],
            };

        let mut le32_bytes = Vec::new();
        le32_hdr.to_writer(&mut le32_bytes).unwrap();
        let le32_header_read = ElfHeader::<
            { ElfClass::Elf32 as u8 },
            { ElfDataEncoding::LittleEndian as u8 },
        >::from_reader(&mut std::io::Cursor::new(&mut le32_bytes))
        .unwrap();
        assert_eq!(le32_header_read, le32_hdr);
    }

    #[test]
    fn test_elf_header_be32() {
        let be32_id = ElfHeaderIdentifier {
            magic: [ElfByte(0x7f), ElfByte(0x45), ElfByte(0x4c), ElfByte(0x46)],
            class: ElfClass::Elf32,
            data_encoding: ElfDataEncoding::BigEndian,
            version: ElfIdentifierVersion::Current,
            os_abi: ElfOSABI::NoneSystemV,
            abi_version: ElfByte(0),
            pad: [ElfByte(0); 7],
        };

        let be32_hdr = ElfHeader::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }> {
            identifier: be32_id.clone(),
            r#type: ElfType::Executable,
            machine: ElfMachine::X86_64,
            version: ElfVersion::Current,
            entrypoint: Some(ElfAddress::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0)),
            program_header_offset: Some(ElfOffset::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0)),
            section_header_offset: Some(ElfOffset::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0)),
            flags: ElfWord::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>(0),
            // NOTE: No extra size, ends at the section name string table index
            header_size: ElfHalfWord::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(
                ElfHeader::<{ ElfClass::Elf32 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::SIZE
                    .try_into()
                    .unwrap(),
            ),
            program_header_entry_size: ElfHalfWord::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            program_header_entry_count: ElfHalfWord::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            section_header_entry_size: ElfHalfWord::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            section_header_entry_count: ElfHalfWord::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            section_name_string_table_index: ElfHalfWord::<
                { ElfClass::Elf32 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            data: vec![],
        };

        let mut be32_bytes = Vec::new();
        be32_hdr.to_writer(&mut be32_bytes).unwrap();
        let be32_header_read = ElfHeader::<
            { ElfClass::Elf32 as u8 },
            { ElfDataEncoding::BigEndian as u8 },
        >::from_reader(&mut std::io::Cursor::new(&mut be32_bytes))
        .unwrap();
        assert_eq!(be32_header_read, be32_hdr);
    }

    #[test]
    fn test_elf_header_le64() {
        let le64_id = ElfHeaderIdentifier {
            magic: [ElfByte(0x7f), ElfByte(0x45), ElfByte(0x4c), ElfByte(0x46)],
            class: ElfClass::Elf64,
            data_encoding: ElfDataEncoding::LittleEndian,
            version: ElfIdentifierVersion::Current,
            os_abi: ElfOSABI::NoneSystemV,
            abi_version: ElfByte(0),
            pad: [ElfByte(0); 7],
        };

        let le64_hdr =
            ElfHeader::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }> {
                identifier: le64_id.clone(),
                r#type: ElfType::Executable,
                machine: ElfMachine::X86_64,
                version: ElfVersion::Current,
                entrypoint: Some(ElfAddress::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0)),
                program_header_offset: Some(ElfOffset::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0)),
                section_header_offset: Some(ElfOffset::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0)),
                flags: ElfWord::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>(
                    0,
                ),
                // NOTE: No extra size, ends at the section name string table index
                header_size:
                    ElfHalfWord::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::LittleEndian as u8 }>(
                        ElfHeader::<
                            { ElfClass::Elf64 as u8 },
                            { ElfDataEncoding::LittleEndian as u8 },
                        >::SIZE
                            .try_into()
                            .unwrap(),
                    ),
                program_header_entry_size: ElfHalfWord::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                program_header_entry_count: ElfHalfWord::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                section_header_entry_size: ElfHalfWord::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                section_header_entry_count: ElfHalfWord::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                section_name_string_table_index: ElfHalfWord::<
                    { ElfClass::Elf64 as u8 },
                    { ElfDataEncoding::LittleEndian as u8 },
                >(0),
                data: vec![],
            };

        let mut le64_bytes = Vec::new();
        le64_hdr.to_writer(&mut le64_bytes).unwrap();
        let le64_header_read = ElfHeader::<
            { ElfClass::Elf64 as u8 },
            { ElfDataEncoding::LittleEndian as u8 },
        >::from_reader(&mut std::io::Cursor::new(&mut le64_bytes))
        .unwrap();
        assert_eq!(le64_header_read, le64_hdr);
    }

    #[test]
    fn test_elf_header_be64() {
        let be64_id = ElfHeaderIdentifier {
            magic: [ElfByte(0x7f), ElfByte(0x45), ElfByte(0x4c), ElfByte(0x46)],
            class: ElfClass::Elf64,
            data_encoding: ElfDataEncoding::BigEndian,
            version: ElfIdentifierVersion::Current,
            os_abi: ElfOSABI::NoneSystemV,
            abi_version: ElfByte(0),
            pad: [ElfByte(0); 7],
        };

        let be64_hdr = ElfHeader::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }> {
            identifier: be64_id.clone(),
            r#type: ElfType::Executable,
            machine: ElfMachine::X86_64,
            version: ElfVersion::Current,
            entrypoint: Some(ElfAddress::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0)),
            program_header_offset: Some(ElfOffset::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0)),
            section_header_offset: Some(ElfOffset::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0)),
            flags: ElfWord::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>(0),
            // NOTE: No extra size, ends at the section name string table index
            header_size: ElfHalfWord::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(
                ElfHeader::<{ ElfClass::Elf64 as u8 }, { ElfDataEncoding::BigEndian as u8 }>::SIZE
                    .try_into()
                    .unwrap(),
            ),
            program_header_entry_size: ElfHalfWord::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            program_header_entry_count: ElfHalfWord::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            section_header_entry_size: ElfHalfWord::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            section_header_entry_count: ElfHalfWord::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            section_name_string_table_index: ElfHalfWord::<
                { ElfClass::Elf64 as u8 },
                { ElfDataEncoding::BigEndian as u8 },
            >(0),
            data: vec![],
        };

        let mut be64_bytes = Vec::new();
        be64_hdr.to_writer(&mut be64_bytes).unwrap();
        let be64_header_read = ElfHeader::<
            { ElfClass::Elf64 as u8 },
            { ElfDataEncoding::BigEndian as u8 },
        >::from_reader(&mut std::io::Cursor::new(&mut be64_bytes))
        .unwrap();
        assert_eq!(be64_header_read, be64_hdr);
    }
}
