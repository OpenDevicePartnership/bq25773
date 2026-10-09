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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// BQ25773 errors
pub enum BQ25773Error<I2cError> {
    Bus(I2cError),
    RegisterSizeError,
}

const BQ_ADDR: u8 = 0x6B;
const LARGEST_REG_SIZE_BYTES: usize = 2;
const CHARGE_CURRENT_SCALING_5MOHM: u16 = 8;
const CHARGE_CURRENT_SCALING_2MOHM: u16 = 20;
const CHARGE_VOLTAGE_SCALING: u16 = 4;
/// Largest unshifted encoding in `CHARGE_CURRENT[13:3]`, not a physical current limit.
const CHARGE_CURRENT_ENCODING_MAX: u16 = 2047;
/// Largest unshifted encoding in `CHARGE_VOLTAGE[14:2]`, not a physical voltage limit.
const CHARGE_VOLTAGE_ENCODING_MAX: u16 = 8191;

#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(clippy::unreachable)]
#[allow(unsafe_code)]
mod device;

pub use crate::device::*;

/// BQ25773 interface, which takes an async I2C bus
pub struct DeviceInterface<I2c: embedded_hal_async::i2c::I2c> {
    /// embedded-hal-async compliant I2C bus
    pub i2c: I2c,
}

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::RegisterInterfaceBase for DeviceInterface<I2c> {
    type Error = BQ25773Error<I2c::Error>;
    type AddressType = u8;
}

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface for DeviceInterface<I2c> {
    async fn write_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
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
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
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

    /// Read the programmed charging current in milliamps without modifying any registers.
    ///
    /// Each register step represents 8 mA with a 5 milliohm charge sense resistor,
    /// or 20 mA with a 2 milliohm resistor, as configured by `RSNS_RSR`.
    ///
    /// # Errors
    ///
    /// Returns [`BQ25773Error::Bus`] if reading the sense resistor configuration or
    /// the charging current register fails.
    pub async fn get_charging_current(&mut self) -> Result<charger::MilliAmps, BQ25773Error<I2c::Error>> {
        let scaling_factor = self.charge_current_scaling_factor().await?;
        Ok(self.device.charge_current().read_async().await?.charge_current() * scaling_factor)
    }

    /// Read the programmed charging voltage in millivolts without modifying any registers.
    ///
    /// Each register step represents 4 mV.
    ///
    /// # Errors
    ///
    /// Returns [`BQ25773Error::Bus`] if reading the charging voltage register fails.
    pub async fn get_charging_voltage(&mut self) -> Result<charger::MilliVolts, BQ25773Error<I2c::Error>> {
        Ok(self.device.charge_voltage().read_async().await?.charge_voltage() * CHARGE_VOLTAGE_SCALING)
    }

    async fn charge_current_scaling_factor(&mut self) -> Result<u16, BQ25773Error<I2c::Error>> {
        Ok(match self.device.charge_option_1().read_async().await?.rsns_rsr() {
            ChargeSenseResistorRsr::FiveMilliOhms => CHARGE_CURRENT_SCALING_5MOHM,
            ChargeSenseResistorRsr::TwoMilliOhms => CHARGE_CURRENT_SCALING_2MOHM,
        })
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> charger::ErrorType for Bq25773<I2c> {
    type Error = BQ25773Error<I2c::Error>;
}

impl<I2c: embedded_hal_async::i2c::I2c> charger::Charger for Bq25773<I2c> {
    /// Set the charging current in milliamps and return the scaled register read-back.
    ///
    /// Quantizes down in 8 mA steps at 5 milliohms or 20 mA at 2 milliohms,
    /// reading resistor configuration each call. Positive unshifted encodings saturate to
    /// 1–2047, keeping sub-step requests nonzero and preventing overflow wrap.
    /// The device separately clamps nonzero settings to 128–16320 mA at
    /// 5 milliohms and a 30000 mA maximum at 2 milliohms. Zero disables charging.
    ///
    /// Returns actual read-back, not the request. Bus errors stop the sequence;
    /// read-back failure does not roll back a successful write.
    async fn charging_current(&mut self, current: charger::MilliAmps) -> Result<charger::MilliAmps, Self::Error> {
        let scaling_factor = self.charge_current_scaling_factor().await?;
        let encoding = if current == 0 {
            0
        } else {
            (current / scaling_factor).clamp(1, CHARGE_CURRENT_ENCODING_MAX)
        };

        self.device
            .charge_current()
            .write_async(|w| w.set_charge_current(encoding))
            .await?;
        Ok(self.device.charge_current().read_async().await?.charge_current() * scaling_factor)
    }

    /// Set the charging voltage in millivolts.
    ///
    /// Passing `0` does **not** request 0 V. The BQ25773 treats a zero write to
    /// `CHARGE_VOLTAGE()` as a command to leave the register unchanged and force
    /// `CHARGE_CURRENT()` to zero, which disables charging; see Table 7-15 of the
    /// datasheet. The value returned is therefore the voltage still programmed in
    /// the register, not the zero that was requested. Use [`charger::Charger::charging_current`]
    /// with `0` if disabling charge is what you mean.
    ///
    /// Quantizes down in 4 mV steps. Positive unshifted encodings saturate to 1–8191,
    /// keeping sub-step requests nonzero and preventing overflow wrap. The device
    /// separately clamps nonzero settings to 5000–23000 mV. Returns actual read-back,
    /// not the request. Bus errors stop the sequence; read-back failure does not
    /// roll back a successful write.
    async fn charging_voltage(&mut self, voltage: charger::MilliVolts) -> Result<charger::MilliVolts, Self::Error> {
        let encoding = if voltage == 0 {
            0
        } else {
            (voltage / CHARGE_VOLTAGE_SCALING).clamp(1, CHARGE_VOLTAGE_ENCODING_MAX)
        };
        self.device
            .charge_voltage()
            .write_async(|w| w.set_charge_voltage(encoding))
            .await?;
        self.get_charging_voltage().await
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use embedded_batteries_async::charger::Charger;
    use embedded_hal::i2c::ErrorKind;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    use super::*;

    /// Build the two `CHARGE_OPTION_1` bytes the mock should return for a given
    /// sense resistor selection.
    fn charge_option_1_bytes(resistor: ChargeSenseResistorRsr) -> [u8; 2] {
        let mut reg = ChargeOption1::default();
        reg.set_rsns_rsr(resistor);
        reg.into()
    }

    #[tokio::test]
    async fn read_chip_id() {
        let reg = ManufactureId::from([0x40]);
        let raw_reg: [u8; 1] = reg.into();
        let expectations = vec![Transaction::write_read(BQ_ADDR, vec![0x2E], vec![raw_reg[0]])];
        let i2c = Mock::new(&expectations);
        let mut bq = Device::new(DeviceInterface { i2c });

        bq.manufacture_id().read_async().await.unwrap();

        bq.free().i2c.done();
    }

    #[tokio::test]
    async fn disable_external_ilim_pin() {
        let expectations = vec![
            Transaction::write_read(BQ_ADDR, vec![0x32], vec![0xB7, 0xF3]),
            Transaction::write(BQ_ADDR, vec![0x32, 0x37, 0xF3]),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Device::new(DeviceInterface { i2c });

        bq.charge_option_2()
            .modify_async(|r| r.set_en_extilim(false))
            .await
            .unwrap();

        bq.free().i2c.done();
    }

    #[tokio::test]
    async fn charging_current_trait_test() {
        let raw_options_reg = charge_option_1_bytes(ChargeSenseResistorRsr::FiveMilliOhms);
        // Set charge current to 2000mA, i.e. field value 250 with an 8mA/LSB step.
        let raw_reg: [u8; 2] = ((2000u16 / CHARGE_CURRENT_SCALING_5MOHM) << 3).to_le_bytes();
        let expectations = vec![
            Transaction::write_read(BQ_ADDR, vec![0x30], raw_options_reg.to_vec()),
            Transaction::write(BQ_ADDR, vec![0x02, raw_reg[0], raw_reg[1]]),
            Transaction::write_read(BQ_ADDR, vec![0x02], vec![raw_reg[0], raw_reg[1]]),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq25773::new(i2c);

        let charge_current = bq.charging_current(2000).await.unwrap();

        // Be sure we get 2000mA back
        assert_eq!(charge_current, 2000);

        bq.device.free().i2c.done();
    }

    #[tokio::test]
    async fn charging_current_resistor_values() {
        for (resistor, requested_current, raw_current_reg, expected_current) in [
            (ChargeSenseResistorRsr::FiveMilliOhms, 2000, [0xD0, 0x07], 2000),
            (ChargeSenseResistorRsr::TwoMilliOhms, 2000, [0x20, 0x03], 2000),
            (ChargeSenseResistorRsr::FiveMilliOhms, 2019, [0xE0, 0x07], 2016),
            (ChargeSenseResistorRsr::TwoMilliOhms, 2019, [0x20, 0x03], 2000),
        ] {
            let raw_options_reg = charge_option_1_bytes(resistor);
            let expectations = vec![
                Transaction::write_read(BQ_ADDR, vec![0x30], raw_options_reg.to_vec()),
                Transaction::write(BQ_ADDR, vec![0x02, raw_current_reg[0], raw_current_reg[1]]),
                Transaction::write_read(BQ_ADDR, vec![0x02], vec![raw_current_reg[0], raw_current_reg[1]]),
            ];
            let i2c = Mock::new(&expectations);
            let mut bq = Bq25773::new(i2c);

            let charge_current = bq.charging_current(requested_current).await.unwrap();

            assert_eq!(
                charge_current, expected_current,
                "{resistor:?}, requested {requested_current} mA"
            );

            bq.device.free().i2c.done();
        }
    }

    #[tokio::test]
    async fn charging_voltage_trait_test() {
        // 3S battery at 4.2 V per cell.
        let raw_reg = ((12600u16 / CHARGE_VOLTAGE_SCALING) << 2).to_le_bytes();
        let expectations = vec![
            Transaction::write(BQ_ADDR, vec![0x04, raw_reg[0], raw_reg[1]]),
            Transaction::write_read(BQ_ADDR, vec![0x04], vec![raw_reg[0], raw_reg[1]]),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq25773::new(i2c);

        let charge_voltage = bq.charging_voltage(12600).await.unwrap();

        assert_eq!(charge_voltage, 12600);

        bq.device.free().i2c.done();
    }

    #[tokio::test]
    async fn charging_voltage_zero_does_not_set_zero_volts() {
        // Table 7-15: writing 0 leaves CHARGE_VOLTAGE() unchanged and forces
        // CHARGE_CURRENT() to zero, so the read-back reports the previously
        // programmed voltage rather than the 0 that was asked for.
        let previously_programmed = ((12600u16 / CHARGE_VOLTAGE_SCALING) << 2).to_le_bytes();
        let expectations = vec![
            Transaction::write(BQ_ADDR, vec![0x04, 0x00, 0x00]),
            Transaction::write_read(
                BQ_ADDR,
                vec![0x04],
                vec![previously_programmed[0], previously_programmed[1]],
            ),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq25773::new(i2c);

        let charge_voltage = bq.charging_voltage(0).await.unwrap();

        assert_eq!(charge_voltage, 12600);

        bq.device.free().i2c.done();
    }

    #[tokio::test]
    async fn get_charging_current_resistor_values() {
        for (resistor, raw_current_reg, expected_current) in [
            (ChargeSenseResistorRsr::FiveMilliOhms, [0x00, 0x00], 0),
            (ChargeSenseResistorRsr::TwoMilliOhms, [0x00, 0x00], 0),
            (ChargeSenseResistorRsr::FiveMilliOhms, [0x08, 0x00], 8),
            (ChargeSenseResistorRsr::TwoMilliOhms, [0x08, 0x00], 20),
            (ChargeSenseResistorRsr::FiveMilliOhms, [0xD0, 0x07], 2000),
            (ChargeSenseResistorRsr::TwoMilliOhms, [0x20, 0x03], 2000),
            (ChargeSenseResistorRsr::FiveMilliOhms, [0xF8, 0x3F], 16376),
            (ChargeSenseResistorRsr::TwoMilliOhms, [0xF8, 0x3F], 40940),
            (ChargeSenseResistorRsr::FiveMilliOhms, [0xFF, 0xFF], 16376),
            (ChargeSenseResistorRsr::TwoMilliOhms, [0xFF, 0xFF], 40940),
        ] {
            let raw_options_reg = charge_option_1_bytes(resistor);
            let expectations = vec![
                Transaction::write_read(BQ_ADDR, vec![0x30], raw_options_reg.to_vec()),
                Transaction::write_read(BQ_ADDR, vec![0x02], raw_current_reg.to_vec()),
            ];
            let i2c = Mock::new(&expectations);
            let mut bq = Bq25773::new(i2c);

            let charge_current = bq.get_charging_current().await.unwrap();

            assert_eq!(
                charge_current, expected_current,
                "{resistor:?}, raw register {raw_current_reg:?}"
            );

            bq.device.free().i2c.done();
        }
    }

    #[tokio::test]
    async fn get_charging_voltage_values() {
        for (raw_voltage_reg, expected_voltage) in [
            ([0x00, 0x00], 0),
            ([0x04, 0x00], 4),
            ([0x38, 0x31], 12600),
            ([0xFC, 0x7F], 32764),
            ([0xFF, 0xFF], 32764),
        ] {
            let expectations = vec![Transaction::write_read(BQ_ADDR, vec![0x04], raw_voltage_reg.to_vec())];
            let i2c = Mock::new(&expectations);
            let mut bq = Bq25773::new(i2c);

            let charge_voltage = bq.get_charging_voltage().await.unwrap();

            assert_eq!(charge_voltage, expected_voltage, "raw register {raw_voltage_reg:?}");

            bq.device.free().i2c.done();
        }
    }

    #[tokio::test]
    async fn get_charging_current_propagates_read_errors() {
        for expectations in [
            vec![Transaction::write_read(BQ_ADDR, vec![0x30], vec![0, 0]).with_error(ErrorKind::Other)],
            vec![
                Transaction::write_read(BQ_ADDR, vec![0x30], vec![0, 0]),
                Transaction::write_read(BQ_ADDR, vec![0x02], vec![0, 0]).with_error(ErrorKind::Other),
            ],
        ] {
            let i2c = Mock::new(&expectations);
            let mut bq = Bq25773::new(i2c);

            assert_eq!(
                bq.get_charging_current().await,
                Err(BQ25773Error::Bus(ErrorKind::Other))
            );

            bq.device.free().i2c.done();
        }
    }

    #[tokio::test]
    async fn get_charging_voltage_propagates_read_errors() {
        let expectations = vec![Transaction::write_read(BQ_ADDR, vec![0x04], vec![0, 0]).with_error(ErrorKind::Other)];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq25773::new(i2c);

        assert_eq!(
            bq.get_charging_voltage().await,
            Err(BQ25773Error::Bus(ErrorKind::Other))
        );

        bq.device.free().i2c.done();
    }

    mod charger_protection {
        #![deny(deprecated)]

        use super::{Bq25773, Mock, Transaction};
        use crate::AcocLimit::{self, OneHundredThirtyThreePercent, TwoHundredPercent as AcocTwoHundredPercent};
        use crate::BatdocVth::{self, ThreeHundredPercent, TwoHundredPercent};

        fn acoc_encoding(value: AcocLimit) -> u8 {
            match value {
                OneHundredThirtyThreePercent => 0,
                AcocTwoHundredPercent => 1,
            }
        }

        fn batdoc_encoding(value: BatdocVth) -> u8 {
            match value {
                TwoHundredPercent => 0,
                ThreeHundredPercent => 1,
            }
        }

        #[test]
        fn variant_imports_exhaustive_matches_conversions_and_debug() {
            for (selection, encoding, debug) in [
                (OneHundredThirtyThreePercent, 0, "OneHundredThirtyThreePercent"),
                (AcocTwoHundredPercent, 1, "TwoHundredPercent"),
            ] {
                assert_eq!(acoc_encoding(selection), encoding);
                assert_eq!(u8::from(selection), encoding);
                assert_eq!(AcocLimit::try_from(encoding), Ok(selection));
                assert_eq!(format!("{selection:?}"), debug);
            }
            for (selection, encoding, debug) in [
                (TwoHundredPercent, 0, "TwoHundredPercent"),
                (ThreeHundredPercent, 1, "ThreeHundredPercent"),
            ] {
                assert_eq!(batdoc_encoding(selection), encoding);
                assert_eq!(u8::from(selection), encoding);
                assert_eq!(BatdocVth::try_from(encoding), Ok(selection));
                assert_eq!(format!("{selection:?}"), debug);
            }
            for invalid in [2, u8::MAX] {
                assert!(AcocLimit::try_from(invalid).is_err());
                assert!(BatdocVth::try_from(invalid).is_err());
            }
        }

        // Qualified variants cover both enums exhaustively.
        fn batdoc_percent(selection: BatdocVth) -> u16 {
            match selection {
                BatdocVth::TwoHundredPercent => 200,
                BatdocVth::ThreeHundredPercent => 300,
            }
        }

        fn acoc_percent(selection: AcocLimit) -> u16 {
            match selection {
                AcocLimit::OneHundredThirtyThreePercent => 133,
                AcocLimit::TwoHundredPercent => 200,
            }
        }

        #[test]
        fn qualified_variants_match_documented_percentages() {
            assert_eq!(batdoc_percent(BatdocVth::TwoHundredPercent), 200);
            assert_eq!(batdoc_percent(BatdocVth::ThreeHundredPercent), 300);
            assert_eq!(acoc_percent(AcocLimit::OneHundredThirtyThreePercent), 133);
            assert_eq!(acoc_percent(AcocLimit::TwoHundredPercent), 200);
        }

        // SLUSEK7 table 7-47: BATDOC bit 0, ACOC bit 2, address 0x32.
        // Literal fixtures preserve unrelated bits; they do not model analog trip points.
        macro_rules! threshold_wire_test {
            ($name:ident, $setter:ident, $getter:ident, $selection:expr, $before:expr, $after:expr) => {
                #[tokio::test]
                async fn $name() {
                    let expectations = [
                        Transaction::write_read(0x6B, vec![0x32], vec![$before, 0xF3]),
                        Transaction::write(0x6B, vec![0x32, $after, 0xF3]),
                        Transaction::write_read(0x6B, vec![0x32], vec![$after, 0xF3]),
                    ];
                    let mut bq = Bq25773::new(Mock::new(&expectations));
                    bq.device
                        .charge_option_2()
                        .modify_async(|r| r.$setter($selection))
                        .await
                        .unwrap();
                    let read_back = bq.device.charge_option_2().read_async().await.unwrap();
                    bq.device.free().i2c.done();
                    assert_eq!(read_back.$getter(), $selection);
                }
            };
        }

        threshold_wire_test!(
            batdoc_200_percent_clears_bit_zero_preserving_other_fields,
            set_batdoc_vth,
            batdoc_vth,
            BatdocVth::TwoHundredPercent,
            0xB7,
            0xB6
        );
        threshold_wire_test!(
            batdoc_300_percent_sets_bit_zero_preserving_other_fields,
            set_batdoc_vth,
            batdoc_vth,
            BatdocVth::ThreeHundredPercent,
            0xB6,
            0xB7
        );
        threshold_wire_test!(
            acoc_133_percent_clears_bit_two_preserving_other_fields,
            set_acoc_vth,
            acoc_vth,
            AcocLimit::OneHundredThirtyThreePercent,
            0xB7,
            0xB3
        );
        threshold_wire_test!(
            acoc_200_percent_sets_bit_two_preserving_other_fields,
            set_acoc_vth,
            acoc_vth,
            AcocLimit::TwoHundredPercent,
            0xB3,
            0xB7
        );
    }

    // Oracles: TI SLUSEK7 tables 7-14/7-15 (current bits 13:3, voltage
    // bits 14:2), section 7.3.11 (8/20 mA steps), and the setter contract.
    // Every write/read byte below is a hand-derived literal, not a fieldset
    // encoding. Read-back is an I2C fixture, not an implementation of silicon
    // clamps; in particular no physical minimum is assumed for 2 milliohms.
    mod setter_regressions {
        use super::*;

        async fn check_current_request(
            options: [u8; 2],
            request: u16,
            written: [u8; 2],
            read_back: [u8; 2],
            acknowledged: u16,
        ) {
            let [low, high] = written;
            let expectations = [
                Transaction::write_read(0x6B, vec![0x30], options.to_vec()),
                Transaction::write(0x6B, vec![0x02, low, high]),
                Transaction::write_read(0x6B, vec![0x02], read_back.to_vec()),
            ];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            let result = bq.charging_current(request).await;
            bq.device.free().i2c.done();
            assert_eq!(result, Ok(acknowledged), "current request {request} mA");
        }

        async fn check_voltage_request(request: u16, written: [u8; 2], read_back: [u8; 2], acknowledged: u16) {
            let [low, high] = written;
            let expectations = [
                Transaction::write(0x6B, vec![0x04, low, high]),
                Transaction::write_read(0x6B, vec![0x04], read_back.to_vec()),
            ];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            let result = bq.charging_voltage(request).await;
            bq.device.free().i2c.done();
            assert_eq!(result, Ok(acknowledged), "voltage request {request} mV");
        }

        macro_rules! current_cases {
            ($name:ident, $options:expr, $cases:expr) => {
                #[tokio::test]
                async fn $name() {
                    for (request, written, read_back, acknowledged) in $cases {
                        check_current_request($options, request, written, read_back, acknowledged).await;
                    }
                }
            };
        }

        macro_rules! voltage_cases {
            ($name:ident, $cases:expr) => {
                #[tokio::test]
                async fn $name() {
                    for (request, written, read_back, acknowledged) in $cases {
                        check_voltage_request(request, written, read_back, acknowledged).await;
                    }
                }
            };
        }

        // RSNS_RSR is bit 2 of address 0x31, hence byte 1 of the 0x30 read.
        current_cases!(
            current_zero_remains_zero_5mohm,
            [0x00, 0x00],
            [(0, [0x00, 0x00], [0x00, 0x00], 0),]
        );
        current_cases!(
            current_zero_remains_zero_2mohm,
            [0x00, 0x04],
            [(0, [0x00, 0x00], [0x00, 0x00], 0),]
        );
        current_cases!(
            current_positive_sub_lsb_never_disables_5mohm,
            [0x00, 0x00],
            [
                (1, [0x08, 0x00], [0x80, 0x00], 128),
                (7, [0x08, 0x00], [0x80, 0x00], 128),
            ]
        );
        // 2000 mA here is only an illustrative acknowledged value. The test
        // requires decoding the supplied reply, not a claimed 2mOhm minimum.
        current_cases!(
            current_positive_sub_lsb_never_disables_2mohm,
            [0x00, 0x04],
            [
                (1, [0x08, 0x00], [0x20, 0x03], 2000),
                (19, [0x08, 0x00], [0x20, 0x03], 2000),
            ]
        );
        current_cases!(
            current_largest_sub_lsb_never_disables_5mohm,
            [0x00, 0x00],
            [(7, [0x08, 0x00], [0x80, 0x00], 128),]
        );
        current_cases!(
            current_largest_sub_lsb_never_disables_2mohm,
            [0x00, 0x04],
            [(19, [0x08, 0x00], [0x20, 0x03], 2000),]
        );
        current_cases!(
            current_quantizes_down_5mohm,
            [0x00, 0x00],
            [
                (8, [0x08, 0x00], [0x80, 0x00], 128),
                (15, [0x08, 0x00], [0x80, 0x00], 128),
                (16, [0x10, 0x00], [0x80, 0x00], 128),
                (2019, [0xE0, 0x07], [0xE0, 0x07], 2016),
            ]
        );
        current_cases!(
            current_quantizes_down_2mohm,
            [0x00, 0x04],
            [
                (20, [0x08, 0x00], [0x20, 0x03], 2000),
                (39, [0x08, 0x00], [0x20, 0x03], 2000),
                (40, [0x10, 0x00], [0x20, 0x03], 2000),
                (2019, [0x20, 0x03], [0x20, 0x03], 2000),
            ]
        );
        current_cases!(
            current_physical_clamp_boundaries_5mohm,
            [0x00, 0x00],
            [
                (127, [0x78, 0x00], [0x80, 0x00], 128),
                (128, [0x80, 0x00], [0x80, 0x00], 128),
                (135, [0x80, 0x00], [0x80, 0x00], 128),
                (136, [0x88, 0x00], [0x88, 0x00], 136),
                (16319, [0xB8, 0x3F], [0xB8, 0x3F], 16312),
                (16320, [0xC0, 0x3F], [0xC0, 0x3F], 16320),
                (16327, [0xC0, 0x3F], [0xC0, 0x3F], 16320),
                (16328, [0xC8, 0x3F], [0xC0, 0x3F], 16320),
            ]
        );
        current_cases!(
            current_physical_upper_clamp_boundaries_2mohm,
            [0x00, 0x04],
            [
                (29999, [0xD8, 0x2E], [0xD8, 0x2E], 29980),
                (30000, [0xE0, 0x2E], [0xE0, 0x2E], 30000),
                (30019, [0xE0, 0x2E], [0xE0, 0x2E], 30000),
                (30020, [0xE8, 0x2E], [0xE0, 0x2E], 30000),
            ]
        );
        current_cases!(
            current_last_representable_values_5mohm,
            [0x00, 0x00],
            [
                (16368, [0xF0, 0x3F], [0xC0, 0x3F], 16320),
                (16376, [0xF8, 0x3F], [0xC0, 0x3F], 16320),
                (16383, [0xF8, 0x3F], [0xC0, 0x3F], 16320),
            ]
        );
        current_cases!(
            current_last_representable_values_2mohm,
            [0x00, 0x04],
            [
                (40920, [0xF0, 0x3F], [0xE0, 0x2E], 30000),
                (40940, [0xF8, 0x3F], [0xE0, 0x2E], 30000),
                (40959, [0xF8, 0x3F], [0xE0, 0x2E], 30000),
            ]
        );
        current_cases!(
            current_overflow_saturates_5mohm,
            [0x00, 0x00],
            [
                (16384, [0xF8, 0x3F], [0xC0, 0x3F], 16320),
                (16385, [0xF8, 0x3F], [0xC0, 0x3F], 16320),
                (32768, [0xF8, 0x3F], [0xC0, 0x3F], 16320),
            ]
        );
        current_cases!(
            current_overflow_saturates_2mohm,
            [0x00, 0x04],
            [
                (40960, [0xF8, 0x3F], [0xE0, 0x2E], 30000),
                (40961, [0xF8, 0x3F], [0xE0, 0x2E], 30000),
                (50000, [0xF8, 0x3F], [0xE0, 0x2E], 30000),
            ]
        );
        current_cases!(
            current_u16_max_saturates_5mohm,
            [0x00, 0x00],
            [(u16::MAX, [0xF8, 0x3F], [0xC0, 0x3F], 16320),]
        );
        current_cases!(
            current_overflow_does_not_wrap_to_lower_nonzero_5mohm,
            [0x00, 0x00],
            [(20000, [0xF8, 0x3F], [0xC0, 0x3F], 16320),]
        );
        current_cases!(
            current_overflow_does_not_wrap_to_lower_nonzero_2mohm,
            [0x00, 0x04],
            [(50000, [0xF8, 0x3F], [0xE0, 0x2E], 30000),]
        );
        current_cases!(
            current_u16_max_saturates_2mohm,
            [0x00, 0x04],
            [(u16::MAX, [0xF8, 0x3F], [0xE0, 0x2E], 30000),]
        );
        current_cases!(
            current_returns_acknowledged_not_requested_5mohm,
            [0x00, 0x00],
            [(2000, [0xD0, 0x07], [0x00, 0x04], 1024),]
        );
        current_cases!(
            current_returns_acknowledged_not_requested_2mohm,
            [0x00, 0x04],
            [(2000, [0x20, 0x03], [0x90, 0x01], 1000),]
        );

        voltage_cases!(
            voltage_zero_preserves_acknowledged_voltage,
            [(0, [0x00, 0x00], [0x38, 0x31], 12600),]
        );
        voltage_cases!(
            voltage_positive_sub_lsb_never_disables,
            [
                (1, [0x04, 0x00], [0x88, 0x13], 5000),
                (3, [0x04, 0x00], [0x88, 0x13], 5000),
            ]
        );
        voltage_cases!(
            voltage_largest_sub_lsb_never_disables,
            [(3, [0x04, 0x00], [0x88, 0x13], 5000),]
        );
        voltage_cases!(
            voltage_quantizes_down,
            [
                (4, [0x04, 0x00], [0x88, 0x13], 5000),
                (7, [0x04, 0x00], [0x88, 0x13], 5000),
                (8, [0x08, 0x00], [0x88, 0x13], 5000),
                (12603, [0x38, 0x31], [0x38, 0x31], 12600),
            ]
        );
        voltage_cases!(
            voltage_physical_clamp_boundaries,
            [
                (4999, [0x84, 0x13], [0x88, 0x13], 5000),
                (5000, [0x88, 0x13], [0x88, 0x13], 5000),
                (5003, [0x88, 0x13], [0x88, 0x13], 5000),
                (5004, [0x8C, 0x13], [0x8C, 0x13], 5004),
                (22999, [0xD4, 0x59], [0xD4, 0x59], 22996),
                (23000, [0xD8, 0x59], [0xD8, 0x59], 23000),
                (23003, [0xD8, 0x59], [0xD8, 0x59], 23000),
                (23004, [0xDC, 0x59], [0xD8, 0x59], 23000),
            ]
        );
        voltage_cases!(
            voltage_last_representable_values,
            [
                (32760, [0xF8, 0x7F], [0xD8, 0x59], 23000),
                (32764, [0xFC, 0x7F], [0xD8, 0x59], 23000),
                (32767, [0xFC, 0x7F], [0xD8, 0x59], 23000),
            ]
        );
        voltage_cases!(
            voltage_overflow_saturates,
            [
                (32768, [0xFC, 0x7F], [0xD8, 0x59], 23000),
                (32769, [0xFC, 0x7F], [0xD8, 0x59], 23000),
                (40000, [0xFC, 0x7F], [0xD8, 0x59], 23000),
            ]
        );
        voltage_cases!(
            voltage_u16_max_saturates,
            [(u16::MAX, [0xFC, 0x7F], [0xD8, 0x59], 23000),]
        );
        voltage_cases!(
            voltage_overflow_does_not_wrap_to_lower_nonzero,
            [(40000, [0xFC, 0x7F], [0xD8, 0x59], 23000),]
        );
        voltage_cases!(
            voltage_returns_acknowledged_not_requested,
            [(12600, [0x38, 0x31], [0x40, 0x1F], 8000),]
        );

        #[tokio::test]
        async fn current_increasing_requests_do_not_wrap_5mohm() {
            // One driver, fixed scaling, increasing requests across the boundary.
            let expectations = [
                Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x00]),
                Transaction::write(0x6B, vec![0x02, 0xF8, 0x3F]),
                Transaction::write_read(0x6B, vec![0x02], vec![0xC0, 0x3F]),
                Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x00]),
                Transaction::write(0x6B, vec![0x02, 0xF8, 0x3F]),
                Transaction::write_read(0x6B, vec![0x02], vec![0xC0, 0x3F]),
                Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x00]),
                Transaction::write(0x6B, vec![0x02, 0xF8, 0x3F]),
                Transaction::write_read(0x6B, vec![0x02], vec![0xC0, 0x3F]),
            ];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            assert_eq!(bq.charging_current(16383).await, Ok(16320));
            assert_eq!(bq.charging_current(16384).await, Ok(16320));
            assert_eq!(bq.charging_current(u16::MAX).await, Ok(16320));
            bq.device.free().i2c.done();
        }

        #[tokio::test]
        async fn current_increasing_requests_do_not_wrap_2mohm() {
            let expectations = [
                Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x04]),
                Transaction::write(0x6B, vec![0x02, 0xF8, 0x3F]),
                Transaction::write_read(0x6B, vec![0x02], vec![0xE0, 0x2E]),
                Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x04]),
                Transaction::write(0x6B, vec![0x02, 0xF8, 0x3F]),
                Transaction::write_read(0x6B, vec![0x02], vec![0xE0, 0x2E]),
                Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x04]),
                Transaction::write(0x6B, vec![0x02, 0xF8, 0x3F]),
                Transaction::write_read(0x6B, vec![0x02], vec![0xE0, 0x2E]),
            ];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            for request in [40959, 40960, u16::MAX] {
                assert_eq!(bq.charging_current(request).await, Ok(30000));
            }
            bq.device.free().i2c.done();
        }

        #[tokio::test]
        async fn current_rechecks_resistor_configuration_between_calls() {
            let expectations = [
                Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x00]),
                Transaction::write(0x6B, vec![0x02, 0xD0, 0x07]),
                Transaction::write_read(0x6B, vec![0x02], vec![0xD0, 0x07]),
                Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x04]),
                Transaction::write(0x6B, vec![0x02, 0x20, 0x03]),
                Transaction::write_read(0x6B, vec![0x02], vec![0x20, 0x03]),
            ];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            assert_eq!(bq.charging_current(2000).await, Ok(2000));
            assert_eq!(bq.charging_current(2000).await, Ok(2000));
            bq.device.free().i2c.done();
        }

        #[tokio::test]
        async fn voltage_increasing_requests_do_not_wrap() {
            let expectations = [
                Transaction::write(0x6B, vec![0x04, 0xFC, 0x7F]),
                Transaction::write_read(0x6B, vec![0x04], vec![0xD8, 0x59]),
                Transaction::write(0x6B, vec![0x04, 0xFC, 0x7F]),
                Transaction::write_read(0x6B, vec![0x04], vec![0xD8, 0x59]),
                Transaction::write(0x6B, vec![0x04, 0xFC, 0x7F]),
                Transaction::write_read(0x6B, vec![0x04], vec![0xD8, 0x59]),
            ];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            for request in [32767, 32768, u16::MAX] {
                assert_eq!(bq.charging_voltage(request).await, Ok(23000));
            }
            bq.device.free().i2c.done();
        }

        fn assert_bus_error(result: Result<u16, BQ25773Error<ErrorKind>>, expected: ErrorKind) {
            assert_eq!(result, Err(BQ25773Error::Bus(expected)));
            assert_eq!(
                charger::Error::kind(&result.unwrap_err()),
                charger::ErrorKind::CommError
            );
        }

        #[tokio::test]
        async fn current_stops_at_configuration_error() {
            let expectations = [Transaction::write_read(0x6B, vec![0x30], vec![0x00, 0x00]).with_error(ErrorKind::Bus)];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            let result = bq.charging_current(2000).await;
            bq.device.free().i2c.done();
            assert_bus_error(result, ErrorKind::Bus);
        }

        #[tokio::test]
        async fn current_stops_at_write_error() {
            for (options, written) in [([0x00, 0x00], [0xD0, 0x07]), ([0x00, 0x04], [0x20, 0x03])] {
                let [low, high] = written;
                let expectations = [
                    Transaction::write_read(0x6B, vec![0x30], options.to_vec()),
                    Transaction::write(0x6B, vec![0x02, low, high]).with_error(ErrorKind::ArbitrationLoss),
                ];
                let mut bq = Bq25773::new(Mock::new(&expectations));
                let result = bq.charging_current(2000).await;
                bq.device.free().i2c.done();
                assert_bus_error(result, ErrorKind::ArbitrationLoss);
            }
        }

        #[tokio::test]
        async fn current_stops_at_read_back_error_without_rollback() {
            for (options, written) in [([0x00, 0x00], [0xD0, 0x07]), ([0x00, 0x04], [0x20, 0x03])] {
                let [low, high] = written;
                let expectations = [
                    Transaction::write_read(0x6B, vec![0x30], options.to_vec()),
                    Transaction::write(0x6B, vec![0x02, low, high]),
                    Transaction::write_read(0x6B, vec![0x02], vec![0x00, 0x00]).with_error(ErrorKind::Other),
                ];
                let mut bq = Bq25773::new(Mock::new(&expectations));
                let result = bq.charging_current(2000).await;
                bq.device.free().i2c.done();
                assert_bus_error(result, ErrorKind::Other);
            }
        }

        #[tokio::test]
        async fn voltage_stops_at_write_error() {
            let expectations = [Transaction::write(0x6B, vec![0x04, 0x38, 0x31]).with_error(ErrorKind::Bus)];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            let result = bq.charging_voltage(12600).await;
            bq.device.free().i2c.done();
            assert_bus_error(result, ErrorKind::Bus);
        }

        #[tokio::test]
        async fn voltage_stops_at_read_back_error_without_rollback() {
            let expectations = [
                Transaction::write(0x6B, vec![0x04, 0x38, 0x31]),
                Transaction::write_read(0x6B, vec![0x04], vec![0x00, 0x00]).with_error(ErrorKind::Other),
            ];
            let mut bq = Bq25773::new(Mock::new(&expectations));
            let result = bq.charging_voltage(12600).await;
            bq.device.free().i2c.done();
            assert_bus_error(result, ErrorKind::Other);
        }
    }
}
