//! A thin hid wrapper layer which supports write/read HID reports via USB and BLE

use embassy_usb::{
    class::hid::{HidReader, HidReaderWriter, HidWriter, ReadError},
    driver::Driver,
};
use usbd_hid::descriptor::AsInputReport;

pub(crate) enum ConnectionType {
    USB,
    BLE,
}

pub(crate) trait ConnectionTypeWrapper {
    fn get_conn_type(&self) -> ConnectionType;
}

/// Wrapper trait for hid reading
pub(crate) trait HidReaderWrapper: ConnectionTypeWrapper {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, ReadError>;
}

/// Wrapper trait for hid writing
pub(crate) trait HidWriterWrapper: ConnectionTypeWrapper {
    async fn write_serialize<IR: AsInputReport>(&mut self, r: &IR) -> Result<(), ()>;
    async fn write(&mut self, report: &[u8]) -> Result<(), ()>;
}

pub(crate) trait HidReaderWriterWrapper: HidReaderWrapper + HidWriterWrapper {}

/// Wrapper struct for writing via USB
pub(crate) struct UsbHidWriter<'d, D: Driver<'d>, const N: usize> {
    usb_writer: HidWriter<'d, D, N>,
}

impl<'d, D: Driver<'d>, const N: usize> ConnectionTypeWrapper for UsbHidWriter<'d, D, N> {
    fn get_conn_type(&self) -> ConnectionType {
        ConnectionType::USB
    }
}

impl<'d, D: Driver<'d>, const N: usize> HidWriterWrapper for UsbHidWriter<'d, D, N> {
    async fn write_serialize<IR: AsInputReport>(&mut self, r: &IR) -> Result<(), ()> {
        self.usb_writer.write_serialize(r).await.map_err(|_e| ())
    }

    async fn write(&mut self, report: &[u8]) -> Result<(), ()> {
        self.usb_writer.write(report).await.map_err(|_e| ())
    }
}

impl<'d, D: Driver<'d>, const N: usize> UsbHidWriter<'d, D, N> {
    pub(crate) fn new(usb_writer: HidWriter<'d, D, N>) -> Self {
        Self { usb_writer }
    }
}

/// Wrapper struct for reading via USB
pub(crate) struct UsbHidReader<'d, D: Driver<'d>, const N: usize> {
    usb_reader: HidReader<'d, D, N>,
}

impl<'d, D: Driver<'d>, const N: usize> ConnectionTypeWrapper for UsbHidReader<'d, D, N> {
    fn get_conn_type(&self) -> ConnectionType {
        ConnectionType::USB
    }
}

impl<'d, D: Driver<'d>, const N: usize> HidReaderWrapper for UsbHidReader<'d, D, N> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, ReadError> {
        self.usb_reader.read(buf).await
    }
}

impl<'d, D: Driver<'d>, const N: usize> UsbHidReader<'d, D, N> {
    pub(crate) fn new(usb_reader: HidReader<'d, D, N>) -> Self {
        Self { usb_reader }
    }
}

/// Wrapper struct for reading and writing via USB
pub(crate) struct UsbHidReaderWriter<'d, D: Driver<'d>, const READ_N: usize, const WRITE_N: usize> {
    usb_reader_writer: HidReaderWriter<'d, D, READ_N, WRITE_N>,
}

impl<'d, D: Driver<'d>, const READ_N: usize, const WRITE_N: usize>
    UsbHidReaderWriter<'d, D, READ_N, WRITE_N>
{
    pub(crate) fn new(usb_reader_writer: HidReaderWriter<'d, D, READ_N, WRITE_N>) -> Self {
        Self { usb_reader_writer }
    }
}

impl<'d, D: Driver<'d>, const READ_N: usize, const WRITE_N: usize> HidReaderWriterWrapper
    for UsbHidReaderWriter<'d, D, READ_N, WRITE_N>
{
}

impl<'d, D: Driver<'d>, const READ_N: usize, const WRITE_N: usize> ConnectionTypeWrapper
    for UsbHidReaderWriter<'d, D, READ_N, WRITE_N>
{
    fn get_conn_type(&self) -> ConnectionType {
        ConnectionType::USB
    }
}

impl<'d, D: Driver<'d>, const READ_N: usize, const WRITE_N: usize> HidReaderWrapper
    for UsbHidReaderWriter<'d, D, READ_N, WRITE_N>
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, ReadError> {
        self.usb_reader_writer.read(buf).await
    }
}

impl<'d, D: Driver<'d>, const READ_N: usize, const WRITE_N: usize> HidWriterWrapper
    for UsbHidReaderWriter<'d, D, READ_N, WRITE_N>
{
    async fn write_serialize<IR: AsInputReport>(&mut self, r: &IR) -> Result<(), ()> {
        self.usb_reader_writer
            .write_serialize(r)
            .await
            .map_err(|_e| ())
    }

    async fn write(&mut self, report: &[u8]) -> Result<(), ()> {
        self.usb_reader_writer.write(report).await.map_err(|_e| ())
    }
}