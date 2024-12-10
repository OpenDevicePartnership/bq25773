//! This is a platform-agnostic Rust driver for the Texas Instruments BQ25773 Battery
//! Charger IC based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//!
//! For further details of the device architecture and operation, please refer
//! to the official [`Datasheet`].
//!
//! [`Datasheet`]: https://www.ti.com/lit/ds/symlink/bq25773.pdf

#![doc = include_str!("../README.md")]
#![no_std]
#![allow(missing_docs)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum BQ25773Error<I2cError> {
    I2c(I2cError),
}

const BQ_ADDR: u8 = 0x6B;
const LARGEST_REG_SIZE_BYTES: usize = 2;

pub struct DeviceInterface<I2c: embedded_hal_async::i2c::I2c> {
    pub i2c: I2c,
}

device_driver::create_device!(
    device_name: Device,
    manifest: "device.yaml"
);

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface for DeviceInterface<I2c> {
    type Error = BQ25773Error<I2c::Error>;
    type AddressType = u8;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        assert!((data.len() <= LARGEST_REG_SIZE_BYTES), "Register size too big");

        // Add one byte for register address
        let mut buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES];
        buf[0] = address;
        buf[1..=data.len()].copy_from_slice(data);

        // Because the BQ25773 has a mix of 1 byte and 2 byte registers that can be written to,
        // we pass in a slice of the appropriate size so we do not accidentally write to the register at
        // address + 1 when writing to a 1 byte register
        self.i2c
            .write(BQ_ADDR, &buf[..=data.len()])
            .await
            .map_err(BQ25773Error::I2c)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(BQ_ADDR, &[address], data)
            .await
            .map_err(BQ25773Error::I2c)
    }
}
