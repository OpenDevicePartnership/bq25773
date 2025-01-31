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
#![cfg_attr(not(test), no_std)]
#![allow(missing_docs)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
/// BQ25773 errors
pub enum BQ25773Error<I2cError> {
    I2c(I2cError),
}

const BQ_ADDR: u8 = 0x6B;
const LARGEST_REG_SIZE_BYTES: usize = 2;

/// BQ25773 interface, which takes an async I2C bus
pub struct DeviceInterface<I2c: embedded_hal_async::i2c::I2c> {
    /// embedded-hal-async compliant I2C bus
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
        debug_assert!((data.len() <= LARGEST_REG_SIZE_BYTES), "Register size too big");

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

#[cfg(test)]
mod tests {
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use field_sets::{ChargeOption2A, ManufactureId};

    use super::*;

    #[tokio::test]
    async fn read_chip_id() {
        let reg = ManufactureId::new();
        let raw_reg: [u8; 1] = reg.into();
        let expectations = vec![Transaction::write_read(BQ_ADDR, vec![0x2E], vec![raw_reg[0]])];
        let i2c = Mock::new(&expectations);
        let mut bq = Device::new(DeviceInterface { i2c });

        bq.manufacture_id().read_async().await.unwrap();

        bq.interface.i2c.done();
    }

    #[tokio::test]
    async fn disable_external_ilim_pin() {
        let mut reg = ChargeOption2A::new();
        let raw_reg: [u8; 1] = reg.into();
        reg.set_en_extilim(false);
        let raw_reg_ilim_disabled: [u8; 1] = reg.into();
        let expectations = vec![
            Transaction::write_read(BQ_ADDR, vec![0x32], vec![raw_reg[0]]),
            Transaction::write(BQ_ADDR, vec![0x32, raw_reg_ilim_disabled[0]]),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Device::new(DeviceInterface { i2c });

        bq.charge_option_2_a()
            .modify_async(|r| r.set_en_extilim(false))
            .await
            .unwrap();

        bq.interface.i2c.done();
    }
}
