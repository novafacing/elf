use crate::{ElfClass, ElfDataEncoding, Result};
use std::io::{Read, Write};

pub trait FromReader<const EC: u8, const ED: u8, R>
where
    R: Read,
    Self: Sized,
{
    fn from_reader(reader: &mut R) -> Result<Self>;
}

pub trait FromReader32LSB<R>
where
    R: Read,
    Self: Sized,
    Self: FromReader<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, R>,
{
    fn from_reader_32_lsb(reader: &mut R) -> Result<Self> {
        <Self as FromReader<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, R>>::from_reader(reader)
    }
}

pub trait FromReader32MSB<R>
where
    R: Read,
    Self: Sized,
    Self: FromReader<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, R>,
{
    fn from_reader_32_msb(reader: &mut R) -> Result<Self> {
        <Self as FromReader<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, R>>::from_reader(reader)
    }
}

pub trait FromReader64LSB<R>
where
    R: Read,
    Self: Sized,
    Self: FromReader<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, R>,
{
    fn from_reader_64_lsb(reader: &mut R) -> Result<Self> {
        <Self as FromReader<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, R>>::from_reader(reader)
    }
}

pub trait FromReader64MSB<R>
where
    R: Read,
    Self: Sized,
    Self: FromReader<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, R>,
{
    fn from_reader_64_msb(reader: &mut R) -> Result<Self> {
        <Self as FromReader<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, R>>::from_reader(reader)
    }
}

pub trait FromReaderHost<R>
where
    R: Read,
    Self: Sized,
    Self: FromReader<{ ElfClass::HOST }, { ElfDataEncoding::HOST }, R>,
{
    fn from_reader_host(reader: &mut R) -> Result<Self> {
        <Self as FromReader<{ ElfClass::HOST }, { ElfDataEncoding::HOST }, R>>::from_reader(reader)
    }
}

impl<T, R> FromReader32LSB<R> for T
where
    R: Read,
    T: FromReader<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, R>,
{
}

impl<T, R> FromReader32MSB<R> for T
where
    R: Read,
    T: FromReader<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, R>,
{
}

impl<T, R> FromReader64LSB<R> for T
where
    R: Read,
    T: FromReader<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, R>,
{
}

impl<T, R> FromReader64MSB<R> for T
where
    R: Read,
    T: FromReader<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, R>,
{
}

impl<T, R> FromReaderHost<R> for T
where
    R: Read,
    T: FromReader<{ ElfClass::HOST }, { ElfDataEncoding::HOST }, R>,
{
}

pub trait ToWriter<const EC: u8, const ED: u8, W>
where
    W: Write,
    Self: Sized,
{
    fn to_writer(&self, writer: &mut W) -> Result<()>;
}

pub trait ToWriter32LSB<W>
where
    W: Write,
    Self: Sized,
    Self: ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, W>,
{
    fn to_writer_32_lsb(&self, writer: &mut W) -> Result<()> {
        <Self as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, W>>::to_writer(
            self, writer,
        )
    }
}

pub trait ToWriter32MSB<W>
where
    W: Write,
    Self: Sized,
    Self: ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, W>,
{
    fn to_writer_32_msb(&self, writer: &mut W) -> Result<()> {
        <Self as ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, W>>::to_writer(
            self, writer,
        )
    }
}

pub trait ToWriter64LSB<W>
where
    W: Write,
    Self: Sized,
    Self: ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, W>,
{
    fn to_writer_64_lsb(&self, writer: &mut W) -> Result<()> {
        <Self as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, W>>::to_writer(
            self, writer,
        )
    }
}

pub trait ToWriter64MSB<W>
where
    W: Write,
    Self: Sized,
    Self: ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, W>,
{
    fn to_writer_64_msb(&self, writer: &mut W) -> Result<()> {
        <Self as ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, W>>::to_writer(
            self, writer,
        )
    }
}

pub trait ToWriterHost<W>
where
    W: Write,
    Self: Sized,
    Self: ToWriter<{ ElfClass::HOST }, { ElfDataEncoding::HOST }, W>,
{
    fn to_writer_host(&self, writer: &mut W) -> Result<()> {
        <Self as ToWriter<{ ElfClass::HOST }, { ElfDataEncoding::HOST }, W>>::to_writer(
            self, writer,
        )
    }
}

impl<T, W> ToWriter64LSB<W> for T
where
    W: Write,
    T: ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::LSB }, W>,
{
}

impl<T, W> ToWriter64MSB<W> for T
where
    W: Write,
    T: ToWriter<{ ElfClass::ELF64 }, { ElfDataEncoding::MSB }, W>,
{
}

impl<T, W> ToWriter32LSB<W> for T
where
    W: Write,
    T: ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::LSB }, W>,
{
}

impl<T, W> ToWriter32MSB<W> for T
where
    W: Write,
    T: ToWriter<{ ElfClass::ELF32 }, { ElfDataEncoding::MSB }, W>,
{
}

impl<T, W> ToWriterHost<W> for T
where
    W: Write,
    T: ToWriter<{ ElfClass::HOST }, { ElfDataEncoding::HOST }, W>,
{
}
