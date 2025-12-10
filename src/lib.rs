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

use embedded_batteries_async::charger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
/// BQ25773 errors
pub enum BQ25773Error<I2cError> {
    Bus(I2cError),
    RegisterSizeError,
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
        // Add one byte for register address
        let mut buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES];
        buf[0] = address;

        // Check if buffer size is big enough to hold data.len(), otherwise return an error.
        buf.get_mut(1..=data.len())
            .ok_or(BQ25773Error::RegisterSizeError)?
            .copy_from_slice(data);

        // Because the BQ25773 has a mix of 1 byte and 2 byte registers that can be written to,
        // we pass in a slice of the appropriate size so we do not accidentally write to the register at
        // address + 1 when writing to a 1 byte register
        self.i2c
            .write(BQ_ADDR, buf.get(..=data.len()).ok_or(BQ25773Error::RegisterSizeError)?)
            .await
            .map_err(BQ25773Error::Bus)
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
            .map_err(BQ25773Error::Bus)
    }
}

impl<E: embedded_hal_async::i2c::Error> charger::Error for BQ25773Error<E> {
    fn kind(&self) -> charger::ErrorKind {
        match self {
            Self::Bus(_) => charger::ErrorKind::CommError,
            Self::RegisterSizeError => charger::ErrorKind::Other,
        }
    }
}

pub struct Bq25773<I2c: embedded_hal_async::i2c::I2c> {
    pub device: Device<DeviceInterface<I2c>>,
}

impl<I2c: embedded_hal_async::i2c::I2c> Bq25773<I2c> {
    pub fn new(i2c: I2c) -> Self {
        Bq25773 {
            device: Device::new(DeviceInterface { i2c }),
        }
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> charger::ErrorType for Bq25773<I2c> {
    type Error = BQ25773Error<I2c::Error>;
}

impl<I2c: embedded_hal_async::i2c::I2c> charger::Charger for Bq25773<I2c> {
    async fn charging_current(&mut self, current: charger::MilliAmps) -> Result<charger::MilliAmps, Self::Error> {
        self.device
            .charge_current()
            .write_async(|w| w.set_charge_current(current))
            .await?;
        Ok(self.device.charge_current().read_async().await?.charge_current())
    }

    async fn charging_voltage(&mut self, voltage: charger::MilliVolts) -> Result<charger::MilliVolts, Self::Error> {
        self.device
            .charge_voltage()
            .write_async(|w| w.set_charge_voltage(voltage))
            .await?;
        Ok(self.device.charge_voltage().read_async().await?.charge_voltage())
    }
}

#[cfg(test)]
mod tests {
    use embedded_batteries_async::charger::Charger;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use field_sets::{ChargeCurrent, ChargeOption2A, ManufactureId};

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

    #[tokio::test]
    async fn charging_current_trait_test() {
        let mut reg = ChargeCurrent::new();
        // Set charge current to 2000mA
        reg.set_charge_current(2000);
        let raw_reg_2a: [u8; 2] = reg.into();
        let expectations = vec![
            Transaction::write(BQ_ADDR, vec![0x02, raw_reg_2a[0], raw_reg_2a[1]]),
            Transaction::write_read(BQ_ADDR, vec![0x02], vec![raw_reg_2a[0], raw_reg_2a[1]]),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq25773::new(i2c);

        let charge_current = bq.charging_current(2000).await.unwrap();

        // Be sure we get 2000mA back
        assert_eq!(charge_current, 2000);

        bq.device.interface.i2c.done();
    }
}
