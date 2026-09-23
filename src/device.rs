// This code was generated using device-driver `2.1.1` (),
// a tool distributed under MIT OR Apache-2.0 by Dion Dokter <dev@diondokter.nl>
//
// For more information about device-driver, visit the website: https://device-driver.com

/// Root block of the Device driver
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Device<I> {
    interface: I,
    #[doc(hidden)]
    #[allow(unused)]
    base_address: u8,
}
impl<I> Device<I> {
    /// Create a new instance of the device
    pub const fn new(interface: I) -> Self {
        Self {
            interface,
            base_address: 0,
        }
    }
    /// Drop the driver instance and reclaim the interface
    pub fn free(self) -> I {
        self.interface
    }
    /// Charge option 0 at 0x00-0x01.
    ///
    /// Register operation:
    /// - Address: `0`
    /// - Reset value: `0xE70E`
    #[doc(alias = "CHARGE_OPTION_0")]
    pub fn charge_option_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeOption0, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeOption0::from([14, 231]))
    }
    /// Register operation:
    /// - Address: `2`
    /// - Reset value: `0`
    #[doc(alias = "CHARGE_CURRENT")]
    pub fn charge_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeCurrent, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeCurrent::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `4`
    /// - Reset value: `0`
    #[doc(alias = "CHARGE_VOLTAGE")]
    pub fn charge_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeVoltage, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeVoltage::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `6`
    /// - Reset value: `800`
    #[doc(alias = "IIN_HOST")]
    pub fn iin_host(&mut self) -> ::device_driver::RegisterOperation<'_, Self, IinHost, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::new(self, address as u8, || IinHost::from([32, 3]))
    }
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `640`
    #[doc(alias = "VINDPM")]
    pub fn vindpm(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Vindpm, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(self, address as u8, || Vindpm::from([128, 2]))
    }
    /// Register operation:
    /// - Address: `10`
    /// - Reset value: `480`
    #[doc(alias = "OTG_CURRENT")]
    pub fn otg_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, OtgCurrent, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::new(self, address as u8, || OtgCurrent::from([224, 1]))
    }
    /// Register operation:
    /// - Address: `12`
    /// - Reset value: `1000`
    #[doc(alias = "OTG_VOLTAGE")]
    pub fn otg_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, OtgVoltage, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 12;
        ::device_driver::RegisterOperation::new(self, address as u8, || OtgVoltage::from([232, 3]))
    }
    /// Register operation:
    /// - Address: `14`
    /// - Reset value: `1320`
    #[doc(alias = "VSYS_MIN")]
    pub fn vsys_min(&mut self) -> ::device_driver::RegisterOperation<'_, Self, VsysMin, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 14;
        ::device_driver::RegisterOperation::new(self, address as u8, || VsysMin::from([40, 5]))
    }
    /// Termination and precharge current settings at 0x10-0x11.
    ///
    /// Register operation:
    /// - Address: `16`
    /// - Reset value: `0x3020`
    #[doc(alias = "CHARGE_PROFILE")]
    pub fn charge_profile(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeProfile, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeProfile::from([32, 48]))
    }
    /// Gate drive settings at 0x12-0x13.
    ///
    /// Register operation:
    /// - Address: `18`
    /// - Reset value: `0x6C6C`
    #[doc(alias = "GATE_DRIVE")]
    pub fn gate_drive(&mut self) -> ::device_driver::RegisterOperation<'_, Self, GateDrive, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 18;
        ::device_driver::RegisterOperation::new(self, address as u8, || GateDrive::from([108, 108]))
    }
    /// Charge option 5 at 0x14-0x15.
    ///
    /// Register operation:
    /// - Address: `20`
    /// - Reset value: `0x0685`
    #[doc(alias = "CHARGE_OPTION_5")]
    pub fn charge_option_5(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeOption5, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 20;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeOption5::from([133, 6]))
    }
    /// Automatic charge settings and status at 0x16-0x17.
    ///
    /// Register operation:
    /// - Address: `22`
    /// - Reset value: `0x01C2`
    #[doc(alias = "AUTO_CHARGE")]
    pub fn auto_charge(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AutoCharge, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 22;
        ::device_driver::RegisterOperation::new(self, address as u8, || AutoCharge::from([194, 1]))
    }
    /// Charger status 0 at 0x18-0x19.
    ///
    /// Register operation:
    /// - Address: `24`
    /// - Reset value: `0`
    #[doc(alias = "CHARGER_STATUS_0")]
    pub fn charger_status_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargerStatus0, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 24;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargerStatus0::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `26`
    /// - Reset value: `0`
    #[doc(alias = "ADC_VBAT")]
    pub fn adc_vbat(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AdcVbat, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 26;
        ::device_driver::RegisterOperation::new(self, address as u8, || AdcVbat::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `28`
    /// - Reset value: `0`
    #[doc(alias = "ADC_PSYS")]
    pub fn adc_psys(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AdcPsys, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 28;
        ::device_driver::RegisterOperation::new(self, address as u8, || AdcPsys::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `30`
    /// - Reset value: `0`
    #[doc(alias = "ADC_CMPIN_TR")]
    pub fn adc_cmpin_tr(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AdcCmpinTr, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 30;
        ::device_driver::RegisterOperation::new(self, address as u8, || AdcCmpinTr::from([0, 0]))
    }
    /// Charger status 1 and fault clearing at 0x20-0x21.
    ///
    /// Register operation:
    /// - Address: `32`
    /// - Reset value: `0`
    #[doc(alias = "CHARGER_STATUS_1")]
    pub fn charger_status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargerStatus1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 32;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargerStatus1::from([0, 0]))
    }
    /// PROCHOT status and pulse control at 0x22-0x23.
    ///
    /// Register operation:
    /// - Address: `34`
    /// - Reset value: `0x3800`
    #[doc(alias = "PROCHOT_STATUS_REG")]
    pub fn prochot_status_reg(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ProchotStatusReg, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 34;
        ::device_driver::RegisterOperation::new(self, address as u8, || ProchotStatusReg::from([0, 56]))
    }
    /// Register operation:
    /// - Address: `36`
    /// - Reset value: `800`
    #[doc(alias = "IIN_DPM")]
    pub fn iin_dpm(&mut self) -> ::device_driver::RegisterOperation<'_, Self, IinDpm, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 36;
        ::device_driver::RegisterOperation::new(self, address as u8, || IinDpm::from([32, 3]))
    }
    /// Register operation:
    /// - Address: `38`
    /// - Reset value: `0`
    #[doc(alias = "ADC_VBUS")]
    pub fn adc_vbus(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AdcVbus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 38;
        ::device_driver::RegisterOperation::new(self, address as u8, || AdcVbus::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `40`
    /// - Reset value: `0`
    #[doc(alias = "ADC_IBAT")]
    pub fn adc_ibat(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AdcIbat, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 40;
        ::device_driver::RegisterOperation::new(self, address as u8, || AdcIbat::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `42`
    /// - Reset value: `0`
    #[doc(alias = "ADC_IIN")]
    pub fn adc_iin(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AdcIin, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 42;
        ::device_driver::RegisterOperation::new(self, address as u8, || AdcIin::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `44`
    /// - Reset value: `0`
    #[doc(alias = "ADC_VSYS")]
    pub fn adc_vsys(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AdcVsys, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 44;
        ::device_driver::RegisterOperation::new(self, address as u8, || AdcVsys::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `46`
    /// - Reset value: `64`
    #[doc(alias = "MANUFACTURE_ID")]
    pub fn manufacture_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ManufactureId, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 46;
        ::device_driver::RegisterOperation::new(self, address as u8, || ManufactureId::from([64]))
    }
    /// Register operation:
    /// - Address: `47`
    /// - Reset value: `9`
    #[doc(alias = "DEVICE_ID")]
    pub fn device_id(&mut self) -> ::device_driver::RegisterOperation<'_, Self, DeviceId, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 47;
        ::device_driver::RegisterOperation::new(self, address as u8, || DeviceId::from([9]))
    }
    /// Charge option 1 at 0x30-0x31.
    ///
    /// Register operation:
    /// - Address: `48`
    /// - Reset value: `0x3201`
    #[doc(alias = "CHARGE_OPTION_1")]
    pub fn charge_option_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeOption1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 48;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeOption1::from([1, 50]))
    }
    /// Charge option 2 at 0x32-0x33.
    ///
    /// Register operation:
    /// - Address: `50`
    /// - Reset value: `183`
    #[doc(alias = "CHARGE_OPTION_2")]
    pub fn charge_option_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeOption2, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 50;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeOption2::from([183, 0]))
    }
    /// Charge option 3 at 0x34-0x35.
    ///
    /// Register operation:
    /// - Address: `52`
    /// - Reset value: `0x0534`
    #[doc(alias = "CHARGE_OPTION_3")]
    pub fn charge_option_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeOption3, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 52;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeOption3::from([52, 5]))
    }
    /// PROCHOT option 0 at 0x36-0x37.
    ///
    /// Register operation:
    /// - Address: `54`
    /// - Reset value: `0x4A39`
    #[doc(alias = "PROCHOT_OPTION_0")]
    pub fn prochot_option_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ProchotOption0, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 54;
        ::device_driver::RegisterOperation::new(self, address as u8, || ProchotOption0::from([57, 74]))
    }
    /// PROCHOT option 1 at 0x38-0x39.
    ///
    /// Register operation:
    /// - Address: `56`
    /// - Reset value: `0x41A0`
    #[doc(alias = "PROCHOT_OPTION_1")]
    pub fn prochot_option_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ProchotOption1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 56;
        ::device_driver::RegisterOperation::new(self, address as u8, || ProchotOption1::from([160, 65]))
    }
    /// ADC channel selection and conversion control at 0x3A-0x3B.
    ///
    /// Register operation:
    /// - Address: `58`
    /// - Reset value: `0x9000`
    #[doc(alias = "ADC_OPTION")]
    pub fn adc_option(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AdcOption, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 58;
        ::device_driver::RegisterOperation::new(self, address as u8, || AdcOption::from([0, 144]))
    }
    /// Charge option 4 at 0x3C-0x3D.
    ///
    /// Register operation:
    /// - Address: `60`
    /// - Reset value: `72`
    #[doc(alias = "CHARGE_OPTION_4")]
    pub fn charge_option_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargeOption4, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 60;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargeOption4::from([72, 0]))
    }
    /// Vmin active protection settings at 0x3E-0x3F.
    ///
    /// Register operation:
    /// - Address: `62`
    /// - Reset value: `36`
    #[doc(alias = "VMIN_ACTIVE_PROTECTION")]
    pub fn vmin_active_protection(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, VminActiveProtection, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 62;
        ::device_driver::RegisterOperation::new(self, address as u8, || VminActiveProtection::from([36, 0]))
    }
    /// Autotune readings at 0x60-0x61, with phase B in the low byte and phase A in the high byte.
    ///
    /// Register operation:
    /// - Address: `96`
    /// - Reset value: `0`
    #[doc(alias = "AUTOTUNE_READ")]
    pub fn autotune_read(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AutotuneRead, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 96;
        ::device_driver::RegisterOperation::new(self, address as u8, || AutotuneRead::from([0, 0]))
    }
    /// Forced autotune values at 0x62-0x63, with phase B in the low byte and phase A in the high byte.
    ///
    /// Register operation:
    /// - Address: `98`
    /// - Reset value: `0xC8C8`
    #[doc(alias = "AUTOTUNE_FORCE")]
    pub fn autotune_force(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AutotuneForce, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 98;
        ::device_driver::RegisterOperation::new(self, address as u8, || AutotuneForce::from([200, 200]))
    }
    /// Forced GM adjustment and update control at 0x64-0x65.
    ///
    /// Register operation:
    /// - Address: `100`
    /// - Reset value: `199`
    #[doc(alias = "GM_ADJUST_FORCE")]
    pub fn gm_adjust_force(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, GmAdjustForce, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 100;
        ::device_driver::RegisterOperation::new(self, address as u8, || GmAdjustForce::from([199, 0]))
    }
    /// Virtual control at 0x80-0x81.
    ///
    /// Register operation:
    /// - Address: `128`
    /// - Reset value: `19`
    #[doc(alias = "VIRTUAL_CONTROL")]
    pub fn virtual_control(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, VirtualControl, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 128;
        ::device_driver::RegisterOperation::new(self, address as u8, || VirtualControl::from([19, 0]))
    }
}
impl<I> ::device_driver::Block for Device<I> {
    type Interface = I;
    type RegisterAddressType = u8;
    type CommandAddressType = u8;
    type BufferAddressType = u8;
    type RegisterAddressMode = ();
    fn interface(&mut self) -> &mut Self::Interface {
        &mut self.interface
    }
}
#[doc(alias = "VIRTUAL_CONTROL")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtualControl {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for VirtualControl {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl VirtualControl {
    /// `1:0` - Read the `wdtmr_adj` field.
    ///
    /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command. If device does not receive a write on the CHARGE_VOLTAGE() or the CHARGE_CURRENT() within the watchdog time period, the charger will be suspended by setting the CHARGE_CURRENT() to 0 mA.
    #[doc(alias = "WDTMR_ADJ")]
    #[must_use]
    pub fn wdtmr_adj(&self) -> WdtmrAdj {
        let start = 0;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 2` - Read the `wd_rst` field.
    ///
    /// Reset watch dog timer control.
    #[doc(alias = "WD_RST")]
    #[must_use]
    pub fn wd_rst(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `en_extilim` field.
    ///
    /// Enable ILIM_HIZ pin to set input current limit.
    #[doc(alias = "EN_EXTILIM")]
    #[must_use]
    pub fn en_extilim(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `reg_reset` field.
    ///
    /// Factory Reset Registers.
    #[doc(alias = "REG_RESET")]
    #[must_use]
    pub fn reg_reset(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `en_otg` field.
    ///
    /// OTG Mode Enable.
    #[doc(alias = "EN_OTG")]
    #[must_use]
    pub fn en_otg(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `en_auto_chg` field.
    ///
    /// Automatic charge control(recharge and terminate battery charging automatically).
    #[doc(alias = "EN_AUTO_CHG")]
    #[must_use]
    pub fn en_auto_chg(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `1:0` - Set the `wdtmr_adj` field.
    ///
    /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command. If device does not receive a write on the CHARGE_VOLTAGE() or the CHARGE_CURRENT() within the watchdog time period, the charger will be suspended by setting the CHARGE_CURRENT() to 0 mA.
    #[doc(alias = "WDTMR_ADJ")]
    pub fn set_wdtmr_adj(&mut self, value: WdtmrAdj) {
        let start = 0;
        let end = 1;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `wd_rst` field.
    ///
    /// Reset watch dog timer control.
    #[doc(alias = "WD_RST")]
    pub fn set_wd_rst(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `en_extilim` field.
    ///
    /// Enable ILIM_HIZ pin to set input current limit.
    #[doc(alias = "EN_EXTILIM")]
    pub fn set_en_extilim(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `reg_reset` field.
    ///
    /// Factory Reset Registers.
    #[doc(alias = "REG_RESET")]
    pub fn set_reg_reset(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `en_otg` field.
    ///
    /// OTG Mode Enable.
    #[doc(alias = "EN_OTG")]
    pub fn set_en_otg(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `en_auto_chg` field.
    ///
    /// Automatic charge control(recharge and terminate battery charging automatically).
    #[doc(alias = "EN_AUTO_CHG")]
    pub fn set_en_auto_chg(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for VirtualControl {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for VirtualControl {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<VirtualControl> for [u8; 2] {
    fn from(val: VirtualControl) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for VirtualControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("VirtualControl");
        d.field("wdtmr_adj", &self.wdtmr_adj());
        d.field("wd_rst", &self.wd_rst());
        d.field("en_extilim", &self.en_extilim());
        d.field("reg_reset", &self.reg_reset());
        d.field("en_otg", &self.en_otg());
        d.field("en_auto_chg", &self.en_auto_chg());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VirtualControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VirtualControl {{ ");
        defmt::write!(f, "wdtmr_adj: {}, ", &self.wdtmr_adj());
        defmt::write!(f, "wd_rst: {=bool}, ", &self.wd_rst());
        defmt::write!(f, "en_extilim: {=bool}, ", &self.en_extilim());
        defmt::write!(f, "reg_reset: {=bool}, ", &self.reg_reset());
        defmt::write!(f, "en_otg: {=bool}, ", &self.en_otg());
        defmt::write!(f, "en_auto_chg: {=bool}, ", &self.en_auto_chg());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for VirtualControl {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for VirtualControl {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for VirtualControl {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for VirtualControl {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for VirtualControl {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for VirtualControl {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for VirtualControl {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "GM_ADJUST_FORCE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct GmAdjustForce {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for GmAdjustForce {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl GmAdjustForce {
    /// `bit 0` - Read the `force_autotune_en` field.
    ///
    /// Enable FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B effective for inductor DCR current sense.
    #[doc(alias = "FORCE_AUTOTUNE_EN")]
    #[must_use]
    pub fn force_autotune_en(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `force_gm_adjust_en` field.
    ///
    /// Enable FORCE_GM_ADJUST effective for inductor DCR current sense.
    #[doc(alias = "FORCE_GM_ADJUST_EN")]
    #[must_use]
    pub fn force_gm_adjust_en(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `7:2` - Read the `force_gm_adjust` field.
    ///
    /// Force GM adjustment value for inductor DCR.
    #[doc(alias = "FORCE_GM_ADJUST")]
    #[must_use]
    pub fn force_gm_adjust(&self) -> u8 {
        let start = 2;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 9` - Read the `force_update` field.
    ///
    /// Update FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B, FORCE_GM_ADJUST value to be effective for inductor DCR current sense.
    #[doc(alias = "FORCE_UPDATE")]
    #[must_use]
    pub fn force_update(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `15:10` - Read the `gm_adjust` field.
    ///
    /// Auto adaptive adjustment value for inductor DCR.
    #[doc(alias = "GM_ADJUST")]
    #[must_use]
    pub fn gm_adjust(&self) -> u8 {
        let start = 10;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 0` - Set the `force_autotune_en` field.
    ///
    /// Enable FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B effective for inductor DCR current sense.
    #[doc(alias = "FORCE_AUTOTUNE_EN")]
    pub fn set_force_autotune_en(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `force_gm_adjust_en` field.
    ///
    /// Enable FORCE_GM_ADJUST effective for inductor DCR current sense.
    #[doc(alias = "FORCE_GM_ADJUST_EN")]
    pub fn set_force_gm_adjust_en(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `7:2` - Set the `force_gm_adjust` field.
    ///
    /// Force GM adjustment value for inductor DCR.
    #[doc(alias = "FORCE_GM_ADJUST")]
    pub fn set_force_gm_adjust(&mut self, value: u8) {
        let start = 2;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `force_update` field.
    ///
    /// Update FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B, FORCE_GM_ADJUST value to be effective for inductor DCR current sense.
    #[doc(alias = "FORCE_UPDATE")]
    pub fn set_force_update(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for GmAdjustForce {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for GmAdjustForce {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<GmAdjustForce> for [u8; 2] {
    fn from(val: GmAdjustForce) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for GmAdjustForce {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("GmAdjustForce");
        d.field("force_autotune_en", &self.force_autotune_en());
        d.field("force_gm_adjust_en", &self.force_gm_adjust_en());
        d.field("force_gm_adjust", &self.force_gm_adjust());
        d.field("force_update", &self.force_update());
        d.field("gm_adjust", &self.gm_adjust());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GmAdjustForce {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GmAdjustForce {{ ");
        defmt::write!(f, "force_autotune_en: {=bool}, ", &self.force_autotune_en());
        defmt::write!(f, "force_gm_adjust_en: {=bool}, ", &self.force_gm_adjust_en());
        defmt::write!(f, "force_gm_adjust: {=u8}, ", &self.force_gm_adjust());
        defmt::write!(f, "force_update: {=bool}, ", &self.force_update());
        defmt::write!(f, "gm_adjust: {=u8}, ", &self.gm_adjust());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for GmAdjustForce {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for GmAdjustForce {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for GmAdjustForce {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for GmAdjustForce {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for GmAdjustForce {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for GmAdjustForce {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for GmAdjustForce {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AUTOTUNE_FORCE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AutotuneForce {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AutotuneForce {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AutotuneForce {
    /// `7:0` - Read the `force_autotune_b` field.
    ///
    /// Force value for phase B inductor time constant L(uH)/DCR(mΩ).
    #[doc(alias = "FORCE_AUTOTUNE_B")]
    #[must_use]
    pub fn force_autotune_b(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `force_autotune_a` field.
    ///
    /// Force value for phase A inductor time constant L(uH)/DCR(mΩ).
    #[doc(alias = "FORCE_AUTOTUNE_A")]
    #[must_use]
    pub fn force_autotune_a(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `force_autotune_b` field.
    ///
    /// Force value for phase B inductor time constant L(uH)/DCR(mΩ).
    #[doc(alias = "FORCE_AUTOTUNE_B")]
    pub fn set_force_autotune_b(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:8` - Set the `force_autotune_a` field.
    ///
    /// Force value for phase A inductor time constant L(uH)/DCR(mΩ).
    #[doc(alias = "FORCE_AUTOTUNE_A")]
    pub fn set_force_autotune_a(&mut self, value: u8) {
        let start = 8;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AutotuneForce {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AutotuneForce {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AutotuneForce> for [u8; 2] {
    fn from(val: AutotuneForce) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AutotuneForce {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AutotuneForce");
        d.field("force_autotune_b", &self.force_autotune_b());
        d.field("force_autotune_a", &self.force_autotune_a());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AutotuneForce {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AutotuneForce {{ ");
        defmt::write!(f, "force_autotune_b: {=u8}, ", &self.force_autotune_b());
        defmt::write!(f, "force_autotune_a: {=u8}, ", &self.force_autotune_a());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AutotuneForce {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AutotuneForce {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AutotuneForce {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AutotuneForce {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AutotuneForce {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AutotuneForce {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AutotuneForce {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AUTOTUNE_READ")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AutotuneRead {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AutotuneRead {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AutotuneRead {
    /// `7:0` - Read the `autotune_b` field.
    ///
    /// Phase B inductor time constant L(uH)/DCR(mΩ) value.
    #[doc(alias = "AUTOTUNE_B")]
    #[must_use]
    pub fn autotune_b(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `autotune_a` field.
    ///
    /// Phase A inductor time constant L(uH)/DCR(mΩ) value.
    #[doc(alias = "AUTOTUNE_A")]
    #[must_use]
    pub fn autotune_a(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AutotuneRead {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AutotuneRead {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AutotuneRead> for [u8; 2] {
    fn from(val: AutotuneRead) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AutotuneRead {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AutotuneRead");
        d.field("autotune_b", &self.autotune_b());
        d.field("autotune_a", &self.autotune_a());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AutotuneRead {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AutotuneRead {{ ");
        defmt::write!(f, "autotune_b: {=u8}, ", &self.autotune_b());
        defmt::write!(f, "autotune_a: {=u8}, ", &self.autotune_a());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AutotuneRead {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AutotuneRead {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AutotuneRead {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AutotuneRead {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AutotuneRead {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AutotuneRead {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AutotuneRead {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "VMIN_ACTIVE_PROTECTION")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct VminActiveProtection {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for VminActiveProtection {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl VminActiveProtection {
    /// `bit 0` - Read the `en_frs` field.
    ///
    /// Fast Role Swap Feature Enable.
    #[doc(alias = "EN_FRS")]
    #[must_use]
    pub fn en_frs(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `en_vsysth_2_follow_vsysth_1` field.
    ///
    /// Enable internal VSYS_TH2 follow VSYS_TH1 setting neglecting register VSYS_TH2 setting.
    #[doc(alias = "EN_VSYSTH2_FOLLOW_VSYSTH1")]
    #[must_use]
    pub fn en_vsysth_2_follow_vsysth_1(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `7:2` - Read the `vsys_th_2` field.
    ///
    /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
    #[doc(alias = "VSYS_TH2")]
    #[must_use]
    pub fn vsys_th_2(&self) -> u8 {
        let start = 2;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 8` - Read the `dis_batovp_20_ma` field.
    ///
    /// Disable BATOVP 20mA discharge current through VSYS pin.
    #[doc(alias = "DIS_BATOVP_20MA")]
    #[must_use]
    pub fn dis_batovp_20_ma(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `15:9` - Read the `vbus_vap_th` field.
    ///
    /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
    #[doc(alias = "VBUS_VAP_TH")]
    #[must_use]
    pub fn vbus_vap_th(&self) -> u8 {
        let start = 9;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 0` - Set the `en_frs` field.
    ///
    /// Fast Role Swap Feature Enable.
    #[doc(alias = "EN_FRS")]
    pub fn set_en_frs(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `en_vsysth_2_follow_vsysth_1` field.
    ///
    /// Enable internal VSYS_TH2 follow VSYS_TH1 setting neglecting register VSYS_TH2 setting.
    #[doc(alias = "EN_VSYSTH2_FOLLOW_VSYSTH1")]
    pub fn set_en_vsysth_2_follow_vsysth_1(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `7:2` - Set the `vsys_th_2` field.
    ///
    /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
    #[doc(alias = "VSYS_TH2")]
    pub fn set_vsys_th_2(&mut self, value: u8) {
        let start = 2;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `dis_batovp_20_ma` field.
    ///
    /// Disable BATOVP 20mA discharge current through VSYS pin.
    #[doc(alias = "DIS_BATOVP_20MA")]
    pub fn set_dis_batovp_20_ma(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:9` - Set the `vbus_vap_th` field.
    ///
    /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
    #[doc(alias = "VBUS_VAP_TH")]
    pub fn set_vbus_vap_th(&mut self, value: u8) {
        let start = 9;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for VminActiveProtection {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for VminActiveProtection {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<VminActiveProtection> for [u8; 2] {
    fn from(val: VminActiveProtection) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for VminActiveProtection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("VminActiveProtection");
        d.field("en_frs", &self.en_frs());
        d.field("en_vsysth_2_follow_vsysth_1", &self.en_vsysth_2_follow_vsysth_1());
        d.field("vsys_th_2", &self.vsys_th_2());
        d.field("dis_batovp_20_ma", &self.dis_batovp_20_ma());
        d.field("vbus_vap_th", &self.vbus_vap_th());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VminActiveProtection {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VminActiveProtection {{ ");
        defmt::write!(f, "en_frs: {=bool}, ", &self.en_frs());
        defmt::write!(
            f,
            "en_vsysth_2_follow_vsysth_1: {=bool}, ",
            &self.en_vsysth_2_follow_vsysth_1()
        );
        defmt::write!(f, "vsys_th_2: {=u8}, ", &self.vsys_th_2());
        defmt::write!(f, "dis_batovp_20_ma: {=bool}, ", &self.dis_batovp_20_ma());
        defmt::write!(f, "vbus_vap_th: {=u8}, ", &self.vbus_vap_th());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for VminActiveProtection {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for VminActiveProtection {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for VminActiveProtection {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for VminActiveProtection {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for VminActiveProtection {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for VminActiveProtection {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for VminActiveProtection {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_OPTION_4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeOption4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeOption4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeOption4 {
    /// `bit 0` - Read the `stat_ptm` field.
    ///
    /// PTM operation status active.
    #[doc(alias = "STAT_PTM")]
    #[must_use]
    pub fn stat_ptm(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `stat_idchg_2` field.
    ///
    /// IDCHG2 status triggered.
    #[doc(alias = "STAT_IDCHG2")]
    #[must_use]
    pub fn stat_idchg_2(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `pp_idchg_2` field.
    ///
    /// Enable IDCHG_TH2 PROCHOT Profile.
    #[doc(alias = "PP_IDCHG2")]
    #[must_use]
    pub fn pp_idchg_2(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `5:3` - Read the `idchg_th_2` field.
    ///
    /// Battery discharge current limit2 based on percentage of IDCHG_TH1.
    #[doc(alias = "IDCHG_TH2")]
    #[must_use]
    pub fn idchg_th_2(&self) -> u8 {
        let start = 3;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:6` - Read the `idchg_deg_2` field.
    ///
    /// Battery discharge current limit 2 deglitch time.
    #[doc(alias = "IDCHG_DEG2")]
    #[must_use]
    pub fn idchg_deg_2(&self) -> IdchgDeglitchTime2 {
        let start = 6;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 8` - Read the `stat_vbus_vap` field.
    ///
    /// VBUS_VAP status triggered.
    #[doc(alias = "STAT_VBUS_VAP")]
    #[must_use]
    pub fn stat_vbus_vap(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `pp_vbus_vap` field.
    ///
    /// Enable VBUS_VAP PROCHOT Profile.
    #[doc(alias = "PP_VBUS_VAP")]
    #[must_use]
    pub fn pp_vbus_vap(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `vsys_uvp_no_hiccup` field.
    ///
    /// Disable VSYS_UVP Hiccup mode operation.
    #[doc(alias = "VSYS_UVP_NO_HICCUP")]
    #[must_use]
    pub fn vsys_uvp_no_hiccup(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `12:11` - Read the `en_dither` field.
    ///
    /// Frequency Dither configuration.
    #[doc(alias = "EN_DITHER")]
    #[must_use]
    pub fn en_dither(&self) -> DitherConfig {
        let start = 11;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `15:13` - Read the `vsys_uvp` field.
    ///
    /// VSYS Under Voltage Lock Out. After UVP is triggered the charger enters hiccup mode, and then the charger is latched off if the restart fails 7 times in 90s.
    #[doc(alias = "VSYS_UVP")]
    #[must_use]
    pub fn vsys_uvp(&self) -> u8 {
        let start = 13;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 0` - Set the `stat_ptm` field.
    ///
    /// PTM operation status active.
    #[doc(alias = "STAT_PTM")]
    pub fn set_stat_ptm(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `stat_idchg_2` field.
    ///
    /// IDCHG2 status triggered.
    #[doc(alias = "STAT_IDCHG2")]
    pub fn set_stat_idchg_2(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `pp_idchg_2` field.
    ///
    /// Enable IDCHG_TH2 PROCHOT Profile.
    #[doc(alias = "PP_IDCHG2")]
    pub fn set_pp_idchg_2(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `5:3` - Set the `idchg_th_2` field.
    ///
    /// Battery discharge current limit2 based on percentage of IDCHG_TH1.
    #[doc(alias = "IDCHG_TH2")]
    pub fn set_idchg_th_2(&mut self, value: u8) {
        let start = 3;
        let end = 5;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `7:6` - Set the `idchg_deg_2` field.
    ///
    /// Battery discharge current limit 2 deglitch time.
    #[doc(alias = "IDCHG_DEG2")]
    pub fn set_idchg_deg_2(&mut self, value: IdchgDeglitchTime2) {
        let start = 6;
        let end = 7;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `stat_vbus_vap` field.
    ///
    /// VBUS_VAP status triggered.
    #[doc(alias = "STAT_VBUS_VAP")]
    pub fn set_stat_vbus_vap(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `pp_vbus_vap` field.
    ///
    /// Enable VBUS_VAP PROCHOT Profile.
    #[doc(alias = "PP_VBUS_VAP")]
    pub fn set_pp_vbus_vap(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `vsys_uvp_no_hiccup` field.
    ///
    /// Disable VSYS_UVP Hiccup mode operation.
    #[doc(alias = "VSYS_UVP_NO_HICCUP")]
    pub fn set_vsys_uvp_no_hiccup(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `12:11` - Set the `en_dither` field.
    ///
    /// Frequency Dither configuration.
    #[doc(alias = "EN_DITHER")]
    pub fn set_en_dither(&mut self, value: DitherConfig) {
        let start = 11;
        let end = 12;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:13` - Set the `vsys_uvp` field.
    ///
    /// VSYS Under Voltage Lock Out. After UVP is triggered the charger enters hiccup mode, and then the charger is latched off if the restart fails 7 times in 90s.
    #[doc(alias = "VSYS_UVP")]
    pub fn set_vsys_uvp(&mut self, value: u8) {
        let start = 13;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeOption4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeOption4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeOption4> for [u8; 2] {
    fn from(val: ChargeOption4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeOption4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeOption4");
        d.field("stat_ptm", &self.stat_ptm());
        d.field("stat_idchg_2", &self.stat_idchg_2());
        d.field("pp_idchg_2", &self.pp_idchg_2());
        d.field("idchg_th_2", &self.idchg_th_2());
        d.field("idchg_deg_2", &self.idchg_deg_2());
        d.field("stat_vbus_vap", &self.stat_vbus_vap());
        d.field("pp_vbus_vap", &self.pp_vbus_vap());
        d.field("vsys_uvp_no_hiccup", &self.vsys_uvp_no_hiccup());
        d.field("en_dither", &self.en_dither());
        d.field("vsys_uvp", &self.vsys_uvp());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeOption4 {{ ");
        defmt::write!(f, "stat_ptm: {=bool}, ", &self.stat_ptm());
        defmt::write!(f, "stat_idchg_2: {=bool}, ", &self.stat_idchg_2());
        defmt::write!(f, "pp_idchg_2: {=bool}, ", &self.pp_idchg_2());
        defmt::write!(f, "idchg_th_2: {=u8}, ", &self.idchg_th_2());
        defmt::write!(f, "idchg_deg_2: {}, ", &self.idchg_deg_2());
        defmt::write!(f, "stat_vbus_vap: {=bool}, ", &self.stat_vbus_vap());
        defmt::write!(f, "pp_vbus_vap: {=bool}, ", &self.pp_vbus_vap());
        defmt::write!(f, "vsys_uvp_no_hiccup: {=bool}, ", &self.vsys_uvp_no_hiccup());
        defmt::write!(f, "en_dither: {}, ", &self.en_dither());
        defmt::write!(f, "vsys_uvp: {=u8}, ", &self.vsys_uvp());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeOption4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeOption4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeOption4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeOption4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeOption4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeOption4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeOption4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ADC_OPTION")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdcOption {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AdcOption {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AdcOption {
    /// `bit 0` - Read the `en_adc_vbat` field.
    ///
    /// Enable SRN pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_VBAT")]
    #[must_use]
    pub fn en_adc_vbat(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `en_adc_vsys` field.
    ///
    /// Enable VSYS pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_VSYS")]
    #[must_use]
    pub fn en_adc_vsys(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `en_adc_ibat` field.
    ///
    /// Enable ICHG ADC Channel.
    #[doc(alias = "EN_ADC_IBAT")]
    #[must_use]
    pub fn en_adc_ibat(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `en_adc_iin` field.
    ///
    /// Enable IIN ADC Channel.
    #[doc(alias = "EN_ADC_IIN")]
    #[must_use]
    pub fn en_adc_iin(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `en_adc_psys` field.
    ///
    /// Enable PSYS pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_PSYS")]
    #[must_use]
    pub fn en_adc_psys(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `en_adc_vbus` field.
    ///
    /// Enable VBUS pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_VBUS")]
    #[must_use]
    pub fn en_adc_vbus(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `en_adc_cmpin` field.
    ///
    /// Enable CMPIN_TR pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_CMPIN")]
    #[must_use]
    pub fn en_adc_cmpin(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `adc_avg_init` field.
    ///
    /// ADC average initial value control.
    #[doc(alias = "ADC_AVG_INIT")]
    #[must_use]
    pub fn adc_avg_init(&self) -> AdcAvgInit {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 11` - Read the `adc_avg` field.
    ///
    /// ADC average control.
    #[doc(alias = "ADC_AVG")]
    #[must_use]
    pub fn adc_avg(&self) -> AdcAvgCtrl {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `13:12` - Read the `adc_sample` field.
    ///
    /// ADC sample resolution selection, each channel conversion time is also determined based on resolution.
    #[doc(alias = "ADC_SAMPLE")]
    #[must_use]
    pub fn adc_sample(&self) -> AdcResolution {
        let start = 12;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 14` - Read the `adc_en` field.
    ///
    /// ADC conversion enable command.
    #[doc(alias = "ADC_EN")]
    #[must_use]
    pub fn adc_en(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `adc_rate` field.
    ///
    /// ADC conversion type selection. Typical conversion time is determined by resolution accuracy.
    #[doc(alias = "ADC_RATE")]
    #[must_use]
    pub fn adc_rate(&self) -> AdcRateSelect {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 0` - Set the `en_adc_vbat` field.
    ///
    /// Enable SRN pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_VBAT")]
    pub fn set_en_adc_vbat(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `en_adc_vsys` field.
    ///
    /// Enable VSYS pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_VSYS")]
    pub fn set_en_adc_vsys(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `en_adc_ibat` field.
    ///
    /// Enable ICHG ADC Channel.
    #[doc(alias = "EN_ADC_IBAT")]
    pub fn set_en_adc_ibat(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `en_adc_iin` field.
    ///
    /// Enable IIN ADC Channel.
    #[doc(alias = "EN_ADC_IIN")]
    pub fn set_en_adc_iin(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `en_adc_psys` field.
    ///
    /// Enable PSYS pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_PSYS")]
    pub fn set_en_adc_psys(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `en_adc_vbus` field.
    ///
    /// Enable VBUS pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_VBUS")]
    pub fn set_en_adc_vbus(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `en_adc_cmpin` field.
    ///
    /// Enable CMPIN_TR pin Voltage ADC Channel.
    #[doc(alias = "EN_ADC_CMPIN")]
    pub fn set_en_adc_cmpin(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `adc_avg_init` field.
    ///
    /// ADC average initial value control.
    #[doc(alias = "ADC_AVG_INIT")]
    pub fn set_adc_avg_init(&mut self, value: AdcAvgInit) {
        let start = 10;
        let end = 10;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `adc_avg` field.
    ///
    /// ADC average control.
    #[doc(alias = "ADC_AVG")]
    pub fn set_adc_avg(&mut self, value: AdcAvgCtrl) {
        let start = 11;
        let end = 11;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `13:12` - Set the `adc_sample` field.
    ///
    /// ADC sample resolution selection, each channel conversion time is also determined based on resolution.
    #[doc(alias = "ADC_SAMPLE")]
    pub fn set_adc_sample(&mut self, value: AdcResolution) {
        let start = 12;
        let end = 13;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `adc_en` field.
    ///
    /// ADC conversion enable command.
    #[doc(alias = "ADC_EN")]
    pub fn set_adc_en(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `adc_rate` field.
    ///
    /// ADC conversion type selection. Typical conversion time is determined by resolution accuracy.
    #[doc(alias = "ADC_RATE")]
    pub fn set_adc_rate(&mut self, value: AdcRateSelect) {
        let start = 15;
        let end = 15;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AdcOption {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AdcOption {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AdcOption> for [u8; 2] {
    fn from(val: AdcOption) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AdcOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AdcOption");
        d.field("en_adc_vbat", &self.en_adc_vbat());
        d.field("en_adc_vsys", &self.en_adc_vsys());
        d.field("en_adc_ibat", &self.en_adc_ibat());
        d.field("en_adc_iin", &self.en_adc_iin());
        d.field("en_adc_psys", &self.en_adc_psys());
        d.field("en_adc_vbus", &self.en_adc_vbus());
        d.field("en_adc_cmpin", &self.en_adc_cmpin());
        d.field("adc_avg_init", &self.adc_avg_init());
        d.field("adc_avg", &self.adc_avg());
        d.field("adc_sample", &self.adc_sample());
        d.field("adc_en", &self.adc_en());
        d.field("adc_rate", &self.adc_rate());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcOption {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcOption {{ ");
        defmt::write!(f, "en_adc_vbat: {=bool}, ", &self.en_adc_vbat());
        defmt::write!(f, "en_adc_vsys: {=bool}, ", &self.en_adc_vsys());
        defmt::write!(f, "en_adc_ibat: {=bool}, ", &self.en_adc_ibat());
        defmt::write!(f, "en_adc_iin: {=bool}, ", &self.en_adc_iin());
        defmt::write!(f, "en_adc_psys: {=bool}, ", &self.en_adc_psys());
        defmt::write!(f, "en_adc_vbus: {=bool}, ", &self.en_adc_vbus());
        defmt::write!(f, "en_adc_cmpin: {=bool}, ", &self.en_adc_cmpin());
        defmt::write!(f, "adc_avg_init: {}, ", &self.adc_avg_init());
        defmt::write!(f, "adc_avg: {}, ", &self.adc_avg());
        defmt::write!(f, "adc_sample: {}, ", &self.adc_sample());
        defmt::write!(f, "adc_en: {=bool}, ", &self.adc_en());
        defmt::write!(f, "adc_rate: {}, ", &self.adc_rate());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AdcOption {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AdcOption {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AdcOption {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AdcOption {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AdcOption {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AdcOption {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AdcOption {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "PROCHOT_OPTION_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ProchotOption1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ProchotOption1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ProchotOption1 {
    /// `bit 0` - Read the `pp_acok` field.
    ///
    /// Adapter removal PROCHOT profile enable.
    #[doc(alias = "PP_ACOK")]
    #[must_use]
    pub fn pp_acok(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `pp_batpres` field.
    ///
    /// Battery removal PROCHOT profile enable.
    #[doc(alias = "PP_BATPRES")]
    #[must_use]
    pub fn pp_batpres(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `pp_vsys` field.
    ///
    /// VSYS PROCHOT profile enable.
    #[doc(alias = "PP_VSYS")]
    #[must_use]
    pub fn pp_vsys(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `pp_idchg_1` field.
    ///
    /// IDCHG1 PROCHOT profile enable.
    #[doc(alias = "PP_IDCHG1")]
    #[must_use]
    pub fn pp_idchg_1(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `pp_inom` field.
    ///
    /// INOM PROCHOT profile enable.
    #[doc(alias = "PP_INOM")]
    #[must_use]
    pub fn pp_inom(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `pp_icrit` field.
    ///
    /// ICRIT PROCHOT profile enable.
    #[doc(alias = "PP_ICRIT")]
    #[must_use]
    pub fn pp_icrit(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `pp_cmp` field.
    ///
    /// COMP PROCHOT profile enable.
    #[doc(alias = "PP_CMP")]
    #[must_use]
    pub fn pp_cmp(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `pp_vindpm` field.
    ///
    /// VINDPM PROCHOT profile enable.
    #[doc(alias = "PP_VINDPM")]
    #[must_use]
    pub fn pp_vindpm(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `9:8` - Read the `idchg_deg_1` field.
    ///
    /// IDCHG deglitch time.
    #[doc(alias = "IDCHG_DEG1")]
    #[must_use]
    pub fn idchg_deg_1(&self) -> IdchgDeglitchTime {
        let start = 8;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `15:10` - Read the `idchg_th_1` field.
    ///
    /// IDCHG level 1 Threshold.
    #[doc(alias = "IDCHG_TH1")]
    #[must_use]
    pub fn idchg_th_1(&self) -> u8 {
        let start = 10;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 0` - Set the `pp_acok` field.
    ///
    /// Adapter removal PROCHOT profile enable.
    #[doc(alias = "PP_ACOK")]
    pub fn set_pp_acok(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `pp_batpres` field.
    ///
    /// Battery removal PROCHOT profile enable.
    #[doc(alias = "PP_BATPRES")]
    pub fn set_pp_batpres(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `pp_vsys` field.
    ///
    /// VSYS PROCHOT profile enable.
    #[doc(alias = "PP_VSYS")]
    pub fn set_pp_vsys(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `pp_idchg_1` field.
    ///
    /// IDCHG1 PROCHOT profile enable.
    #[doc(alias = "PP_IDCHG1")]
    pub fn set_pp_idchg_1(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `pp_inom` field.
    ///
    /// INOM PROCHOT profile enable.
    #[doc(alias = "PP_INOM")]
    pub fn set_pp_inom(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `pp_icrit` field.
    ///
    /// ICRIT PROCHOT profile enable.
    #[doc(alias = "PP_ICRIT")]
    pub fn set_pp_icrit(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `pp_cmp` field.
    ///
    /// COMP PROCHOT profile enable.
    #[doc(alias = "PP_CMP")]
    pub fn set_pp_cmp(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `pp_vindpm` field.
    ///
    /// VINDPM PROCHOT profile enable.
    #[doc(alias = "PP_VINDPM")]
    pub fn set_pp_vindpm(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `9:8` - Set the `idchg_deg_1` field.
    ///
    /// IDCHG deglitch time.
    #[doc(alias = "IDCHG_DEG1")]
    pub fn set_idchg_deg_1(&mut self, value: IdchgDeglitchTime) {
        let start = 8;
        let end = 9;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:10` - Set the `idchg_th_1` field.
    ///
    /// IDCHG level 1 Threshold.
    #[doc(alias = "IDCHG_TH1")]
    pub fn set_idchg_th_1(&mut self, value: u8) {
        let start = 10;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ProchotOption1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ProchotOption1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ProchotOption1> for [u8; 2] {
    fn from(val: ProchotOption1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ProchotOption1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ProchotOption1");
        d.field("pp_acok", &self.pp_acok());
        d.field("pp_batpres", &self.pp_batpres());
        d.field("pp_vsys", &self.pp_vsys());
        d.field("pp_idchg_1", &self.pp_idchg_1());
        d.field("pp_inom", &self.pp_inom());
        d.field("pp_icrit", &self.pp_icrit());
        d.field("pp_cmp", &self.pp_cmp());
        d.field("pp_vindpm", &self.pp_vindpm());
        d.field("idchg_deg_1", &self.idchg_deg_1());
        d.field("idchg_th_1", &self.idchg_th_1());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ProchotOption1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ProchotOption1 {{ ");
        defmt::write!(f, "pp_acok: {=bool}, ", &self.pp_acok());
        defmt::write!(f, "pp_batpres: {=bool}, ", &self.pp_batpres());
        defmt::write!(f, "pp_vsys: {=bool}, ", &self.pp_vsys());
        defmt::write!(f, "pp_idchg_1: {=bool}, ", &self.pp_idchg_1());
        defmt::write!(f, "pp_inom: {=bool}, ", &self.pp_inom());
        defmt::write!(f, "pp_icrit: {=bool}, ", &self.pp_icrit());
        defmt::write!(f, "pp_cmp: {=bool}, ", &self.pp_cmp());
        defmt::write!(f, "pp_vindpm: {=bool}, ", &self.pp_vindpm());
        defmt::write!(f, "idchg_deg_1: {}, ", &self.idchg_deg_1());
        defmt::write!(f, "idchg_th_1: {=u8}, ", &self.idchg_th_1());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ProchotOption1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ProchotOption1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ProchotOption1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ProchotOption1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ProchotOption1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ProchotOption1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ProchotOption1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "PROCHOT_OPTION_0")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ProchotOption0 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ProchotOption0 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ProchotOption0 {
    /// `bit 0` - Read the `lower_prochot_vindpm` field.
    ///
    /// Enable lower threshold of PROCHOT_VINDPM comparator.
    #[doc(alias = "LOWER_PROCHOT_VINDPM")]
    #[must_use]
    pub fn lower_prochot_vindpm(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `inom_deg` field.
    ///
    /// INOM deglitch time.
    #[doc(alias = "INOM_DEG")]
    #[must_use]
    pub fn inom_deg(&self) -> InomDeglitchTime {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `7:2` - Read the `vsys_th_1` field.
    ///
    /// VSYS threshold to trigger discharging VBUS in VAP mode.
    #[doc(alias = "VSYS_TH1")]
    #[must_use]
    pub fn vsys_th_1(&self) -> u8 {
        let start = 2;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 8` - Read the `prochot_vindpm_80_90` field.
    ///
    /// Lower threshold of the PROCHOT_VINDPM comparator. When LOWER_PROCHOT_VINDPM=1, the threshold of PROCHOT_VINDPM is determined by this setting.
    #[doc(alias = "PROCHOT_VINDPM_80_90")]
    #[must_use]
    pub fn prochot_vindpm_80_90(&self) -> Threshold {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `10:9` - Read the `icrit_deg` field.
    ///
    /// ICRIT deglitch time to trigger PROCHOT.
    #[doc(alias = "ICRIT_DEG")]
    #[must_use]
    pub fn icrit_deg(&self) -> IcritDeglitchTime {
        let start = 9;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `15:11` - Read the `ilim_2_vth` field.
    ///
    /// ILIM2 Threshold.
    #[doc(alias = "ILIM2_VTH")]
    #[must_use]
    pub fn ilim_2_vth(&self) -> u8 {
        let start = 11;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 0` - Set the `lower_prochot_vindpm` field.
    ///
    /// Enable lower threshold of PROCHOT_VINDPM comparator.
    #[doc(alias = "LOWER_PROCHOT_VINDPM")]
    pub fn set_lower_prochot_vindpm(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `inom_deg` field.
    ///
    /// INOM deglitch time.
    #[doc(alias = "INOM_DEG")]
    pub fn set_inom_deg(&mut self, value: InomDeglitchTime) {
        let start = 1;
        let end = 1;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `7:2` - Set the `vsys_th_1` field.
    ///
    /// VSYS threshold to trigger discharging VBUS in VAP mode.
    #[doc(alias = "VSYS_TH1")]
    pub fn set_vsys_th_1(&mut self, value: u8) {
        let start = 2;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `prochot_vindpm_80_90` field.
    ///
    /// Lower threshold of the PROCHOT_VINDPM comparator. When LOWER_PROCHOT_VINDPM=1, the threshold of PROCHOT_VINDPM is determined by this setting.
    #[doc(alias = "PROCHOT_VINDPM_80_90")]
    pub fn set_prochot_vindpm_80_90(&mut self, value: Threshold) {
        let start = 8;
        let end = 8;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `10:9` - Set the `icrit_deg` field.
    ///
    /// ICRIT deglitch time to trigger PROCHOT.
    #[doc(alias = "ICRIT_DEG")]
    pub fn set_icrit_deg(&mut self, value: IcritDeglitchTime) {
        let start = 9;
        let end = 10;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:11` - Set the `ilim_2_vth` field.
    ///
    /// ILIM2 Threshold.
    #[doc(alias = "ILIM2_VTH")]
    pub fn set_ilim_2_vth(&mut self, value: u8) {
        let start = 11;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ProchotOption0 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ProchotOption0 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ProchotOption0> for [u8; 2] {
    fn from(val: ProchotOption0) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ProchotOption0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ProchotOption0");
        d.field("lower_prochot_vindpm", &self.lower_prochot_vindpm());
        d.field("inom_deg", &self.inom_deg());
        d.field("vsys_th_1", &self.vsys_th_1());
        d.field("prochot_vindpm_80_90", &self.prochot_vindpm_80_90());
        d.field("icrit_deg", &self.icrit_deg());
        d.field("ilim_2_vth", &self.ilim_2_vth());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ProchotOption0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ProchotOption0 {{ ");
        defmt::write!(f, "lower_prochot_vindpm: {=bool}, ", &self.lower_prochot_vindpm());
        defmt::write!(f, "inom_deg: {}, ", &self.inom_deg());
        defmt::write!(f, "vsys_th_1: {=u8}, ", &self.vsys_th_1());
        defmt::write!(f, "prochot_vindpm_80_90: {}, ", &self.prochot_vindpm_80_90());
        defmt::write!(f, "icrit_deg: {}, ", &self.icrit_deg());
        defmt::write!(f, "ilim_2_vth: {=u8}, ", &self.ilim_2_vth());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ProchotOption0 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ProchotOption0 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ProchotOption0 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ProchotOption0 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ProchotOption0 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ProchotOption0 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ProchotOption0 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_OPTION_3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeOption3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeOption3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeOption3 {
    /// `bit 0` - Read the `psys_otg_idchg` field.
    ///
    /// PSYS definition during OTG mode.
    #[doc(alias = "PSYS_OTG_IDCHG")]
    #[must_use]
    pub fn psys_otg_idchg(&self) -> PsysOtg {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 1` - Read the `batfetoff_hiz` field.
    ///
    /// BATFET off during HIZ mode?
    #[doc(alias = "BATFETOFF_HIZ")]
    #[must_use]
    pub fn batfetoff_hiz(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `cmp_en` field.
    ///
    /// Enable Independent Comparator with effective low.
    #[doc(alias = "CMP_EN")]
    #[must_use]
    pub fn cmp_en(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `4:3` - Read the `il_avg` field.
    ///
    /// Inductor average current clamp.
    #[doc(alias = "IL_AVG")]
    #[must_use]
    pub fn il_avg(&self) -> IlAvgClamp {
        let start = 3;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 5` - Read the `otg_vap_mode` field.
    ///
    /// The selection of the external EN_OTG pin control.
    #[doc(alias = "OTG_VAP_MODE")]
    #[must_use]
    pub fn otg_vap_mode(&self) -> EnOtgPinSelect {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 7` - Read the `pkpwr_tovld_deg` field.
    ///
    /// Force turn off BATFET under battery only low power mode.
    #[doc(alias = "PKPWR_TOVLD_DEG")]
    #[must_use]
    pub fn pkpwr_tovld_deg(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `9:8` - Read the `en_vsys_min_soft_sr` field.
    ///
    /// VSYS_MIN soft slew rate control for VSYS_MIN step up transition. Note for step down doesn't need the soft transition.
    #[doc(alias = "EN_VSYS_MIN_SOFT_SR")]
    #[must_use]
    pub fn en_vsys_min_soft_sr(&self) -> VsysMinSoftSlewRate {
        let start = 8;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 10` - Read the `en_port_ctrl` field.
    ///
    /// Enable BATFET control for dual port application.
    #[doc(alias = "EN_PORT_CTRL")]
    #[must_use]
    pub fn en_port_ctrl(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `en_ico_mode` field.
    ///
    /// Enable ICO Algorithm.
    #[doc(alias = "EN_ICO_MODE")]
    #[must_use]
    pub fn en_ico_mode(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `en_otg` field.
    ///
    /// OTG Mode Enable. Enable device in OTG mode when EN_OTG pin is HIGH.
    #[doc(alias = "EN_OTG")]
    #[must_use]
    pub fn en_otg(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `detect_vindpm` field.
    ///
    /// Set VINDPM threshold based on VBUS measurement result minus 1.28V, Converter is disabled to measure VBUS.
    #[doc(alias = "DETECT_VINDPM")]
    #[must_use]
    pub fn detect_vindpm(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `reg_reset` field.
    ///
    /// Factory Reset Registers.
    #[doc(alias = "REG_RESET")]
    #[must_use]
    pub fn reg_reset(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `en_hiz` field.
    ///
    /// Device Hi-Z Mode Enable. When the charger is in Hi-Z mode, the device draws minimal quiescent current. With VBUS above UVLO. REGN LDO stays on, and system powers from battery.
    #[doc(alias = "EN_HIZ")]
    #[must_use]
    pub fn en_hiz(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `psys_otg_idchg` field.
    ///
    /// PSYS definition during OTG mode.
    #[doc(alias = "PSYS_OTG_IDCHG")]
    pub fn set_psys_otg_idchg(&mut self, value: PsysOtg) {
        let start = 0;
        let end = 0;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `batfetoff_hiz` field.
    ///
    /// BATFET off during HIZ mode?
    #[doc(alias = "BATFETOFF_HIZ")]
    pub fn set_batfetoff_hiz(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `cmp_en` field.
    ///
    /// Enable Independent Comparator with effective low.
    #[doc(alias = "CMP_EN")]
    pub fn set_cmp_en(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `4:3` - Set the `il_avg` field.
    ///
    /// Inductor average current clamp.
    #[doc(alias = "IL_AVG")]
    pub fn set_il_avg(&mut self, value: IlAvgClamp) {
        let start = 3;
        let end = 4;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `otg_vap_mode` field.
    ///
    /// The selection of the external EN_OTG pin control.
    #[doc(alias = "OTG_VAP_MODE")]
    pub fn set_otg_vap_mode(&mut self, value: EnOtgPinSelect) {
        let start = 5;
        let end = 5;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `pkpwr_tovld_deg` field.
    ///
    /// Force turn off BATFET under battery only low power mode.
    #[doc(alias = "PKPWR_TOVLD_DEG")]
    pub fn set_pkpwr_tovld_deg(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `9:8` - Set the `en_vsys_min_soft_sr` field.
    ///
    /// VSYS_MIN soft slew rate control for VSYS_MIN step up transition. Note for step down doesn't need the soft transition.
    #[doc(alias = "EN_VSYS_MIN_SOFT_SR")]
    pub fn set_en_vsys_min_soft_sr(&mut self, value: VsysMinSoftSlewRate) {
        let start = 8;
        let end = 9;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `en_port_ctrl` field.
    ///
    /// Enable BATFET control for dual port application.
    #[doc(alias = "EN_PORT_CTRL")]
    pub fn set_en_port_ctrl(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `en_ico_mode` field.
    ///
    /// Enable ICO Algorithm.
    #[doc(alias = "EN_ICO_MODE")]
    pub fn set_en_ico_mode(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `en_otg` field.
    ///
    /// OTG Mode Enable. Enable device in OTG mode when EN_OTG pin is HIGH.
    #[doc(alias = "EN_OTG")]
    pub fn set_en_otg(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `detect_vindpm` field.
    ///
    /// Set VINDPM threshold based on VBUS measurement result minus 1.28V, Converter is disabled to measure VBUS.
    #[doc(alias = "DETECT_VINDPM")]
    pub fn set_detect_vindpm(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `reg_reset` field.
    ///
    /// Factory Reset Registers.
    #[doc(alias = "REG_RESET")]
    pub fn set_reg_reset(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `en_hiz` field.
    ///
    /// Device Hi-Z Mode Enable. When the charger is in Hi-Z mode, the device draws minimal quiescent current. With VBUS above UVLO. REGN LDO stays on, and system powers from battery.
    #[doc(alias = "EN_HIZ")]
    pub fn set_en_hiz(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeOption3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeOption3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeOption3> for [u8; 2] {
    fn from(val: ChargeOption3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeOption3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeOption3");
        d.field("psys_otg_idchg", &self.psys_otg_idchg());
        d.field("batfetoff_hiz", &self.batfetoff_hiz());
        d.field("cmp_en", &self.cmp_en());
        d.field("il_avg", &self.il_avg());
        d.field("otg_vap_mode", &self.otg_vap_mode());
        d.field("pkpwr_tovld_deg", &self.pkpwr_tovld_deg());
        d.field("en_vsys_min_soft_sr", &self.en_vsys_min_soft_sr());
        d.field("en_port_ctrl", &self.en_port_ctrl());
        d.field("en_ico_mode", &self.en_ico_mode());
        d.field("en_otg", &self.en_otg());
        d.field("detect_vindpm", &self.detect_vindpm());
        d.field("reg_reset", &self.reg_reset());
        d.field("en_hiz", &self.en_hiz());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeOption3 {{ ");
        defmt::write!(f, "psys_otg_idchg: {}, ", &self.psys_otg_idchg());
        defmt::write!(f, "batfetoff_hiz: {=bool}, ", &self.batfetoff_hiz());
        defmt::write!(f, "cmp_en: {=bool}, ", &self.cmp_en());
        defmt::write!(f, "il_avg: {}, ", &self.il_avg());
        defmt::write!(f, "otg_vap_mode: {}, ", &self.otg_vap_mode());
        defmt::write!(f, "pkpwr_tovld_deg: {=bool}, ", &self.pkpwr_tovld_deg());
        defmt::write!(f, "en_vsys_min_soft_sr: {}, ", &self.en_vsys_min_soft_sr());
        defmt::write!(f, "en_port_ctrl: {=bool}, ", &self.en_port_ctrl());
        defmt::write!(f, "en_ico_mode: {=bool}, ", &self.en_ico_mode());
        defmt::write!(f, "en_otg: {=bool}, ", &self.en_otg());
        defmt::write!(f, "detect_vindpm: {=bool}, ", &self.detect_vindpm());
        defmt::write!(f, "reg_reset: {=bool}, ", &self.reg_reset());
        defmt::write!(f, "en_hiz: {=bool}, ", &self.en_hiz());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeOption3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeOption3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeOption3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeOption3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeOption3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeOption3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeOption3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_OPTION_2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeOption2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeOption2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeOption2 {
    /// `bit 0` - Read the `batdoc_vth` field.
    ///
    /// Set battery discharge overcurrent threshold as percentage of PROCHOT battery discharge current limit.
    #[doc(alias = "BATDOC_VTH")]
    #[must_use]
    pub fn batdoc_vth(&self) -> BatdocVth {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 1` - Read the `en_batdoc` field.
    ///
    /// Battery discharge overcurrent (BATDOC) protection enable.
    #[doc(alias = "EN_BATDOC")]
    #[must_use]
    pub fn en_batdoc(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `acoc_vth` field.
    ///
    /// ACOC Limit. Set ACOC threshold as percentage of ILIM2_VTH with current sensed from RAC.
    #[doc(alias = "ACOC_VTH")]
    #[must_use]
    pub fn acoc_vth(&self) -> AcocLimit {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 3` - Read the `en_acoc` field.
    ///
    /// Input overcurrent (ACOC) protection enable.
    #[doc(alias = "EN_ACOC")]
    #[must_use]
    pub fn en_acoc(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `ocp_sw_1_x_high_range` field.
    ///
    /// Over current protection threshold by sensing RAC resistor across voltage.
    #[doc(alias = "OCP_SW1X_HIGH_RANGE")]
    #[must_use]
    pub fn ocp_sw_1_x_high_range(&self) -> OverCurrentThresholdRac {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 5` - Read the `ocp_sw_2_high_range` field.
    ///
    /// Over current protection threshold by sensing Q4 Vds.
    #[doc(alias = "OCP_SW2_HIGH_RANGE")]
    #[must_use]
    pub fn ocp_sw_2_high_range(&self) -> OverCurrentThresholdQ4Vds {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 6` - Read the `en_ichg_idchg` field.
    ///
    /// IBAT pin monitor selection for discharge current and charge current.
    #[doc(alias = "EN_ICHG_IDCHG")]
    #[must_use]
    pub fn en_ichg_idchg(&self) -> IBatPinSelect {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 7` - Read the `en_extilim` field.
    ///
    /// Enable ILIM_HIZ pin to set input current limit.
    #[doc(alias = "EN_EXTILIM")]
    #[must_use]
    pub fn en_extilim(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `9:8` - Read the `pkpwr_tmax` field.
    ///
    /// Peak power mode overload and relax cycle time.
    #[doc(alias = "PKPWR_TMAX")]
    #[must_use]
    pub fn pkpwr_tmax(&self) -> PkpwrTmax {
        let start = 8;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 10` - Read the `stat_pkpwr_relax` field.
    ///
    /// Device is in relaxation cycle?
    #[doc(alias = "STAT_PKPWR_RELAX")]
    #[must_use]
    pub fn stat_pkpwr_relax(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `stat_pkpwr_ovld` field.
    ///
    /// Device is in overloading cycle?
    #[doc(alias = "STAT_PKPWR_OVLD")]
    #[must_use]
    pub fn stat_pkpwr_ovld(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `en_pkpwr_vsys` field.
    ///
    /// Enable Peak Power Mode triggered by system voltage under-shoot.
    #[doc(alias = "EN_PKPWR_VSYS")]
    #[must_use]
    pub fn en_pkpwr_vsys(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `en_pkpwr_iin_dp` field.
    ///
    /// Enable Peak Power Mode triggered by input current overshoot.
    #[doc(alias = "EN_PKPWR_IIN_DP")]
    #[must_use]
    pub fn en_pkpwr_iin_dp(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `15:14` - Read the `pkpwr_tovld_deg` field.
    ///
    /// Input Overload time in Peak Power Mode.
    #[doc(alias = "PKPWR_TOVLD_DEG")]
    #[must_use]
    pub fn pkpwr_tovld_deg(&self) -> PkpwrTovldDeg {
        let start = 14;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 0` - Set the `batdoc_vth` field.
    ///
    /// Set battery discharge overcurrent threshold as percentage of PROCHOT battery discharge current limit.
    #[doc(alias = "BATDOC_VTH")]
    pub fn set_batdoc_vth(&mut self, value: BatdocVth) {
        let start = 0;
        let end = 0;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `en_batdoc` field.
    ///
    /// Battery discharge overcurrent (BATDOC) protection enable.
    #[doc(alias = "EN_BATDOC")]
    pub fn set_en_batdoc(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `acoc_vth` field.
    ///
    /// ACOC Limit. Set ACOC threshold as percentage of ILIM2_VTH with current sensed from RAC.
    #[doc(alias = "ACOC_VTH")]
    pub fn set_acoc_vth(&mut self, value: AcocLimit) {
        let start = 2;
        let end = 2;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `en_acoc` field.
    ///
    /// Input overcurrent (ACOC) protection enable.
    #[doc(alias = "EN_ACOC")]
    pub fn set_en_acoc(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `ocp_sw_1_x_high_range` field.
    ///
    /// Over current protection threshold by sensing RAC resistor across voltage.
    #[doc(alias = "OCP_SW1X_HIGH_RANGE")]
    pub fn set_ocp_sw_1_x_high_range(&mut self, value: OverCurrentThresholdRac) {
        let start = 4;
        let end = 4;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `ocp_sw_2_high_range` field.
    ///
    /// Over current protection threshold by sensing Q4 Vds.
    #[doc(alias = "OCP_SW2_HIGH_RANGE")]
    pub fn set_ocp_sw_2_high_range(&mut self, value: OverCurrentThresholdQ4Vds) {
        let start = 5;
        let end = 5;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `en_ichg_idchg` field.
    ///
    /// IBAT pin monitor selection for discharge current and charge current.
    #[doc(alias = "EN_ICHG_IDCHG")]
    pub fn set_en_ichg_idchg(&mut self, value: IBatPinSelect) {
        let start = 6;
        let end = 6;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `en_extilim` field.
    ///
    /// Enable ILIM_HIZ pin to set input current limit.
    #[doc(alias = "EN_EXTILIM")]
    pub fn set_en_extilim(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `9:8` - Set the `pkpwr_tmax` field.
    ///
    /// Peak power mode overload and relax cycle time.
    #[doc(alias = "PKPWR_TMAX")]
    pub fn set_pkpwr_tmax(&mut self, value: PkpwrTmax) {
        let start = 8;
        let end = 9;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `stat_pkpwr_relax` field.
    ///
    /// Device is in relaxation cycle?
    #[doc(alias = "STAT_PKPWR_RELAX")]
    pub fn set_stat_pkpwr_relax(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `stat_pkpwr_ovld` field.
    ///
    /// Device is in overloading cycle?
    #[doc(alias = "STAT_PKPWR_OVLD")]
    pub fn set_stat_pkpwr_ovld(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `en_pkpwr_vsys` field.
    ///
    /// Enable Peak Power Mode triggered by system voltage under-shoot.
    #[doc(alias = "EN_PKPWR_VSYS")]
    pub fn set_en_pkpwr_vsys(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `en_pkpwr_iin_dp` field.
    ///
    /// Enable Peak Power Mode triggered by input current overshoot.
    #[doc(alias = "EN_PKPWR_IIN_DP")]
    pub fn set_en_pkpwr_iin_dp(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:14` - Set the `pkpwr_tovld_deg` field.
    ///
    /// Input Overload time in Peak Power Mode.
    #[doc(alias = "PKPWR_TOVLD_DEG")]
    pub fn set_pkpwr_tovld_deg(&mut self, value: PkpwrTovldDeg) {
        let start = 14;
        let end = 15;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeOption2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeOption2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeOption2> for [u8; 2] {
    fn from(val: ChargeOption2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeOption2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeOption2");
        d.field("batdoc_vth", &self.batdoc_vth());
        d.field("en_batdoc", &self.en_batdoc());
        d.field("acoc_vth", &self.acoc_vth());
        d.field("en_acoc", &self.en_acoc());
        d.field("ocp_sw_1_x_high_range", &self.ocp_sw_1_x_high_range());
        d.field("ocp_sw_2_high_range", &self.ocp_sw_2_high_range());
        d.field("en_ichg_idchg", &self.en_ichg_idchg());
        d.field("en_extilim", &self.en_extilim());
        d.field("pkpwr_tmax", &self.pkpwr_tmax());
        d.field("stat_pkpwr_relax", &self.stat_pkpwr_relax());
        d.field("stat_pkpwr_ovld", &self.stat_pkpwr_ovld());
        d.field("en_pkpwr_vsys", &self.en_pkpwr_vsys());
        d.field("en_pkpwr_iin_dp", &self.en_pkpwr_iin_dp());
        d.field("pkpwr_tovld_deg", &self.pkpwr_tovld_deg());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeOption2 {{ ");
        defmt::write!(f, "batdoc_vth: {}, ", &self.batdoc_vth());
        defmt::write!(f, "en_batdoc: {=bool}, ", &self.en_batdoc());
        defmt::write!(f, "acoc_vth: {}, ", &self.acoc_vth());
        defmt::write!(f, "en_acoc: {=bool}, ", &self.en_acoc());
        defmt::write!(f, "ocp_sw_1_x_high_range: {}, ", &self.ocp_sw_1_x_high_range());
        defmt::write!(f, "ocp_sw_2_high_range: {}, ", &self.ocp_sw_2_high_range());
        defmt::write!(f, "en_ichg_idchg: {}, ", &self.en_ichg_idchg());
        defmt::write!(f, "en_extilim: {=bool}, ", &self.en_extilim());
        defmt::write!(f, "pkpwr_tmax: {}, ", &self.pkpwr_tmax());
        defmt::write!(f, "stat_pkpwr_relax: {=bool}, ", &self.stat_pkpwr_relax());
        defmt::write!(f, "stat_pkpwr_ovld: {=bool}, ", &self.stat_pkpwr_ovld());
        defmt::write!(f, "en_pkpwr_vsys: {=bool}, ", &self.en_pkpwr_vsys());
        defmt::write!(f, "en_pkpwr_iin_dp: {=bool}, ", &self.en_pkpwr_iin_dp());
        defmt::write!(f, "pkpwr_tovld_deg: {}, ", &self.pkpwr_tovld_deg());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeOption2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeOption2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeOption2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeOption2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeOption2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeOption2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeOption2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_OPTION_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeOption1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeOption1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeOption1 {
    /// `bit 0` - Read the `en_sc_vbusacp` field.
    ///
    /// SC_VBUSACP protection enable.
    #[doc(alias = "EN_SC_VBUSACP")]
    #[must_use]
    pub fn en_sc_vbusacp(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `en_ship_dchg` field.
    ///
    /// Discharge SRN for Shipping Mode. Used to discharge SRN pin capacitor voltage which is necessary for battery gauge device shipping mode.
    #[doc(alias = "EN_SHIP_DCHG")]
    #[must_use]
    pub fn en_ship_dchg(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `en_ptm` field.
    ///
    /// PTM enable register bit, it will automatically reset to zero.
    #[doc(alias = "EN_PTM")]
    #[must_use]
    pub fn en_ptm(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `frc_conv_off` field.
    ///
    /// Force Power Path Off.
    #[doc(alias = "FRC_CONV_OFF")]
    #[must_use]
    pub fn frc_conv_off(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `5:4` - Read the `cmp_deg` field.
    ///
    /// Independent comparator deglitch time, only applied to the falling edge of CMPOUT (HIGH to LOW).
    #[doc(alias = "CMP_DEG")]
    #[must_use]
    pub fn cmp_deg(&self) -> ComparatorDeglitchTime {
        let start = 4;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 6` - Read the `cmp_pol` field.
    ///
    /// Independent Comparator output Polarity
    #[doc(alias = "CMP_POL")]
    #[must_use]
    pub fn cmp_pol(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `sysovp_max` field.
    ///
    /// Force SYSOVP protection threshold to 27V neglecting CELL_BATPRES pin configuration.
    #[doc(alias = "SYSOVP_MAX")]
    #[must_use]
    pub fn sysovp_max(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `en_otg_big_cap` field.
    ///
    /// Enable OTG compensation for VBUS effective capacitance larger than 60uF.
    #[doc(alias = "EN_OTG_BIG_CAP")]
    #[must_use]
    pub fn en_otg_big_cap(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `psys_ratio` field.
    ///
    /// PSYS Gain. Ratio of PSYS output current vs total input and battery power.
    #[doc(alias = "PSYS_RATIO")]
    #[must_use]
    pub fn psys_ratio(&self) -> PsysGain {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 10` - Read the `rsns_rsr` field.
    ///
    /// Charge sense resistor RSR. Not recommend to change this value during ICHG/IPRECHG/BATFET_CLAMP1/ BATFET_CLAMP2/BAT_SHORT regulation.
    #[doc(alias = "RSNS_RSR")]
    #[must_use]
    pub fn rsns_rsr(&self) -> ChargeSenseResistorRsr {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 11` - Read the `rsns_rac` field.
    ///
    /// Input sense resistor RAC. Not recommend to change this value during IINDPM/IOTG regulation.
    #[doc(alias = "RSNS_RAC")]
    #[must_use]
    pub fn rsns_rac(&self) -> InputSenseResistorRac {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `13:12` - Read the `psys_config` field.
    ///
    /// PSYS Enable and Definition Register. Enable PSYS sensing circuit and output buffer (whole PSYS circuit). In low power mode (EN_LWPWR=1b), PSYS sensing and buffer are always disabled regardless of this bit value.
    #[doc(alias = "PSYS_CONFIG")]
    #[must_use]
    pub fn psys_config(&self) -> PsysEnable {
        let start = 12;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 14` - Read the `en_lwpwr_cmp` field.
    ///
    /// Independent Comparator Enable.
    #[doc(alias = "EN_LWPWR_CMP")]
    #[must_use]
    pub fn en_lwpwr_cmp(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `en_ibat` field.
    ///
    /// IBAT Enable. In low power mode (EN_LWPWR=1b), IBAT buffer is always disabled regardless of this bit value.
    #[doc(alias = "EN_IBAT")]
    #[must_use]
    pub fn en_ibat(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `en_sc_vbusacp` field.
    ///
    /// SC_VBUSACP protection enable.
    #[doc(alias = "EN_SC_VBUSACP")]
    pub fn set_en_sc_vbusacp(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `en_ship_dchg` field.
    ///
    /// Discharge SRN for Shipping Mode. Used to discharge SRN pin capacitor voltage which is necessary for battery gauge device shipping mode.
    #[doc(alias = "EN_SHIP_DCHG")]
    pub fn set_en_ship_dchg(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `en_ptm` field.
    ///
    /// PTM enable register bit, it will automatically reset to zero.
    #[doc(alias = "EN_PTM")]
    pub fn set_en_ptm(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `frc_conv_off` field.
    ///
    /// Force Power Path Off.
    #[doc(alias = "FRC_CONV_OFF")]
    pub fn set_frc_conv_off(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `5:4` - Set the `cmp_deg` field.
    ///
    /// Independent comparator deglitch time, only applied to the falling edge of CMPOUT (HIGH to LOW).
    #[doc(alias = "CMP_DEG")]
    pub fn set_cmp_deg(&mut self, value: ComparatorDeglitchTime) {
        let start = 4;
        let end = 5;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `cmp_pol` field.
    ///
    /// Independent Comparator output Polarity
    #[doc(alias = "CMP_POL")]
    pub fn set_cmp_pol(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `sysovp_max` field.
    ///
    /// Force SYSOVP protection threshold to 27V neglecting CELL_BATPRES pin configuration.
    #[doc(alias = "SYSOVP_MAX")]
    pub fn set_sysovp_max(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `en_otg_big_cap` field.
    ///
    /// Enable OTG compensation for VBUS effective capacitance larger than 60uF.
    #[doc(alias = "EN_OTG_BIG_CAP")]
    pub fn set_en_otg_big_cap(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `psys_ratio` field.
    ///
    /// PSYS Gain. Ratio of PSYS output current vs total input and battery power.
    #[doc(alias = "PSYS_RATIO")]
    pub fn set_psys_ratio(&mut self, value: PsysGain) {
        let start = 9;
        let end = 9;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `rsns_rsr` field.
    ///
    /// Charge sense resistor RSR. Not recommend to change this value during ICHG/IPRECHG/BATFET_CLAMP1/ BATFET_CLAMP2/BAT_SHORT regulation.
    #[doc(alias = "RSNS_RSR")]
    pub fn set_rsns_rsr(&mut self, value: ChargeSenseResistorRsr) {
        let start = 10;
        let end = 10;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `rsns_rac` field.
    ///
    /// Input sense resistor RAC. Not recommend to change this value during IINDPM/IOTG regulation.
    #[doc(alias = "RSNS_RAC")]
    pub fn set_rsns_rac(&mut self, value: InputSenseResistorRac) {
        let start = 11;
        let end = 11;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `13:12` - Set the `psys_config` field.
    ///
    /// PSYS Enable and Definition Register. Enable PSYS sensing circuit and output buffer (whole PSYS circuit). In low power mode (EN_LWPWR=1b), PSYS sensing and buffer are always disabled regardless of this bit value.
    #[doc(alias = "PSYS_CONFIG")]
    pub fn set_psys_config(&mut self, value: PsysEnable) {
        let start = 12;
        let end = 13;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `en_lwpwr_cmp` field.
    ///
    /// Independent Comparator Enable.
    #[doc(alias = "EN_LWPWR_CMP")]
    pub fn set_en_lwpwr_cmp(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `en_ibat` field.
    ///
    /// IBAT Enable. In low power mode (EN_LWPWR=1b), IBAT buffer is always disabled regardless of this bit value.
    #[doc(alias = "EN_IBAT")]
    pub fn set_en_ibat(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeOption1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeOption1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeOption1> for [u8; 2] {
    fn from(val: ChargeOption1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeOption1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeOption1");
        d.field("en_sc_vbusacp", &self.en_sc_vbusacp());
        d.field("en_ship_dchg", &self.en_ship_dchg());
        d.field("en_ptm", &self.en_ptm());
        d.field("frc_conv_off", &self.frc_conv_off());
        d.field("cmp_deg", &self.cmp_deg());
        d.field("cmp_pol", &self.cmp_pol());
        d.field("sysovp_max", &self.sysovp_max());
        d.field("en_otg_big_cap", &self.en_otg_big_cap());
        d.field("psys_ratio", &self.psys_ratio());
        d.field("rsns_rsr", &self.rsns_rsr());
        d.field("rsns_rac", &self.rsns_rac());
        d.field("psys_config", &self.psys_config());
        d.field("en_lwpwr_cmp", &self.en_lwpwr_cmp());
        d.field("en_ibat", &self.en_ibat());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeOption1 {{ ");
        defmt::write!(f, "en_sc_vbusacp: {=bool}, ", &self.en_sc_vbusacp());
        defmt::write!(f, "en_ship_dchg: {=bool}, ", &self.en_ship_dchg());
        defmt::write!(f, "en_ptm: {=bool}, ", &self.en_ptm());
        defmt::write!(f, "frc_conv_off: {=bool}, ", &self.frc_conv_off());
        defmt::write!(f, "cmp_deg: {}, ", &self.cmp_deg());
        defmt::write!(f, "cmp_pol: {=bool}, ", &self.cmp_pol());
        defmt::write!(f, "sysovp_max: {=bool}, ", &self.sysovp_max());
        defmt::write!(f, "en_otg_big_cap: {=bool}, ", &self.en_otg_big_cap());
        defmt::write!(f, "psys_ratio: {}, ", &self.psys_ratio());
        defmt::write!(f, "rsns_rsr: {}, ", &self.rsns_rsr());
        defmt::write!(f, "rsns_rac: {}, ", &self.rsns_rac());
        defmt::write!(f, "psys_config: {}, ", &self.psys_config());
        defmt::write!(f, "en_lwpwr_cmp: {=bool}, ", &self.en_lwpwr_cmp());
        defmt::write!(f, "en_ibat: {=bool}, ", &self.en_ibat());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeOption1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeOption1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeOption1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeOption1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeOption1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeOption1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeOption1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "DEVICE_ID")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DeviceId {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for DeviceId {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl DeviceId {
    /// `7:0` - Read the `device_id` field.
    ///
    /// Device ID.
    #[doc(alias = "DEVICE_ID")]
    #[must_use]
    pub fn device_id(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `device_id` field.
    ///
    /// Device ID.
    #[doc(alias = "DEVICE_ID")]
    pub fn set_device_id(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for DeviceId {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for DeviceId {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<DeviceId> for [u8; 1] {
    fn from(val: DeviceId) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for DeviceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("DeviceId");
        d.field("device_id", &self.device_id());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DeviceId {{ ");
        defmt::write!(f, "device_id: {=u8}, ", &self.device_id());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for DeviceId {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for DeviceId {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for DeviceId {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for DeviceId {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for DeviceId {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for DeviceId {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for DeviceId {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MANUFACTURE_ID")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ManufactureId {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for ManufactureId {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl ManufactureId {
    /// `7:0` - Read the `manufacture_id` field.
    ///
    /// Manufacture ID.
    #[doc(alias = "MANUFACTURE_ID")]
    #[must_use]
    pub fn manufacture_id(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `manufacture_id` field.
    ///
    /// Manufacture ID.
    #[doc(alias = "MANUFACTURE_ID")]
    pub fn set_manufacture_id(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ManufactureId {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for ManufactureId {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<ManufactureId> for [u8; 1] {
    fn from(val: ManufactureId) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ManufactureId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ManufactureId");
        d.field("manufacture_id", &self.manufacture_id());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ManufactureId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ManufactureId {{ ");
        defmt::write!(f, "manufacture_id: {=u8}, ", &self.manufacture_id());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ManufactureId {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ManufactureId {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ManufactureId {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ManufactureId {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ManufactureId {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ManufactureId {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ManufactureId {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ADC_VSYS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdcVsys {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AdcVsys {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AdcVsys {
    /// `15:0` - Read the `adc_vsys` field.
    ///
    /// VSYS ADC reading.
    #[doc(alias = "ADC_VSYS")]
    #[must_use]
    pub fn adc_vsys(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AdcVsys {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AdcVsys {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AdcVsys> for [u8; 2] {
    fn from(val: AdcVsys) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AdcVsys {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AdcVsys");
        d.field("adc_vsys", &self.adc_vsys());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcVsys {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcVsys {{ ");
        defmt::write!(f, "adc_vsys: {=u16}, ", &self.adc_vsys());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AdcVsys {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AdcVsys {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AdcVsys {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AdcVsys {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AdcVsys {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AdcVsys {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AdcVsys {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ADC_IIN")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdcIin {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AdcIin {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AdcIin {
    /// `15:0` - Read the `adc_iin` field.
    ///
    /// IIN ADC reading with 10mΩ sense resistor. Current flowing from the adapter to the converter (like in forward mode) is represented as positive and current flowing to the adapter (like in OTG mode) is negative.
    #[doc(alias = "ADC_IIN")]
    #[must_use]
    pub fn adc_iin(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AdcIin {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AdcIin {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AdcIin> for [u8; 2] {
    fn from(val: AdcIin) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AdcIin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AdcIin");
        d.field("adc_iin", &self.adc_iin());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcIin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcIin {{ ");
        defmt::write!(f, "adc_iin: {=u16}, ", &self.adc_iin());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AdcIin {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AdcIin {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AdcIin {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AdcIin {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AdcIin {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AdcIin {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AdcIin {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ADC_IBAT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdcIbat {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AdcIbat {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AdcIbat {
    /// `15:0` - Read the `adc_ibat` field.
    ///
    /// IBAT ADC reading with 5mΩ sense resistor. Note the charger only measures discharging current (negative voltage) under battery only or OTG modes, and only measure charging current(positive voltage) when valid adapter is plugged in.
    #[doc(alias = "ADC_IBAT")]
    #[must_use]
    pub fn adc_ibat(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AdcIbat {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AdcIbat {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AdcIbat> for [u8; 2] {
    fn from(val: AdcIbat) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AdcIbat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AdcIbat");
        d.field("adc_ibat", &self.adc_ibat());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcIbat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcIbat {{ ");
        defmt::write!(f, "adc_ibat: {=u16}, ", &self.adc_ibat());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AdcIbat {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AdcIbat {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AdcIbat {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AdcIbat {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AdcIbat {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AdcIbat {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AdcIbat {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ADC_VBUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdcVbus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AdcVbus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AdcVbus {
    /// `15:0` - Read the `adc_vbus` field.
    ///
    /// VBUS ADC reading.
    #[doc(alias = "ADC_VBUS")]
    #[must_use]
    pub fn adc_vbus(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AdcVbus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AdcVbus {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AdcVbus> for [u8; 2] {
    fn from(val: AdcVbus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AdcVbus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AdcVbus");
        d.field("adc_vbus", &self.adc_vbus());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcVbus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcVbus {{ ");
        defmt::write!(f, "adc_vbus: {=u16}, ", &self.adc_vbus());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AdcVbus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AdcVbus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AdcVbus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AdcVbus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AdcVbus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AdcVbus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AdcVbus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "IIN_DPM")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct IinDpm {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for IinDpm {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl IinDpm {
    /// `10:2` - Read the `iin_host` field.
    ///
    /// Input current setting with 10mΩ sense resistor.
    #[doc(alias = "IIN_HOST")]
    #[must_use]
    pub fn iin_host(&self) -> u16 {
        let start = 2;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `10:2` - Set the `iin_host` field.
    ///
    /// Input current setting with 10mΩ sense resistor.
    #[doc(alias = "IIN_HOST")]
    pub fn set_iin_host(&mut self, value: u16) {
        let start = 2;
        let end = 10;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for IinDpm {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for IinDpm {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<IinDpm> for [u8; 2] {
    fn from(val: IinDpm) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for IinDpm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("IinDpm");
        d.field("iin_host", &self.iin_host());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IinDpm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IinDpm {{ ");
        defmt::write!(f, "iin_host: {=u16}, ", &self.iin_host());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for IinDpm {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for IinDpm {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for IinDpm {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for IinDpm {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for IinDpm {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for IinDpm {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for IinDpm {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "PROCHOT_STATUS_REG")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ProchotStatusReg {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ProchotStatusReg {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ProchotStatusReg {
    /// `bit 0` - Read the `stat_adapter_removal` field.
    ///
    /// Adapter removed?
    #[doc(alias = "STAT_ADAPTER_REMOVAL")]
    #[must_use]
    pub fn stat_adapter_removal(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `stat_battery_removal` field.
    ///
    /// Battery removed?
    #[doc(alias = "STAT_BATTERY_REMOVAL")]
    #[must_use]
    pub fn stat_battery_removal(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `stat_vsys` field.
    ///
    /// VSYS status triggered?
    #[doc(alias = "STAT_VSYS")]
    #[must_use]
    pub fn stat_vsys(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `stat_idchg_1` field.
    ///
    /// IDCHG1 status triggered?
    #[doc(alias = "STAT_IDCHG1")]
    #[must_use]
    pub fn stat_idchg_1(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `stat_inom` field.
    ///
    /// INOM status triggered?
    #[doc(alias = "STAT_INOM")]
    #[must_use]
    pub fn stat_inom(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `stat_icrit` field.
    ///
    /// ICRIT status triggered?
    #[doc(alias = "STAT_ICRIT")]
    #[must_use]
    pub fn stat_icrit(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `stat_comp` field.
    ///
    /// COMP status triggered?
    #[doc(alias = "STAT_COMP")]
    #[must_use]
    pub fn stat_comp(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `stat_vindpm` field.
    ///
    /// PROCHOT Profile VINDPM status bit, once triggered 1b, PROCHOT pin is low until host writes this status bit to 0b when PP_VINDPM = 1b.
    #[doc(alias = "STAT_VINDPM")]
    #[must_use]
    pub fn stat_vindpm(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `stat_exit_vap` field.
    ///
    /// PROCHOT_EXIT_VAP is active?
    #[doc(alias = "STAT_EXIT_VAP")]
    #[must_use]
    pub fn stat_exit_vap(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `stat_vap_fail` field.
    ///
    /// In VAP failure?
    #[doc(alias = "STAT_VAP_FAIL")]
    #[must_use]
    pub fn stat_vap_fail(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `tshut` field.
    ///
    /// TSHUT triggered?
    #[doc(alias = "TSHUT")]
    #[must_use]
    pub fn tshut(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `prochot_clear` field.
    ///
    /// PROCHOT Pulse Clear. Clear PROCHOT pulse when EN_PROCHOT_EXT=0b.
    #[doc(alias = "PROCHOT_CLEAR")]
    #[must_use]
    pub fn prochot_clear(&self) -> ProchotClear {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `13:12` - Read the `prochot_width` field.
    ///
    /// PROCHOT Pulse Width when EN_PROCHOT_EXT = 0b.
    #[doc(alias = "PROCHOT_WIDTH")]
    #[must_use]
    pub fn prochot_width(&self) -> ProchotPulseWidth {
        let start = 12;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 14` - Read the `en_prochot_ext` field.
    ///
    /// PROCHOT Pulse Extension Enable. When pulse extension is enabled, keep the PROCHOT pin voltage LOW until host writes PROCHOT_CLEAR= 0b.
    #[doc(alias = "EN_PROCHOT_EXT")]
    #[must_use]
    pub fn en_prochot_ext(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Set the `stat_vindpm` field.
    ///
    /// PROCHOT Profile VINDPM status bit, once triggered 1b, PROCHOT pin is low until host writes this status bit to 0b when PP_VINDPM = 1b.
    #[doc(alias = "STAT_VINDPM")]
    pub fn set_stat_vindpm(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `stat_exit_vap` field.
    ///
    /// PROCHOT_EXIT_VAP is active?
    #[doc(alias = "STAT_EXIT_VAP")]
    pub fn set_stat_exit_vap(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `stat_vap_fail` field.
    ///
    /// In VAP failure?
    #[doc(alias = "STAT_VAP_FAIL")]
    pub fn set_stat_vap_fail(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `prochot_clear` field.
    ///
    /// PROCHOT Pulse Clear. Clear PROCHOT pulse when EN_PROCHOT_EXT=0b.
    #[doc(alias = "PROCHOT_CLEAR")]
    pub fn set_prochot_clear(&mut self, value: ProchotClear) {
        let start = 11;
        let end = 11;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `13:12` - Set the `prochot_width` field.
    ///
    /// PROCHOT Pulse Width when EN_PROCHOT_EXT = 0b.
    #[doc(alias = "PROCHOT_WIDTH")]
    pub fn set_prochot_width(&mut self, value: ProchotPulseWidth) {
        let start = 12;
        let end = 13;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `en_prochot_ext` field.
    ///
    /// PROCHOT Pulse Extension Enable. When pulse extension is enabled, keep the PROCHOT pin voltage LOW until host writes PROCHOT_CLEAR= 0b.
    #[doc(alias = "EN_PROCHOT_EXT")]
    pub fn set_en_prochot_ext(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ProchotStatusReg {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ProchotStatusReg {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ProchotStatusReg> for [u8; 2] {
    fn from(val: ProchotStatusReg) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ProchotStatusReg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ProchotStatusReg");
        d.field("stat_adapter_removal", &self.stat_adapter_removal());
        d.field("stat_battery_removal", &self.stat_battery_removal());
        d.field("stat_vsys", &self.stat_vsys());
        d.field("stat_idchg_1", &self.stat_idchg_1());
        d.field("stat_inom", &self.stat_inom());
        d.field("stat_icrit", &self.stat_icrit());
        d.field("stat_comp", &self.stat_comp());
        d.field("stat_vindpm", &self.stat_vindpm());
        d.field("stat_exit_vap", &self.stat_exit_vap());
        d.field("stat_vap_fail", &self.stat_vap_fail());
        d.field("tshut", &self.tshut());
        d.field("prochot_clear", &self.prochot_clear());
        d.field("prochot_width", &self.prochot_width());
        d.field("en_prochot_ext", &self.en_prochot_ext());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ProchotStatusReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ProchotStatusReg {{ ");
        defmt::write!(f, "stat_adapter_removal: {=bool}, ", &self.stat_adapter_removal());
        defmt::write!(f, "stat_battery_removal: {=bool}, ", &self.stat_battery_removal());
        defmt::write!(f, "stat_vsys: {=bool}, ", &self.stat_vsys());
        defmt::write!(f, "stat_idchg_1: {=bool}, ", &self.stat_idchg_1());
        defmt::write!(f, "stat_inom: {=bool}, ", &self.stat_inom());
        defmt::write!(f, "stat_icrit: {=bool}, ", &self.stat_icrit());
        defmt::write!(f, "stat_comp: {=bool}, ", &self.stat_comp());
        defmt::write!(f, "stat_vindpm: {=bool}, ", &self.stat_vindpm());
        defmt::write!(f, "stat_exit_vap: {=bool}, ", &self.stat_exit_vap());
        defmt::write!(f, "stat_vap_fail: {=bool}, ", &self.stat_vap_fail());
        defmt::write!(f, "tshut: {=bool}, ", &self.tshut());
        defmt::write!(f, "prochot_clear: {}, ", &self.prochot_clear());
        defmt::write!(f, "prochot_width: {}, ", &self.prochot_width());
        defmt::write!(f, "en_prochot_ext: {=bool}, ", &self.en_prochot_ext());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ProchotStatusReg {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ProchotStatusReg {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ProchotStatusReg {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ProchotStatusReg {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ProchotStatusReg {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ProchotStatusReg {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ProchotStatusReg {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGER_STATUS_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargerStatus1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargerStatus1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargerStatus1 {
    /// `bit 0` - Read the `fault_otg_uvp` field.
    ///
    /// OTG_UVP fault detected.
    #[doc(alias = "FAULT_OTG_UVP")]
    #[must_use]
    pub fn fault_otg_uvp(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `fault_otg_ovp` field.
    ///
    /// OTG_OVP fault detected.
    #[doc(alias = "FAULT_OTG_OVP")]
    #[must_use]
    pub fn fault_otg_ovp(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `fault_frc_conv_off` field.
    ///
    /// OTG_OVP fault detected. Force converter off when independent comparator is triggered low effective.
    #[doc(alias = "FAULT_FRC_CONV_OFF")]
    #[must_use]
    pub fn fault_frc_conv_off(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `fault_vsys_uvp` field.
    ///
    /// VSYS_UVP fault status and clear. It is latched until a clear from host by writing this bit to 0.
    #[doc(alias = "FAULT_VSYS_UVP")]
    #[must_use]
    pub fn fault_vsys_uvp(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `fault_sysovp` field.
    ///
    /// SYSOVP fault status and Clear. When the SYSOVP occurs, this bit is set HIGH. As long as this bit is high, the converter is disabled. After the SYSOVP is removed, the user must write a 0 to this bit or unplug the adapter to clear the SYSOVP condition to enable the converter again.
    #[doc(alias = "FAULT_SYSOVP")]
    #[must_use]
    pub fn fault_sysovp(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `fault_acoc` field.
    ///
    /// ACOC fault detected.
    #[doc(alias = "FAULT_ACOC")]
    #[must_use]
    pub fn fault_acoc(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `fault_batdoc` field.
    ///
    /// BATDOC fault detected.
    #[doc(alias = "FAULT_BATDOC")]
    #[must_use]
    pub fn fault_batdoc(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `fault_acov` field.
    ///
    /// ACOV fault detected.
    #[doc(alias = "FAULT_ACOV")]
    #[must_use]
    pub fn fault_acov(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `in_otg` field.
    ///
    /// In OTG?
    #[doc(alias = "IN_OTG")]
    #[must_use]
    pub fn in_otg(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `fault_batcoc` field.
    ///
    /// BATCOC fault detected.
    #[doc(alias = "FAULT_BATCOC")]
    #[must_use]
    pub fn fault_batcoc(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `fault_sc_vbusacp` field.
    ///
    /// VBUSACP fault detected.
    #[doc(alias = "FAULT_SC_VBUSACP")]
    #[must_use]
    pub fn fault_sc_vbusacp(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `in_iin_dpm` field.
    ///
    /// In IIN_DPM or current regulation during OTG mode?
    #[doc(alias = "IN_IIN_DPM")]
    #[must_use]
    pub fn in_iin_dpm(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `in_vindpm` field.
    ///
    /// In VINDPM or boltage regulation during OTG mode?
    #[doc(alias = "IN_VINDPM")]
    #[must_use]
    pub fn in_vindpm(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `in_vap` field.
    ///
    /// VAP (Vmin Active Protection) enabled?
    #[doc(alias = "IN_VAP")]
    #[must_use]
    pub fn in_vap(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `ico_done` field.
    ///
    /// After the ICO routine is successfully executed, the bit goes 1.
    #[doc(alias = "ICO_DONE")]
    #[must_use]
    pub fn ico_done(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `stat_ac` field.
    ///
    /// Input source status, STAT_AC is active as long as valid VBUS source exist.
    #[doc(alias = "STAT_AC")]
    #[must_use]
    pub fn stat_ac(&self) -> InputSrcStat {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 3` - Set the `fault_vsys_uvp` field.
    ///
    /// VSYS_UVP fault status and clear. It is latched until a clear from host by writing this bit to 0.
    #[doc(alias = "FAULT_VSYS_UVP")]
    pub fn set_fault_vsys_uvp(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `fault_sysovp` field.
    ///
    /// SYSOVP fault status and Clear. When the SYSOVP occurs, this bit is set HIGH. As long as this bit is high, the converter is disabled. After the SYSOVP is removed, the user must write a 0 to this bit or unplug the adapter to clear the SYSOVP condition to enable the converter again.
    #[doc(alias = "FAULT_SYSOVP")]
    pub fn set_fault_sysovp(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargerStatus1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargerStatus1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargerStatus1> for [u8; 2] {
    fn from(val: ChargerStatus1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargerStatus1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargerStatus1");
        d.field("fault_otg_uvp", &self.fault_otg_uvp());
        d.field("fault_otg_ovp", &self.fault_otg_ovp());
        d.field("fault_frc_conv_off", &self.fault_frc_conv_off());
        d.field("fault_vsys_uvp", &self.fault_vsys_uvp());
        d.field("fault_sysovp", &self.fault_sysovp());
        d.field("fault_acoc", &self.fault_acoc());
        d.field("fault_batdoc", &self.fault_batdoc());
        d.field("fault_acov", &self.fault_acov());
        d.field("in_otg", &self.in_otg());
        d.field("fault_batcoc", &self.fault_batcoc());
        d.field("fault_sc_vbusacp", &self.fault_sc_vbusacp());
        d.field("in_iin_dpm", &self.in_iin_dpm());
        d.field("in_vindpm", &self.in_vindpm());
        d.field("in_vap", &self.in_vap());
        d.field("ico_done", &self.ico_done());
        d.field("stat_ac", &self.stat_ac());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargerStatus1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargerStatus1 {{ ");
        defmt::write!(f, "fault_otg_uvp: {=bool}, ", &self.fault_otg_uvp());
        defmt::write!(f, "fault_otg_ovp: {=bool}, ", &self.fault_otg_ovp());
        defmt::write!(f, "fault_frc_conv_off: {=bool}, ", &self.fault_frc_conv_off());
        defmt::write!(f, "fault_vsys_uvp: {=bool}, ", &self.fault_vsys_uvp());
        defmt::write!(f, "fault_sysovp: {=bool}, ", &self.fault_sysovp());
        defmt::write!(f, "fault_acoc: {=bool}, ", &self.fault_acoc());
        defmt::write!(f, "fault_batdoc: {=bool}, ", &self.fault_batdoc());
        defmt::write!(f, "fault_acov: {=bool}, ", &self.fault_acov());
        defmt::write!(f, "in_otg: {=bool}, ", &self.in_otg());
        defmt::write!(f, "fault_batcoc: {=bool}, ", &self.fault_batcoc());
        defmt::write!(f, "fault_sc_vbusacp: {=bool}, ", &self.fault_sc_vbusacp());
        defmt::write!(f, "in_iin_dpm: {=bool}, ", &self.in_iin_dpm());
        defmt::write!(f, "in_vindpm: {=bool}, ", &self.in_vindpm());
        defmt::write!(f, "in_vap: {=bool}, ", &self.in_vap());
        defmt::write!(f, "ico_done: {=bool}, ", &self.ico_done());
        defmt::write!(f, "stat_ac: {}, ", &self.stat_ac());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargerStatus1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargerStatus1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargerStatus1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargerStatus1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargerStatus1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargerStatus1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargerStatus1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ADC_CMPIN_TR")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdcCmpinTr {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AdcCmpinTr {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AdcCmpinTr {
    /// `15:0` - Read the `adc_cmpin_tr` field.
    ///
    /// CMPIN_TR pin voltage ADC reading.
    #[doc(alias = "ADC_CMPIN_TR")]
    #[must_use]
    pub fn adc_cmpin_tr(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AdcCmpinTr {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AdcCmpinTr {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AdcCmpinTr> for [u8; 2] {
    fn from(val: AdcCmpinTr) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AdcCmpinTr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AdcCmpinTr");
        d.field("adc_cmpin_tr", &self.adc_cmpin_tr());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCmpinTr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcCmpinTr {{ ");
        defmt::write!(f, "adc_cmpin_tr: {=u16}, ", &self.adc_cmpin_tr());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AdcCmpinTr {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AdcCmpinTr {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AdcCmpinTr {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AdcCmpinTr {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AdcCmpinTr {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AdcCmpinTr {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AdcCmpinTr {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ADC_PSYS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdcPsys {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AdcPsys {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AdcPsys {
    /// `15:0` - Read the `adc_psys` field.
    ///
    /// System Power PSYS ADC reading.
    #[doc(alias = "ADC_PSYS")]
    #[must_use]
    pub fn adc_psys(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AdcPsys {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AdcPsys {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AdcPsys> for [u8; 2] {
    fn from(val: AdcPsys) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AdcPsys {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AdcPsys");
        d.field("adc_psys", &self.adc_psys());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcPsys {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcPsys {{ ");
        defmt::write!(f, "adc_psys: {=u16}, ", &self.adc_psys());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AdcPsys {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AdcPsys {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AdcPsys {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AdcPsys {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AdcPsys {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AdcPsys {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AdcPsys {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ADC_VBAT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdcVbat {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AdcVbat {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AdcVbat {
    /// `15:0` - Read the `adc_vbat` field.
    ///
    /// VBAT ADC reading.
    #[doc(alias = "ADC_VBAT")]
    #[must_use]
    pub fn adc_vbat(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AdcVbat {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AdcVbat {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AdcVbat> for [u8; 2] {
    fn from(val: AdcVbat) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AdcVbat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AdcVbat");
        d.field("adc_vbat", &self.adc_vbat());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcVbat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcVbat {{ ");
        defmt::write!(f, "adc_vbat: {=u16}, ", &self.adc_vbat());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AdcVbat {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AdcVbat {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AdcVbat {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AdcVbat {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AdcVbat {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AdcVbat {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AdcVbat {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGER_STATUS_0")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargerStatus0 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargerStatus0 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargerStatus0 {
    /// `bit 3` - Read the `fault_regn` field.
    ///
    /// REGN fault detected.
    #[doc(alias = "FAULT_REGN")]
    #[must_use]
    pub fn fault_regn(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `fault_ocp` field.
    ///
    /// OCP fault detected.
    #[doc(alias = "FAULT_OCP")]
    #[must_use]
    pub fn fault_ocp(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `fault_batovp` field.
    ///
    /// BATOVP fault detected.
    #[doc(alias = "FAULT_BATOVP")]
    #[must_use]
    pub fn fault_batovp(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `10:8` - Read the `mode_stat` field.
    ///
    /// MODE pin program status.
    #[doc(alias = "MODE_STAT")]
    #[must_use]
    pub fn mode_stat(&self) -> ModePinProgStatus {
        let start = 8;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 11` - Read the `treg_stat` field.
    ///
    /// Temperature regulation status.
    #[doc(alias = "TREG_STAT")]
    #[must_use]
    pub fn treg_stat(&self) -> TempRegulationStat {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 12` - Read the `chg_tmr_stat` field.
    ///
    /// Charge safety timer status.
    #[doc(alias = "CHG_TMR_STAT")]
    #[must_use]
    pub fn chg_tmr_stat(&self) -> ChrgSafetyTimerStat {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `15:13` - Read the `chrg_stat` field.
    ///
    /// Charge Cycle Status.
    #[doc(alias = "CHRG_STAT")]
    #[must_use]
    pub fn chrg_stat(&self) -> ChrgCycleStat {
        let start = 13;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
}
impl Default for ChargerStatus0 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargerStatus0 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargerStatus0> for [u8; 2] {
    fn from(val: ChargerStatus0) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargerStatus0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargerStatus0");
        d.field("fault_regn", &self.fault_regn());
        d.field("fault_ocp", &self.fault_ocp());
        d.field("fault_batovp", &self.fault_batovp());
        d.field("mode_stat", &self.mode_stat());
        d.field("treg_stat", &self.treg_stat());
        d.field("chg_tmr_stat", &self.chg_tmr_stat());
        d.field("chrg_stat", &self.chrg_stat());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargerStatus0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargerStatus0 {{ ");
        defmt::write!(f, "fault_regn: {=bool}, ", &self.fault_regn());
        defmt::write!(f, "fault_ocp: {=bool}, ", &self.fault_ocp());
        defmt::write!(f, "fault_batovp: {=bool}, ", &self.fault_batovp());
        defmt::write!(f, "mode_stat: {}, ", &self.mode_stat());
        defmt::write!(f, "treg_stat: {}, ", &self.treg_stat());
        defmt::write!(f, "chg_tmr_stat: {}, ", &self.chg_tmr_stat());
        defmt::write!(f, "chrg_stat: {}, ", &self.chrg_stat());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargerStatus0 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargerStatus0 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargerStatus0 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargerStatus0 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargerStatus0 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargerStatus0 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargerStatus0 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AUTO_CHARGE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AutoCharge {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AutoCharge {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AutoCharge {
    /// `1:0` - Read the `acov_adj` field.
    ///
    /// ACOV protection threshold adjustment.
    #[doc(alias = "ACOV_ADJ")]
    #[must_use]
    pub fn acov_adj(&self) -> AcovThreshold {
        let start = 0;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 2` - Read the `thermal_deg` field.
    ///
    /// Adjust TREG thermal deglitch time to trigger prochot profile pull down pulse.
    #[doc(alias = "THERMAL_DEG")]
    #[must_use]
    pub fn thermal_deg(&self) -> ThermalDeglitchTime {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 3` - Read the `stat_thermal` field.
    ///
    /// PROCHOT profile status bit for TREG thermal overheat (CMPIN_TR< 1.1V). The status is latched until a read from host.
    #[doc(alias = "STAT_THERMAL")]
    #[must_use]
    pub fn stat_thermal(&self) -> ProchotStatusOverheat {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 4` - Read the `pp_thermal` field.
    ///
    /// Enable temperature regulation(TREG) for PROCHOT profile.
    #[doc(alias = "PP_THERMAL")]
    #[must_use]
    pub fn pp_thermal(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `en_treg` field.
    ///
    /// Enable temperature regulation function.
    #[doc(alias = "EN_TREG")]
    #[must_use]
    pub fn en_treg(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `en_chg_tmr` field.
    ///
    /// Enable charge safety timer.
    #[doc(alias = "EN_CHG_TMR")]
    #[must_use]
    pub fn en_chg_tmr(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `en_tmr_2_x` field.
    ///
    /// Charge Safety Timer speed control (Note changing the state of EN_TMR2X only impacts the rate at which the counter is counting and has no effect on any existing accumulated count).
    #[doc(alias = "EN_TMR2X")]
    #[must_use]
    pub fn en_tmr_2_x(&self) -> ChgTmrSpeedCtrl {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `9:8` - Read the `chg_tmr` field.
    ///
    /// Automatic Charge Safety Timer control.
    #[doc(alias = "CHG_TMR")]
    #[must_use]
    pub fn chg_tmr(&self) -> ChgTmrCtrl {
        let start = 8;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `13:10` - Read the `vrechg` field.
    ///
    /// Battery automatic recharge threshold below CHARGE_VOLTAGE().
    #[doc(alias = "VRECHG")]
    #[must_use]
    pub fn vrechg(&self) -> u8 {
        let start = 10;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 14` - Read the `chrg_ok_int` field.
    ///
    /// Enable CHRG_OK pin for interrupt function.
    #[doc(alias = "CHRG_OK_INT")]
    #[must_use]
    pub fn chrg_ok_int(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `en_auto_chg` field.
    ///
    /// Automatic charge control (recharge and terminate battery charging automatically).
    #[doc(alias = "EN_AUTO_CHG")]
    #[must_use]
    pub fn en_auto_chg(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `1:0` - Set the `acov_adj` field.
    ///
    /// ACOV protection threshold adjustment.
    #[doc(alias = "ACOV_ADJ")]
    pub fn set_acov_adj(&mut self, value: AcovThreshold) {
        let start = 0;
        let end = 1;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `thermal_deg` field.
    ///
    /// Adjust TREG thermal deglitch time to trigger prochot profile pull down pulse.
    #[doc(alias = "THERMAL_DEG")]
    pub fn set_thermal_deg(&mut self, value: ThermalDeglitchTime) {
        let start = 2;
        let end = 2;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `pp_thermal` field.
    ///
    /// Enable temperature regulation(TREG) for PROCHOT profile.
    #[doc(alias = "PP_THERMAL")]
    pub fn set_pp_thermal(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `en_treg` field.
    ///
    /// Enable temperature regulation function.
    #[doc(alias = "EN_TREG")]
    pub fn set_en_treg(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `en_chg_tmr` field.
    ///
    /// Enable charge safety timer.
    #[doc(alias = "EN_CHG_TMR")]
    pub fn set_en_chg_tmr(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `en_tmr_2_x` field.
    ///
    /// Charge Safety Timer speed control (Note changing the state of EN_TMR2X only impacts the rate at which the counter is counting and has no effect on any existing accumulated count).
    #[doc(alias = "EN_TMR2X")]
    pub fn set_en_tmr_2_x(&mut self, value: ChgTmrSpeedCtrl) {
        let start = 7;
        let end = 7;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `9:8` - Set the `chg_tmr` field.
    ///
    /// Automatic Charge Safety Timer control.
    #[doc(alias = "CHG_TMR")]
    pub fn set_chg_tmr(&mut self, value: ChgTmrCtrl) {
        let start = 8;
        let end = 9;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `13:10` - Set the `vrechg` field.
    ///
    /// Battery automatic recharge threshold below CHARGE_VOLTAGE().
    #[doc(alias = "VRECHG")]
    pub fn set_vrechg(&mut self, value: u8) {
        let start = 10;
        let end = 13;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `chrg_ok_int` field.
    ///
    /// Enable CHRG_OK pin for interrupt function.
    #[doc(alias = "CHRG_OK_INT")]
    pub fn set_chrg_ok_int(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `en_auto_chg` field.
    ///
    /// Automatic charge control (recharge and terminate battery charging automatically).
    #[doc(alias = "EN_AUTO_CHG")]
    pub fn set_en_auto_chg(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AutoCharge {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AutoCharge {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AutoCharge> for [u8; 2] {
    fn from(val: AutoCharge) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AutoCharge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AutoCharge");
        d.field("acov_adj", &self.acov_adj());
        d.field("thermal_deg", &self.thermal_deg());
        d.field("stat_thermal", &self.stat_thermal());
        d.field("pp_thermal", &self.pp_thermal());
        d.field("en_treg", &self.en_treg());
        d.field("en_chg_tmr", &self.en_chg_tmr());
        d.field("en_tmr_2_x", &self.en_tmr_2_x());
        d.field("chg_tmr", &self.chg_tmr());
        d.field("vrechg", &self.vrechg());
        d.field("chrg_ok_int", &self.chrg_ok_int());
        d.field("en_auto_chg", &self.en_auto_chg());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AutoCharge {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AutoCharge {{ ");
        defmt::write!(f, "acov_adj: {}, ", &self.acov_adj());
        defmt::write!(f, "thermal_deg: {}, ", &self.thermal_deg());
        defmt::write!(f, "stat_thermal: {}, ", &self.stat_thermal());
        defmt::write!(f, "pp_thermal: {=bool}, ", &self.pp_thermal());
        defmt::write!(f, "en_treg: {=bool}, ", &self.en_treg());
        defmt::write!(f, "en_chg_tmr: {=bool}, ", &self.en_chg_tmr());
        defmt::write!(f, "en_tmr_2_x: {}, ", &self.en_tmr_2_x());
        defmt::write!(f, "chg_tmr: {}, ", &self.chg_tmr());
        defmt::write!(f, "vrechg: {=u8}, ", &self.vrechg());
        defmt::write!(f, "chrg_ok_int: {=bool}, ", &self.chrg_ok_int());
        defmt::write!(f, "en_auto_chg: {=bool}, ", &self.en_auto_chg());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AutoCharge {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AutoCharge {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AutoCharge {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AutoCharge {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AutoCharge {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AutoCharge {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AutoCharge {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_OPTION_5")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeOption5 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeOption5 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeOption5 {
    /// `1:0` - Read the `ph_drop_deg` field.
    ///
    /// Adjust dual phase to single phase (phase dropping transition) deglitch time.
    #[doc(alias = "PH_DROP_DEG")]
    #[must_use]
    pub fn ph_drop_deg(&self) -> PhaseDroppingTransitionDeglitchTime {
        let start = 0;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `3:2` - Read the `ph_add_deg` field.
    ///
    /// Adjust single phase to dual phase (phase adding transition) deglitch time.
    #[doc(alias = "PH_ADD_DEG")]
    #[must_use]
    pub fn ph_add_deg(&self) -> PhaseAddingTransitionDeglitchTime {
        let start = 2;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 4` - Read the `force_single` field.
    ///
    /// Force single phase operation under buck mode when quasi dual phase is chosen through MODE pin programming.
    #[doc(alias = "FORCE_SINGLE")]
    #[must_use]
    pub fn force_single(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `7:5` - Read the `single_dual_trans_th` field.
    ///
    /// Buck mode single to dual phase transition threshold adjustment based on output load current.
    #[doc(alias = "SINGLE_DUAL_TRANS_TH")]
    #[must_use]
    pub fn single_dual_trans_th(&self) -> SingleDualTransThreshold {
        let start = 5;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `10:9` - Read the `batcoc_config` field.
    ///
    /// Disable BATCOC and configure BATCOC thresholds across SRP-SRN.
    #[doc(alias = "BATCOC_CONFIG")]
    #[must_use]
    pub fn batcoc_config(&self) -> BatcocThreshold {
        let start = 9;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 11` - Read the `en_regn_lwpwr` field.
    ///
    /// Enable REGN with scale down current 5mA capability under battery only and low power mode.
    #[doc(alias = "EN_REGN_LWPWR")]
    #[must_use]
    pub fn en_regn_lwpwr(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `regn_ext` field.
    ///
    /// Enable external 5V overdrive for REGN.
    #[doc(alias = "REGN_EXT")]
    #[must_use]
    pub fn regn_ext(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `cmpin_tr_select` field.
    ///
    /// CMPIN_TR pin function selection.
    #[doc(alias = "CMPIN_TR_SELECT")]
    #[must_use]
    pub fn cmpin_tr_select(&self) -> CmpinFuncSelect {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 14` - Read the `wd_rst` field.
    ///
    /// Reset watch dog timer control.
    #[doc(alias = "WD_RST")]
    #[must_use]
    pub fn wd_rst(&self) -> WatchDogReset {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 15` - Read the `ptm_exit_light_load` field.
    ///
    /// Enable PTM auto exit under light load.
    #[doc(alias = "PTM_EXIT_LIGHT_LOAD")]
    #[must_use]
    pub fn ptm_exit_light_load(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `1:0` - Set the `ph_drop_deg` field.
    ///
    /// Adjust dual phase to single phase (phase dropping transition) deglitch time.
    #[doc(alias = "PH_DROP_DEG")]
    pub fn set_ph_drop_deg(&mut self, value: PhaseDroppingTransitionDeglitchTime) {
        let start = 0;
        let end = 1;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `3:2` - Set the `ph_add_deg` field.
    ///
    /// Adjust single phase to dual phase (phase adding transition) deglitch time.
    #[doc(alias = "PH_ADD_DEG")]
    pub fn set_ph_add_deg(&mut self, value: PhaseAddingTransitionDeglitchTime) {
        let start = 2;
        let end = 3;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `force_single` field.
    ///
    /// Force single phase operation under buck mode when quasi dual phase is chosen through MODE pin programming.
    #[doc(alias = "FORCE_SINGLE")]
    pub fn set_force_single(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `7:5` - Set the `single_dual_trans_th` field.
    ///
    /// Buck mode single to dual phase transition threshold adjustment based on output load current.
    #[doc(alias = "SINGLE_DUAL_TRANS_TH")]
    pub fn set_single_dual_trans_th(&mut self, value: SingleDualTransThreshold) {
        let start = 5;
        let end = 7;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `10:9` - Set the `batcoc_config` field.
    ///
    /// Disable BATCOC and configure BATCOC thresholds across SRP-SRN.
    #[doc(alias = "BATCOC_CONFIG")]
    pub fn set_batcoc_config(&mut self, value: BatcocThreshold) {
        let start = 9;
        let end = 10;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `en_regn_lwpwr` field.
    ///
    /// Enable REGN with scale down current 5mA capability under battery only and low power mode.
    #[doc(alias = "EN_REGN_LWPWR")]
    pub fn set_en_regn_lwpwr(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `regn_ext` field.
    ///
    /// Enable external 5V overdrive for REGN.
    #[doc(alias = "REGN_EXT")]
    pub fn set_regn_ext(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `cmpin_tr_select` field.
    ///
    /// CMPIN_TR pin function selection.
    #[doc(alias = "CMPIN_TR_SELECT")]
    pub fn set_cmpin_tr_select(&mut self, value: CmpinFuncSelect) {
        let start = 13;
        let end = 13;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `wd_rst` field.
    ///
    /// Reset watch dog timer control.
    #[doc(alias = "WD_RST")]
    pub fn set_wd_rst(&mut self, value: WatchDogReset) {
        let start = 14;
        let end = 14;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `ptm_exit_light_load` field.
    ///
    /// Enable PTM auto exit under light load.
    #[doc(alias = "PTM_EXIT_LIGHT_LOAD")]
    pub fn set_ptm_exit_light_load(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeOption5 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeOption5 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeOption5> for [u8; 2] {
    fn from(val: ChargeOption5) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeOption5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeOption5");
        d.field("ph_drop_deg", &self.ph_drop_deg());
        d.field("ph_add_deg", &self.ph_add_deg());
        d.field("force_single", &self.force_single());
        d.field("single_dual_trans_th", &self.single_dual_trans_th());
        d.field("batcoc_config", &self.batcoc_config());
        d.field("en_regn_lwpwr", &self.en_regn_lwpwr());
        d.field("regn_ext", &self.regn_ext());
        d.field("cmpin_tr_select", &self.cmpin_tr_select());
        d.field("wd_rst", &self.wd_rst());
        d.field("ptm_exit_light_load", &self.ptm_exit_light_load());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeOption5 {{ ");
        defmt::write!(f, "ph_drop_deg: {}, ", &self.ph_drop_deg());
        defmt::write!(f, "ph_add_deg: {}, ", &self.ph_add_deg());
        defmt::write!(f, "force_single: {=bool}, ", &self.force_single());
        defmt::write!(f, "single_dual_trans_th: {}, ", &self.single_dual_trans_th());
        defmt::write!(f, "batcoc_config: {}, ", &self.batcoc_config());
        defmt::write!(f, "en_regn_lwpwr: {=bool}, ", &self.en_regn_lwpwr());
        defmt::write!(f, "regn_ext: {=bool}, ", &self.regn_ext());
        defmt::write!(f, "cmpin_tr_select: {}, ", &self.cmpin_tr_select());
        defmt::write!(f, "wd_rst: {}, ", &self.wd_rst());
        defmt::write!(f, "ptm_exit_light_load: {=bool}, ", &self.ptm_exit_light_load());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeOption5 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeOption5 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeOption5 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeOption5 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeOption5 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeOption5 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeOption5 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "GATE_DRIVE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct GateDrive {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for GateDrive {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl GateDrive {
    /// `bit 1` - Read the `vsys_reg_slow` field.
    ///
    /// System regulation loop bandwidth slow down to reduce input current overshoot during load transient.
    #[doc(alias = "VSYS_REG_SLOW")]
    #[must_use]
    pub fn vsys_reg_slow(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `4:2` - Read the `lodrv_2_stat` field.
    ///
    /// Suggested LODRV2 LS MOSFET gate drive strength adjustment for both turn on and turn off.
    #[doc(alias = "LODRV2_STAT")]
    #[must_use]
    pub fn lodrv_2_stat(&self) -> Lodrv2GateDriveStrengthAdjustment {
        let start = 2;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `7:5` - Read the `hidrv_2_stat` field.
    ///
    /// Suggested HIDRV2 HS MOSFET gate drive strength adjustment for both turn on and turn off.
    #[doc(alias = "HIDRV2_STAT")]
    #[must_use]
    pub fn hidrv_2_stat(&self) -> Hidrv2GateDriveStrengthAdjustment {
        let start = 5;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 8` - Read the `batovp_extend` field.
    ///
    /// Enable BATOVP for both charge enable and disable scenarios including AC+battery and battery only.
    #[doc(alias = "BATOVP_EXTEND")]
    #[must_use]
    pub fn batovp_extend(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `12:10` - Read the `lodrv_1_stat` field.
    ///
    /// Suggested LODRV1_A and LODRV1_B LS MOSFET gate drive strength adjustment for both turn on and turn off.
    #[doc(alias = "LODRV1_STAT")]
    #[must_use]
    pub fn lodrv_1_stat(&self) -> Lodrv1GateDriveStrengthAdjustment {
        let start = 10;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `15:13` - Read the `hidrv_1_stat` field.
    ///
    /// Suggested HIDRV1_A and HIDRV1_B HS MOSFET gate drive strength adjustment for both turn on and turn off.
    #[doc(alias = "HIDRV1_STAT")]
    #[must_use]
    pub fn hidrv_1_stat(&self) -> Hidrv1GateDriveStrengthAdjustment {
        let start = 13;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 1` - Set the `vsys_reg_slow` field.
    ///
    /// System regulation loop bandwidth slow down to reduce input current overshoot during load transient.
    #[doc(alias = "VSYS_REG_SLOW")]
    pub fn set_vsys_reg_slow(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `4:2` - Set the `lodrv_2_stat` field.
    ///
    /// Suggested LODRV2 LS MOSFET gate drive strength adjustment for both turn on and turn off.
    #[doc(alias = "LODRV2_STAT")]
    pub fn set_lodrv_2_stat(&mut self, value: Lodrv2GateDriveStrengthAdjustment) {
        let start = 2;
        let end = 4;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `7:5` - Set the `hidrv_2_stat` field.
    ///
    /// Suggested HIDRV2 HS MOSFET gate drive strength adjustment for both turn on and turn off.
    #[doc(alias = "HIDRV2_STAT")]
    pub fn set_hidrv_2_stat(&mut self, value: Hidrv2GateDriveStrengthAdjustment) {
        let start = 5;
        let end = 7;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `batovp_extend` field.
    ///
    /// Enable BATOVP for both charge enable and disable scenarios including AC+battery and battery only.
    #[doc(alias = "BATOVP_EXTEND")]
    pub fn set_batovp_extend(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `12:10` - Set the `lodrv_1_stat` field.
    ///
    /// Suggested LODRV1_A and LODRV1_B LS MOSFET gate drive strength adjustment for both turn on and turn off.
    #[doc(alias = "LODRV1_STAT")]
    pub fn set_lodrv_1_stat(&mut self, value: Lodrv1GateDriveStrengthAdjustment) {
        let start = 10;
        let end = 12;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:13` - Set the `hidrv_1_stat` field.
    ///
    /// Suggested HIDRV1_A and HIDRV1_B HS MOSFET gate drive strength adjustment for both turn on and turn off.
    #[doc(alias = "HIDRV1_STAT")]
    pub fn set_hidrv_1_stat(&mut self, value: Hidrv1GateDriveStrengthAdjustment) {
        let start = 13;
        let end = 15;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for GateDrive {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for GateDrive {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<GateDrive> for [u8; 2] {
    fn from(val: GateDrive) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for GateDrive {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("GateDrive");
        d.field("vsys_reg_slow", &self.vsys_reg_slow());
        d.field("lodrv_2_stat", &self.lodrv_2_stat());
        d.field("hidrv_2_stat", &self.hidrv_2_stat());
        d.field("batovp_extend", &self.batovp_extend());
        d.field("lodrv_1_stat", &self.lodrv_1_stat());
        d.field("hidrv_1_stat", &self.hidrv_1_stat());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GateDrive {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GateDrive {{ ");
        defmt::write!(f, "vsys_reg_slow: {=bool}, ", &self.vsys_reg_slow());
        defmt::write!(f, "lodrv_2_stat: {}, ", &self.lodrv_2_stat());
        defmt::write!(f, "hidrv_2_stat: {}, ", &self.hidrv_2_stat());
        defmt::write!(f, "batovp_extend: {=bool}, ", &self.batovp_extend());
        defmt::write!(f, "lodrv_1_stat: {}, ", &self.lodrv_1_stat());
        defmt::write!(f, "hidrv_1_stat: {}, ", &self.hidrv_1_stat());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for GateDrive {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for GateDrive {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for GateDrive {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for GateDrive {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for GateDrive {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for GateDrive {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for GateDrive {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_PROFILE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeProfile {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeProfile {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeProfile {
    /// `7:0` - Read the `iterm` field.
    ///
    /// Termination current setting with 5mΩ sense resistor.
    #[doc(alias = "ITERM")]
    #[must_use]
    pub fn iterm(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `iprechg` field.
    ///
    /// Maximum precharge current clamp setting with 5mΩ sense resistor (The lower setting of CHARGE_CURRENT() and IPRECHG determine the practical precharge current when VBAT< VSYS_MIN()).
    #[doc(alias = "IPRECHG")]
    #[must_use]
    pub fn iprechg(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `iterm` field.
    ///
    /// Termination current setting with 5mΩ sense resistor.
    #[doc(alias = "ITERM")]
    pub fn set_iterm(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:8` - Set the `iprechg` field.
    ///
    /// Maximum precharge current clamp setting with 5mΩ sense resistor (The lower setting of CHARGE_CURRENT() and IPRECHG determine the practical precharge current when VBAT< VSYS_MIN()).
    #[doc(alias = "IPRECHG")]
    pub fn set_iprechg(&mut self, value: u8) {
        let start = 8;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeProfile {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeProfile {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeProfile> for [u8; 2] {
    fn from(val: ChargeProfile) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeProfile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeProfile");
        d.field("iterm", &self.iterm());
        d.field("iprechg", &self.iprechg());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeProfile {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeProfile {{ ");
        defmt::write!(f, "iterm: {=u8}, ", &self.iterm());
        defmt::write!(f, "iprechg: {=u8}, ", &self.iprechg());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeProfile {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeProfile {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeProfile {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeProfile {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeProfile {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeProfile {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeProfile {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "VSYS_MIN")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct VsysMin {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for VsysMin {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl VsysMin {
    /// `12:0` - Read the `vsys_min` field.
    ///
    /// Minimum system voltage configuration register.
    #[doc(alias = "VSYS_MIN")]
    #[must_use]
    pub fn vsys_min(&self) -> u16 {
        let start = 0;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `12:0` - Set the `vsys_min` field.
    ///
    /// Minimum system voltage configuration register.
    #[doc(alias = "VSYS_MIN")]
    pub fn set_vsys_min(&mut self, value: u16) {
        let start = 0;
        let end = 12;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for VsysMin {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for VsysMin {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<VsysMin> for [u8; 2] {
    fn from(val: VsysMin) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for VsysMin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("VsysMin");
        d.field("vsys_min", &self.vsys_min());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VsysMin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VsysMin {{ ");
        defmt::write!(f, "vsys_min: {=u16}, ", &self.vsys_min());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for VsysMin {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for VsysMin {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for VsysMin {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for VsysMin {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for VsysMin {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for VsysMin {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for VsysMin {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "OTG_VOLTAGE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct OtgVoltage {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for OtgVoltage {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl OtgVoltage {
    /// `12:2` - Read the `otg_voltage` field.
    ///
    /// OTG output voltage regulation.
    #[doc(alias = "OTG_VOLTAGE")]
    #[must_use]
    pub fn otg_voltage(&self) -> u16 {
        let start = 2;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `12:2` - Set the `otg_voltage` field.
    ///
    /// OTG output voltage regulation.
    #[doc(alias = "OTG_VOLTAGE")]
    pub fn set_otg_voltage(&mut self, value: u16) {
        let start = 2;
        let end = 12;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for OtgVoltage {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for OtgVoltage {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<OtgVoltage> for [u8; 2] {
    fn from(val: OtgVoltage) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for OtgVoltage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("OtgVoltage");
        d.field("otg_voltage", &self.otg_voltage());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtgVoltage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OtgVoltage {{ ");
        defmt::write!(f, "otg_voltage: {=u16}, ", &self.otg_voltage());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for OtgVoltage {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for OtgVoltage {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for OtgVoltage {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for OtgVoltage {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for OtgVoltage {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for OtgVoltage {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for OtgVoltage {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "OTG_CURRENT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct OtgCurrent {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for OtgCurrent {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl OtgCurrent {
    /// `10:2` - Read the `otg_current` field.
    ///
    /// OTG output current limit with 10mΩ Rac current sense.
    #[doc(alias = "OTG_CURRENT")]
    #[must_use]
    pub fn otg_current(&self) -> u16 {
        let start = 2;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `10:2` - Set the `otg_current` field.
    ///
    /// OTG output current limit with 10mΩ Rac current sense.
    #[doc(alias = "OTG_CURRENT")]
    pub fn set_otg_current(&mut self, value: u16) {
        let start = 2;
        let end = 10;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for OtgCurrent {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for OtgCurrent {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<OtgCurrent> for [u8; 2] {
    fn from(val: OtgCurrent) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for OtgCurrent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("OtgCurrent");
        d.field("otg_current", &self.otg_current());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtgCurrent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OtgCurrent {{ ");
        defmt::write!(f, "otg_current: {=u16}, ", &self.otg_current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for OtgCurrent {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for OtgCurrent {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for OtgCurrent {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for OtgCurrent {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for OtgCurrent {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for OtgCurrent {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for OtgCurrent {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "VINDPM")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Vindpm {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Vindpm {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Vindpm {
    /// `12:2` - Read the `vindpm` field.
    ///
    /// Input voltage limit.
    #[doc(alias = "VINDPM")]
    #[must_use]
    pub fn vindpm(&self) -> u16 {
        let start = 2;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `12:2` - Set the `vindpm` field.
    ///
    /// Input voltage limit.
    #[doc(alias = "VINDPM")]
    pub fn set_vindpm(&mut self, value: u16) {
        let start = 2;
        let end = 12;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for Vindpm {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Vindpm {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Vindpm> for [u8; 2] {
    fn from(val: Vindpm) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Vindpm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Vindpm");
        d.field("vindpm", &self.vindpm());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vindpm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Vindpm {{ ");
        defmt::write!(f, "vindpm: {=u16}, ", &self.vindpm());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Vindpm {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Vindpm {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Vindpm {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Vindpm {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Vindpm {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Vindpm {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Vindpm {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "IIN_HOST")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct IinHost {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for IinHost {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl IinHost {
    /// `10:2` - Read the `iin_host` field.
    ///
    /// Maximum input current limit with 10mΩ sense resistor.
    #[doc(alias = "IIN_HOST")]
    #[must_use]
    pub fn iin_host(&self) -> u16 {
        let start = 2;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `10:2` - Set the `iin_host` field.
    ///
    /// Maximum input current limit with 10mΩ sense resistor.
    #[doc(alias = "IIN_HOST")]
    pub fn set_iin_host(&mut self, value: u16) {
        let start = 2;
        let end = 10;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for IinHost {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for IinHost {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<IinHost> for [u8; 2] {
    fn from(val: IinHost) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for IinHost {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("IinHost");
        d.field("iin_host", &self.iin_host());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IinHost {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IinHost {{ ");
        defmt::write!(f, "iin_host: {=u16}, ", &self.iin_host());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for IinHost {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for IinHost {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for IinHost {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for IinHost {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for IinHost {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for IinHost {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for IinHost {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_VOLTAGE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeVoltage {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeVoltage {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeVoltage {
    /// `14:2` - Read the `charge_voltage` field.
    ///
    /// Charge voltage setting, in 4mV/bit steps.
    #[doc(alias = "CHARGE_VOLTAGE")]
    #[must_use]
    pub fn charge_voltage(&self) -> u16 {
        let start = 2;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `14:2` - Set the `charge_voltage` field.
    ///
    /// Charge voltage setting, in 4mV/bit steps.
    #[doc(alias = "CHARGE_VOLTAGE")]
    pub fn set_charge_voltage(&mut self, value: u16) {
        let start = 2;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeVoltage {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeVoltage {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeVoltage> for [u8; 2] {
    fn from(val: ChargeVoltage) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeVoltage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeVoltage");
        d.field("charge_voltage", &self.charge_voltage());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeVoltage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeVoltage {{ ");
        defmt::write!(f, "charge_voltage: {=u16}, ", &self.charge_voltage());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeVoltage {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeVoltage {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeVoltage {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeVoltage {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeVoltage {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeVoltage {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeVoltage {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_CURRENT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeCurrent {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeCurrent {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeCurrent {
    /// `13:3` - Read the `charge_current` field.
    ///
    /// Charge current setting with 5mΩ sense resistor, in 8mA/bit steps
    #[doc(alias = "CHARGE_CURRENT")]
    #[must_use]
    pub fn charge_current(&self) -> u16 {
        let start = 3;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `13:3` - Set the `charge_current` field.
    ///
    /// Charge current setting with 5mΩ sense resistor, in 8mA/bit steps
    #[doc(alias = "CHARGE_CURRENT")]
    pub fn set_charge_current(&mut self, value: u16) {
        let start = 3;
        let end = 13;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeCurrent {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeCurrent {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeCurrent> for [u8; 2] {
    fn from(val: ChargeCurrent) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeCurrent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeCurrent");
        d.field("charge_current", &self.charge_current());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeCurrent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeCurrent {{ ");
        defmt::write!(f, "charge_current: {=u16}, ", &self.charge_current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeCurrent {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeCurrent {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeCurrent {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeCurrent {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeCurrent {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeCurrent {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeCurrent {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGE_OPTION_0")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargeOption0 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargeOption0 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargeOption0 {
    /// `bit 0` - Read the `chrg_inhibit` field.
    ///
    /// Charge Inhibit. When bit is 0 battery charging will start with valid values in the CHARGE_VOLTAGE() and CHARGE_CURRENT().
    #[doc(alias = "CHRG_INHIBIT")]
    #[must_use]
    pub fn chrg_inhibit(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `en_iin_dpm` field.
    ///
    /// IIN_DPM Enable. Host writes this bit to enable IIN_DPM regulation loop. When the IIN_DPM is disabled by the charger (refer to IIN_DPM_AUTO_DISABLE), this bit goes LOW. Under OTG mode, this bit is also used to enable/disable IOTG regulation.
    #[doc(alias = "EN_IIN_DPM")]
    #[must_use]
    pub fn en_iin_dpm(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `en_ldo` field.
    ///
    /// LDO Mode Enable. When battery voltage is below VSYS_MIN(), the charger is in pre-charge with LDO mode enabled.
    #[doc(alias = "EN_LDO")]
    #[must_use]
    pub fn en_ldo(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `ibat_gain` field.
    ///
    /// IBAT Amplifier Ratio. The ratio of voltage on IBAT and voltage across SRP and SRN.
    #[doc(alias = "IBAT_GAIN")]
    #[must_use]
    pub fn ibat_gain(&self) -> IbatGain {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 4` - Read the `iadpt_gain` field.
    ///
    /// IADPT Amplifier Ratio. The ratio of voltage on IADPT and voltage across ACP and ACN.
    #[doc(alias = "IADPT_GAIN")]
    #[must_use]
    pub fn iadpt_gain(&self) -> IadptGain {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 5` - Read the `en_learn` field.
    ///
    /// LEARN mode function enable.
    #[doc(alias = "EN_LEARN")]
    #[must_use]
    pub fn en_learn(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `vsys_uvp_enz` field.
    ///
    /// Disable system under voltage protection.
    #[doc(alias = "VSYS_UVP_ENZ")]
    #[must_use]
    pub fn vsys_uvp_enz(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `en_cmp_latch` field.
    ///
    /// Enable Latch of Independent Comparator. Comparator output with effective low. If enabled in PROCHOT profile PP_CMP=1b, STAT_COMP bit keep 1b after triggered until read by host and clear. host can clear CMPOUT pin by toggling this EN_CMP_LATCH bit.
    #[doc(alias = "EN_CMP_LATCH")]
    #[must_use]
    pub fn en_cmp_latch(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `en_batovp` field.
    ///
    /// Enable BATOVP protection.
    #[doc(alias = "EN_BATOVP")]
    #[must_use]
    pub fn en_batovp(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `pwm_freq` field.
    ///
    /// Switching Frequency Selection.
    #[doc(alias = "PWM_FREQ")]
    #[must_use]
    pub fn pwm_freq(&self) -> SwitchingFreq {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 10` - Read the `en_ooa` field.
    ///
    /// Out-of-Audio Enable.
    #[doc(alias = "EN_OOA")]
    #[must_use]
    pub fn en_ooa(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `otg_on_chrgok` field.
    ///
    /// Add OTG to CHRG_OK.
    #[doc(alias = "OTG_ON_CHRGOK")]
    #[must_use]
    pub fn otg_on_chrgok(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `iin_dpm_auto_disable` field.
    ///
    /// IIN_DPM Auto Disable.
    #[doc(alias = "IIN_DPM_AUTO_DISABLE")]
    #[must_use]
    pub fn iin_dpm_auto_disable(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `14:13` - Read the `wdtmr_adj` field.
    ///
    /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command.
    #[doc(alias = "WDTMR_ADJ")]
    #[must_use]
    pub fn wdtmr_adj(&self) -> MaxDelay {
        let start = 13;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 15` - Read the `en_lwpwr` field.
    ///
    /// Low Power Mode enable.
    #[doc(alias = "EN_LWPWR")]
    #[must_use]
    pub fn en_lwpwr(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `chrg_inhibit` field.
    ///
    /// Charge Inhibit. When bit is 0 battery charging will start with valid values in the CHARGE_VOLTAGE() and CHARGE_CURRENT().
    #[doc(alias = "CHRG_INHIBIT")]
    pub fn set_chrg_inhibit(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `en_iin_dpm` field.
    ///
    /// IIN_DPM Enable. Host writes this bit to enable IIN_DPM regulation loop. When the IIN_DPM is disabled by the charger (refer to IIN_DPM_AUTO_DISABLE), this bit goes LOW. Under OTG mode, this bit is also used to enable/disable IOTG regulation.
    #[doc(alias = "EN_IIN_DPM")]
    pub fn set_en_iin_dpm(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `en_ldo` field.
    ///
    /// LDO Mode Enable. When battery voltage is below VSYS_MIN(), the charger is in pre-charge with LDO mode enabled.
    #[doc(alias = "EN_LDO")]
    pub fn set_en_ldo(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `ibat_gain` field.
    ///
    /// IBAT Amplifier Ratio. The ratio of voltage on IBAT and voltage across SRP and SRN.
    #[doc(alias = "IBAT_GAIN")]
    pub fn set_ibat_gain(&mut self, value: IbatGain) {
        let start = 3;
        let end = 3;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `iadpt_gain` field.
    ///
    /// IADPT Amplifier Ratio. The ratio of voltage on IADPT and voltage across ACP and ACN.
    #[doc(alias = "IADPT_GAIN")]
    pub fn set_iadpt_gain(&mut self, value: IadptGain) {
        let start = 4;
        let end = 4;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `en_learn` field.
    ///
    /// LEARN mode function enable.
    #[doc(alias = "EN_LEARN")]
    pub fn set_en_learn(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `vsys_uvp_enz` field.
    ///
    /// Disable system under voltage protection.
    #[doc(alias = "VSYS_UVP_ENZ")]
    pub fn set_vsys_uvp_enz(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `en_cmp_latch` field.
    ///
    /// Enable Latch of Independent Comparator. Comparator output with effective low. If enabled in PROCHOT profile PP_CMP=1b, STAT_COMP bit keep 1b after triggered until read by host and clear. host can clear CMPOUT pin by toggling this EN_CMP_LATCH bit.
    #[doc(alias = "EN_CMP_LATCH")]
    pub fn set_en_cmp_latch(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `en_batovp` field.
    ///
    /// Enable BATOVP protection.
    #[doc(alias = "EN_BATOVP")]
    pub fn set_en_batovp(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `pwm_freq` field.
    ///
    /// Switching Frequency Selection.
    #[doc(alias = "PWM_FREQ")]
    pub fn set_pwm_freq(&mut self, value: SwitchingFreq) {
        let start = 9;
        let end = 9;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `en_ooa` field.
    ///
    /// Out-of-Audio Enable.
    #[doc(alias = "EN_OOA")]
    pub fn set_en_ooa(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `otg_on_chrgok` field.
    ///
    /// Add OTG to CHRG_OK.
    #[doc(alias = "OTG_ON_CHRGOK")]
    pub fn set_otg_on_chrgok(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `iin_dpm_auto_disable` field.
    ///
    /// IIN_DPM Auto Disable.
    #[doc(alias = "IIN_DPM_AUTO_DISABLE")]
    pub fn set_iin_dpm_auto_disable(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `14:13` - Set the `wdtmr_adj` field.
    ///
    /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command.
    #[doc(alias = "WDTMR_ADJ")]
    pub fn set_wdtmr_adj(&mut self, value: MaxDelay) {
        let start = 13;
        let end = 14;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `en_lwpwr` field.
    ///
    /// Low Power Mode enable.
    #[doc(alias = "EN_LWPWR")]
    pub fn set_en_lwpwr(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ChargeOption0 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargeOption0 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargeOption0> for [u8; 2] {
    fn from(val: ChargeOption0) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargeOption0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargeOption0");
        d.field("chrg_inhibit", &self.chrg_inhibit());
        d.field("en_iin_dpm", &self.en_iin_dpm());
        d.field("en_ldo", &self.en_ldo());
        d.field("ibat_gain", &self.ibat_gain());
        d.field("iadpt_gain", &self.iadpt_gain());
        d.field("en_learn", &self.en_learn());
        d.field("vsys_uvp_enz", &self.vsys_uvp_enz());
        d.field("en_cmp_latch", &self.en_cmp_latch());
        d.field("en_batovp", &self.en_batovp());
        d.field("pwm_freq", &self.pwm_freq());
        d.field("en_ooa", &self.en_ooa());
        d.field("otg_on_chrgok", &self.otg_on_chrgok());
        d.field("iin_dpm_auto_disable", &self.iin_dpm_auto_disable());
        d.field("wdtmr_adj", &self.wdtmr_adj());
        d.field("en_lwpwr", &self.en_lwpwr());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargeOption0 {{ ");
        defmt::write!(f, "chrg_inhibit: {=bool}, ", &self.chrg_inhibit());
        defmt::write!(f, "en_iin_dpm: {=bool}, ", &self.en_iin_dpm());
        defmt::write!(f, "en_ldo: {=bool}, ", &self.en_ldo());
        defmt::write!(f, "ibat_gain: {}, ", &self.ibat_gain());
        defmt::write!(f, "iadpt_gain: {}, ", &self.iadpt_gain());
        defmt::write!(f, "en_learn: {=bool}, ", &self.en_learn());
        defmt::write!(f, "vsys_uvp_enz: {=bool}, ", &self.vsys_uvp_enz());
        defmt::write!(f, "en_cmp_latch: {=bool}, ", &self.en_cmp_latch());
        defmt::write!(f, "en_batovp: {=bool}, ", &self.en_batovp());
        defmt::write!(f, "pwm_freq: {}, ", &self.pwm_freq());
        defmt::write!(f, "en_ooa: {=bool}, ", &self.en_ooa());
        defmt::write!(f, "otg_on_chrgok: {=bool}, ", &self.otg_on_chrgok());
        defmt::write!(f, "iin_dpm_auto_disable: {=bool}, ", &self.iin_dpm_auto_disable());
        defmt::write!(f, "wdtmr_adj: {}, ", &self.wdtmr_adj());
        defmt::write!(f, "en_lwpwr: {=bool}, ", &self.en_lwpwr());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargeOption0 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargeOption0 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargeOption0 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargeOption0 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargeOption0 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargeOption0 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargeOption0 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtmrAdj {
    Disable = 0,
    FiveSecs = 1,
    EightyEightSecs = 2,
    OneHundredSeventyFiveSecs = 3,
}
impl core::convert::TryFrom<u8> for WdtmrAdj {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disable),
            1 => Ok(Self::FiveSecs),
            2 => Ok(Self::EightyEightSecs),
            3 => Ok(Self::OneHundredSeventyFiveSecs),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "WdtmrAdj",
            }),
        }
    }
}
impl From<WdtmrAdj> for u8 {
    fn from(val: WdtmrAdj) -> Self {
        match val {
            WdtmrAdj::Disable => 0,
            WdtmrAdj::FiveSecs => 1,
            WdtmrAdj::EightyEightSecs => 2,
            WdtmrAdj::OneHundredSeventyFiveSecs => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for WdtmrAdj {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DitherConfig {
    Disable = 0,
    OneX = 1,
    TwoX = 2,
    ThreeX = 3,
}
impl core::convert::TryFrom<u8> for DitherConfig {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disable),
            1 => Ok(Self::OneX),
            2 => Ok(Self::TwoX),
            3 => Ok(Self::ThreeX),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "DitherConfig",
            }),
        }
    }
}
impl From<DitherConfig> for u8 {
    fn from(val: DitherConfig) -> Self {
        match val {
            DitherConfig::Disable => 0,
            DitherConfig::OneX => 1,
            DitherConfig::TwoX => 2,
            DitherConfig::ThreeX => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for DitherConfig {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdchgDeglitchTime2 {
    VeryShort = 0,
    Short = 1,
    Long = 2,
    VeryLong = 3,
}
impl core::convert::TryFrom<u8> for IdchgDeglitchTime2 {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::VeryShort),
            1 => Ok(Self::Short),
            2 => Ok(Self::Long),
            3 => Ok(Self::VeryLong),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IdchgDeglitchTime2",
            }),
        }
    }
}
impl From<IdchgDeglitchTime2> for u8 {
    fn from(val: IdchgDeglitchTime2) -> Self {
        match val {
            IdchgDeglitchTime2::VeryShort => 0,
            IdchgDeglitchTime2::Short => 1,
            IdchgDeglitchTime2::Long => 2,
            IdchgDeglitchTime2::VeryLong => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for IdchgDeglitchTime2 {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcRateSelect {
    Continuous = 0,
    OneShot = 1,
}
impl core::convert::TryFrom<u8> for AdcRateSelect {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Continuous),
            1 => Ok(Self::OneShot),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AdcRateSelect",
            }),
        }
    }
}
impl From<AdcRateSelect> for u8 {
    fn from(val: AdcRateSelect) -> Self {
        match val {
            AdcRateSelect::Continuous => 0,
            AdcRateSelect::OneShot => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AdcRateSelect {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcResolution {
    FifteenBits = 0,
    FourteenBits = 1,
    ThirteenBits = 2,
    Reserved = 3,
}
impl core::convert::TryFrom<u8> for AdcResolution {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::FifteenBits),
            1 => Ok(Self::FourteenBits),
            2 => Ok(Self::ThirteenBits),
            3 => Ok(Self::Reserved),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AdcResolution",
            }),
        }
    }
}
impl From<AdcResolution> for u8 {
    fn from(val: AdcResolution) -> Self {
        match val {
            AdcResolution::FifteenBits => 0,
            AdcResolution::FourteenBits => 1,
            AdcResolution::ThirteenBits => 2,
            AdcResolution::Reserved => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AdcResolution {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcAvgCtrl {
    SingleValue = 0,
    RunningAvg = 1,
}
impl core::convert::TryFrom<u8> for AdcAvgCtrl {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::SingleValue),
            1 => Ok(Self::RunningAvg),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AdcAvgCtrl",
            }),
        }
    }
}
impl From<AdcAvgCtrl> for u8 {
    fn from(val: AdcAvgCtrl) -> Self {
        match val {
            AdcAvgCtrl::SingleValue => 0,
            AdcAvgCtrl::RunningAvg => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AdcAvgCtrl {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcAvgInit {
    ExistingRegValue = 0,
    NewAdcValue = 1,
}
impl core::convert::TryFrom<u8> for AdcAvgInit {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ExistingRegValue),
            1 => Ok(Self::NewAdcValue),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AdcAvgInit",
            }),
        }
    }
}
impl From<AdcAvgInit> for u8 {
    fn from(val: AdcAvgInit) -> Self {
        match val {
            AdcAvgInit::ExistingRegValue => 0,
            AdcAvgInit::NewAdcValue => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AdcAvgInit {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdchgDeglitchTime {
    VeryShort = 0,
    Short = 1,
    Long = 2,
    VeryLong = 3,
}
impl core::convert::TryFrom<u8> for IdchgDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::VeryShort),
            1 => Ok(Self::Short),
            2 => Ok(Self::Long),
            3 => Ok(Self::VeryLong),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IdchgDeglitchTime",
            }),
        }
    }
}
impl From<IdchgDeglitchTime> for u8 {
    fn from(val: IdchgDeglitchTime) -> Self {
        match val {
            IdchgDeglitchTime::VeryShort => 0,
            IdchgDeglitchTime::Short => 1,
            IdchgDeglitchTime::Long => 2,
            IdchgDeglitchTime::VeryLong => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for IdchgDeglitchTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "ICRITDeglitchTime")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcritDeglitchTime {
    VeryShort = 0,
    Short = 1,
    Long = 2,
    VeryLong = 3,
}
impl core::convert::TryFrom<u8> for IcritDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::VeryShort),
            1 => Ok(Self::Short),
            2 => Ok(Self::Long),
            3 => Ok(Self::VeryLong),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IcritDeglitchTime",
            }),
        }
    }
}
impl From<IcritDeglitchTime> for u8 {
    fn from(val: IcritDeglitchTime) -> Self {
        match val {
            IcritDeglitchTime::VeryShort => 0,
            IcritDeglitchTime::Short => 1,
            IcritDeglitchTime::Long => 2,
            IcritDeglitchTime::VeryLong => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for IcritDeglitchTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Threshold {
    EightyThreePercent = 0,
    NinetyOnePercent = 1,
}
impl core::convert::TryFrom<u8> for Threshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightyThreePercent),
            1 => Ok(Self::NinetyOnePercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Threshold",
            }),
        }
    }
}
impl From<Threshold> for u8 {
    fn from(val: Threshold) -> Self {
        match val {
            Threshold::EightyThreePercent => 0,
            Threshold::NinetyOnePercent => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Threshold {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InomDeglitchTime {
    Short = 0,
    Long = 1,
}
impl core::convert::TryFrom<u8> for InomDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Short),
            1 => Ok(Self::Long),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "InomDeglitchTime",
            }),
        }
    }
}
impl From<InomDeglitchTime> for u8 {
    fn from(val: InomDeglitchTime) -> Self {
        match val {
            InomDeglitchTime::Short => 0,
            InomDeglitchTime::Long => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for InomDeglitchTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VsysMinSoftSlewRate {
    Disable = 0,
    Fast = 1,
    Slow = 2,
    VerySlow = 3,
}
impl core::convert::TryFrom<u8> for VsysMinSoftSlewRate {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disable),
            1 => Ok(Self::Fast),
            2 => Ok(Self::Slow),
            3 => Ok(Self::VerySlow),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "VsysMinSoftSlewRate",
            }),
        }
    }
}
impl From<VsysMinSoftSlewRate> for u8 {
    fn from(val: VsysMinSoftSlewRate) -> Self {
        match val {
            VsysMinSoftSlewRate::Disable => 0,
            VsysMinSoftSlewRate::Fast => 1,
            VsysMinSoftSlewRate::Slow => 2,
            VsysMinSoftSlewRate::VerySlow => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for VsysMinSoftSlewRate {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnOtgPinSelect {
    VapMode = 0,
    OtgMode = 1,
}
impl core::convert::TryFrom<u8> for EnOtgPinSelect {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::VapMode),
            1 => Ok(Self::OtgMode),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "EnOtgPinSelect",
            }),
        }
    }
}
impl From<EnOtgPinSelect> for u8 {
    fn from(val: EnOtgPinSelect) -> Self {
        match val {
            EnOtgPinSelect::VapMode => 0,
            EnOtgPinSelect::OtgMode => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for EnOtgPinSelect {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "ILAvgClamp")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IlAvgClamp {
    TenAmps = 0,
    EighteenAmps = 1,
    TwentyFourAmps = 2,
    Disable = 3,
}
impl core::convert::TryFrom<u8> for IlAvgClamp {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TenAmps),
            1 => Ok(Self::EighteenAmps),
            2 => Ok(Self::TwentyFourAmps),
            3 => Ok(Self::Disable),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IlAvgClamp",
            }),
        }
    }
}
impl From<IlAvgClamp> for u8 {
    fn from(val: IlAvgClamp) -> Self {
        match val {
            IlAvgClamp::TenAmps => 0,
            IlAvgClamp::EighteenAmps => 1,
            IlAvgClamp::TwentyFourAmps => 2,
            IlAvgClamp::Disable => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for IlAvgClamp {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsysOtg {
    BattDischargePowerMinusOtg = 0,
    BattDischargePower = 1,
}
impl core::convert::TryFrom<u8> for PsysOtg {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::BattDischargePowerMinusOtg),
            1 => Ok(Self::BattDischargePower),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PsysOtg",
            }),
        }
    }
}
impl From<PsysOtg> for u8 {
    fn from(val: PsysOtg) -> Self {
        match val {
            PsysOtg::BattDischargePowerMinusOtg => 0,
            PsysOtg::BattDischargePower => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PsysOtg {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkpwrTovldDeg {
    OneMillisecond = 0,
    TwoMilliseconds = 1,
    FiveMilliseconds = 2,
    TenMilliseconds = 3,
}
impl core::convert::TryFrom<u8> for PkpwrTovldDeg {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneMillisecond),
            1 => Ok(Self::TwoMilliseconds),
            2 => Ok(Self::FiveMilliseconds),
            3 => Ok(Self::TenMilliseconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PkpwrTovldDeg",
            }),
        }
    }
}
impl From<PkpwrTovldDeg> for u8 {
    fn from(val: PkpwrTovldDeg) -> Self {
        match val {
            PkpwrTovldDeg::OneMillisecond => 0,
            PkpwrTovldDeg::TwoMilliseconds => 1,
            PkpwrTovldDeg::FiveMilliseconds => 2,
            PkpwrTovldDeg::TenMilliseconds => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PkpwrTovldDeg {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkpwrTmax {
    TwentyMilliseconds = 0,
    FortyMilliseconds = 1,
    EightyMilliseconds = 2,
    OneSecond = 3,
}
impl core::convert::TryFrom<u8> for PkpwrTmax {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwentyMilliseconds),
            1 => Ok(Self::FortyMilliseconds),
            2 => Ok(Self::EightyMilliseconds),
            3 => Ok(Self::OneSecond),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PkpwrTmax",
            }),
        }
    }
}
impl From<PkpwrTmax> for u8 {
    fn from(val: PkpwrTmax) -> Self {
        match val {
            PkpwrTmax::TwentyMilliseconds => 0,
            PkpwrTmax::FortyMilliseconds => 1,
            PkpwrTmax::EightyMilliseconds => 2,
            PkpwrTmax::OneSecond => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PkpwrTmax {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IBatPinSelect {
    IBatPinAsDischarge = 0,
    IBatPinAsCharge = 1,
}
impl core::convert::TryFrom<u8> for IBatPinSelect {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::IBatPinAsDischarge),
            1 => Ok(Self::IBatPinAsCharge),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IBatPinSelect",
            }),
        }
    }
}
impl From<IBatPinSelect> for u8 {
    fn from(val: IBatPinSelect) -> Self {
        match val {
            IBatPinSelect::IBatPinAsDischarge => 0,
            IBatPinSelect::IBatPinAsCharge => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for IBatPinSelect {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverCurrentThresholdQ4Vds {
    OneHundredFiftyMillivolts = 0,
    TwoHundredSixtyMillivolts = 1,
}
impl core::convert::TryFrom<u8> for OverCurrentThresholdQ4Vds {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHundredFiftyMillivolts),
            1 => Ok(Self::TwoHundredSixtyMillivolts),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "OverCurrentThresholdQ4Vds",
            }),
        }
    }
}
impl From<OverCurrentThresholdQ4Vds> for u8 {
    fn from(val: OverCurrentThresholdQ4Vds) -> Self {
        match val {
            OverCurrentThresholdQ4Vds::OneHundredFiftyMillivolts => 0,
            OverCurrentThresholdQ4Vds::TwoHundredSixtyMillivolts => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for OverCurrentThresholdQ4Vds {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverCurrentThresholdRac {
    ThreeHundredMillivolts = 0,
    FourHundredFiftyMillivolts = 1,
}
impl core::convert::TryFrom<u8> for OverCurrentThresholdRac {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ThreeHundredMillivolts),
            1 => Ok(Self::FourHundredFiftyMillivolts),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "OverCurrentThresholdRac",
            }),
        }
    }
}
impl From<OverCurrentThresholdRac> for u8 {
    fn from(val: OverCurrentThresholdRac) -> Self {
        match val {
            OverCurrentThresholdRac::ThreeHundredMillivolts => 0,
            OverCurrentThresholdRac::FourHundredFiftyMillivolts => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for OverCurrentThresholdRac {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AcocLimit {
    OnePoint33Percent = 0,
    TwoPercent = 1,
}
impl core::convert::TryFrom<u8> for AcocLimit {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OnePoint33Percent),
            1 => Ok(Self::TwoPercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AcocLimit",
            }),
        }
    }
}
impl From<AcocLimit> for u8 {
    fn from(val: AcocLimit) -> Self {
        match val {
            AcocLimit::OnePoint33Percent => 0,
            AcocLimit::TwoPercent => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AcocLimit {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BatdocVth {
    TwoPercent = 0,
    ThreePercent = 1,
}
impl core::convert::TryFrom<u8> for BatdocVth {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwoPercent),
            1 => Ok(Self::ThreePercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "BatdocVth",
            }),
        }
    }
}
impl From<BatdocVth> for u8 {
    fn from(val: BatdocVth) -> Self {
        match val {
            BatdocVth::TwoPercent => 0,
            BatdocVth::ThreePercent => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for BatdocVth {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsysEnable {
    PbusAndPbat = 0,
    Pbus = 1,
    Reserved = 2,
    Off = 3,
}
impl core::convert::TryFrom<u8> for PsysEnable {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::PbusAndPbat),
            1 => Ok(Self::Pbus),
            2 => Ok(Self::Reserved),
            3 => Ok(Self::Off),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PsysEnable",
            }),
        }
    }
}
impl From<PsysEnable> for u8 {
    fn from(val: PsysEnable) -> Self {
        match val {
            PsysEnable::PbusAndPbat => 0,
            PsysEnable::Pbus => 1,
            PsysEnable::Reserved => 2,
            PsysEnable::Off => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PsysEnable {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "InputSenseResistorRAC")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputSenseResistorRac {
    TenMilliOhms = 0,
    FiveMilliOhms = 1,
}
impl core::convert::TryFrom<u8> for InputSenseResistorRac {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TenMilliOhms),
            1 => Ok(Self::FiveMilliOhms),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "InputSenseResistorRac",
            }),
        }
    }
}
impl From<InputSenseResistorRac> for u8 {
    fn from(val: InputSenseResistorRac) -> Self {
        match val {
            InputSenseResistorRac::TenMilliOhms => 0,
            InputSenseResistorRac::FiveMilliOhms => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for InputSenseResistorRac {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeSenseResistorRsr {
    FiveMilliOhms = 0,
    TwoMilliOhms = 1,
}
impl core::convert::TryFrom<u8> for ChargeSenseResistorRsr {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::FiveMilliOhms),
            1 => Ok(Self::TwoMilliOhms),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ChargeSenseResistorRsr",
            }),
        }
    }
}
impl From<ChargeSenseResistorRsr> for u8 {
    fn from(val: ChargeSenseResistorRsr) -> Self {
        match val {
            ChargeSenseResistorRsr::FiveMilliOhms => 0,
            ChargeSenseResistorRsr::TwoMilliOhms => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ChargeSenseResistorRsr {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsysGain {
    #[doc(alias = "ZeroPoint25uAperW")]
    ZeroPoint25UAperW = 0,
    #[doc(alias = "OnePoint00uAperW")]
    OnePoint00UAperW = 1,
}
impl core::convert::TryFrom<u8> for PsysGain {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ZeroPoint25UAperW),
            1 => Ok(Self::OnePoint00UAperW),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PsysGain",
            }),
        }
    }
}
impl From<PsysGain> for u8 {
    fn from(val: PsysGain) -> Self {
        match val {
            PsysGain::ZeroPoint25UAperW => 0,
            PsysGain::OnePoint00UAperW => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PsysGain {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ComparatorDeglitchTime {
    VeryShort = 0,
    Short = 1,
    Long = 2,
    VeryLong = 3,
}
impl core::convert::TryFrom<u8> for ComparatorDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::VeryShort),
            1 => Ok(Self::Short),
            2 => Ok(Self::Long),
            3 => Ok(Self::VeryLong),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ComparatorDeglitchTime",
            }),
        }
    }
}
impl From<ComparatorDeglitchTime> for u8 {
    fn from(val: ComparatorDeglitchTime) -> Self {
        match val {
            ComparatorDeglitchTime::VeryShort => 0,
            ComparatorDeglitchTime::Short => 1,
            ComparatorDeglitchTime::Long => 2,
            ComparatorDeglitchTime::VeryLong => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ComparatorDeglitchTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProchotPulseWidth {
    OneHundredMilliseconds = 0,
    FiftyMilliseconds = 1,
    SixMilliseconds = 2,
    TwelveMilliseconds = 3,
}
impl core::convert::TryFrom<u8> for ProchotPulseWidth {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHundredMilliseconds),
            1 => Ok(Self::FiftyMilliseconds),
            2 => Ok(Self::SixMilliseconds),
            3 => Ok(Self::TwelveMilliseconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ProchotPulseWidth",
            }),
        }
    }
}
impl From<ProchotPulseWidth> for u8 {
    fn from(val: ProchotPulseWidth) -> Self {
        match val {
            ProchotPulseWidth::OneHundredMilliseconds => 0,
            ProchotPulseWidth::FiftyMilliseconds => 1,
            ProchotPulseWidth::SixMilliseconds => 2,
            ProchotPulseWidth::TwelveMilliseconds => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ProchotPulseWidth {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProchotClear {
    Clear = 0,
    Idle = 1,
}
impl core::convert::TryFrom<u8> for ProchotClear {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Clear),
            1 => Ok(Self::Idle),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ProchotClear",
            }),
        }
    }
}
impl From<ProchotClear> for u8 {
    fn from(val: ProchotClear) -> Self {
        match val {
            ProchotClear::Clear => 0,
            ProchotClear::Idle => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ProchotClear {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputSrcStat {
    NotPresent = 0,
    Present = 1,
}
impl core::convert::TryFrom<u8> for InputSrcStat {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NotPresent),
            1 => Ok(Self::Present),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "InputSrcStat",
            }),
        }
    }
}
impl From<InputSrcStat> for u8 {
    fn from(val: InputSrcStat) -> Self {
        match val {
            InputSrcStat::NotPresent => 0,
            InputSrcStat::Present => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for InputSrcStat {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChrgCycleStat {
    NotChrging = 0,
    Trickle = 1,
    PreChrg = 2,
    #[doc(alias = "FastChrgCC")]
    FastChrgCc = 3,
    #[doc(alias = "FastChrgCV")]
    FastChrgCv = 4,
    Reserved1 = 5,
    Reserved2 = 6,
    ChrgTerminationDone = 7,
}
impl core::convert::TryFrom<u8> for ChrgCycleStat {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NotChrging),
            1 => Ok(Self::Trickle),
            2 => Ok(Self::PreChrg),
            3 => Ok(Self::FastChrgCc),
            4 => Ok(Self::FastChrgCv),
            5 => Ok(Self::Reserved1),
            6 => Ok(Self::Reserved2),
            7 => Ok(Self::ChrgTerminationDone),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ChrgCycleStat",
            }),
        }
    }
}
impl From<ChrgCycleStat> for u8 {
    fn from(val: ChrgCycleStat) -> Self {
        match val {
            ChrgCycleStat::NotChrging => 0,
            ChrgCycleStat::Trickle => 1,
            ChrgCycleStat::PreChrg => 2,
            ChrgCycleStat::FastChrgCc => 3,
            ChrgCycleStat::FastChrgCv => 4,
            ChrgCycleStat::Reserved1 => 5,
            ChrgCycleStat::Reserved2 => 6,
            ChrgCycleStat::ChrgTerminationDone => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ChrgCycleStat {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChrgSafetyTimerStat {
    Normal = 0,
    Expired = 1,
}
impl core::convert::TryFrom<u8> for ChrgSafetyTimerStat {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Expired),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ChrgSafetyTimerStat",
            }),
        }
    }
}
impl From<ChrgSafetyTimerStat> for u8 {
    fn from(val: ChrgSafetyTimerStat) -> Self {
        match val {
            ChrgSafetyTimerStat::Normal => 0,
            ChrgSafetyTimerStat::Expired => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ChrgSafetyTimerStat {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TempRegulationStat {
    NotTempRegulated = 0,
    TempRegulated = 1,
}
impl core::convert::TryFrom<u8> for TempRegulationStat {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NotTempRegulated),
            1 => Ok(Self::TempRegulated),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "TempRegulationStat",
            }),
        }
    }
}
impl From<TempRegulationStat> for u8 {
    fn from(val: TempRegulationStat) -> Self {
        match val {
            TempRegulationStat::NotTempRegulated => 0,
            TempRegulationStat::TempRegulated => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for TempRegulationStat {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModePinProgStatus {
    #[doc(alias = "DualPhaseNormalComp600kHz")]
    DualPhaseNormalComp600KHz = 0,
    #[doc(alias = "DualPhaseNormalComp800kHz")]
    DualPhaseNormalComp800KHz = 1,
    #[doc(alias = "DualPhaseSlowComp600kHz")]
    DualPhaseSlowComp600KHz = 2,
    #[doc(alias = "DualPhaseSlowComp800kHz")]
    DualPhaseSlowComp800KHz = 3,
    #[doc(alias = "SinglePhaseNormalComp600kHz")]
    SinglePhaseNormalComp600KHz = 4,
    #[doc(alias = "SinglePhaseNormalComp800kHz")]
    SinglePhaseNormalComp800KHz = 5,
    #[doc(alias = "SinglePhaseSlowComp600kHz")]
    SinglePhaseSlowComp600KHz = 6,
    #[doc(alias = "SinglePhaseSlowComp800kHz")]
    SinglePhaseSlowComp800KHz = 7,
}
impl core::convert::TryFrom<u8> for ModePinProgStatus {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::DualPhaseNormalComp600KHz),
            1 => Ok(Self::DualPhaseNormalComp800KHz),
            2 => Ok(Self::DualPhaseSlowComp600KHz),
            3 => Ok(Self::DualPhaseSlowComp800KHz),
            4 => Ok(Self::SinglePhaseNormalComp600KHz),
            5 => Ok(Self::SinglePhaseNormalComp800KHz),
            6 => Ok(Self::SinglePhaseSlowComp600KHz),
            7 => Ok(Self::SinglePhaseSlowComp800KHz),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ModePinProgStatus",
            }),
        }
    }
}
impl From<ModePinProgStatus> for u8 {
    fn from(val: ModePinProgStatus) -> Self {
        match val {
            ModePinProgStatus::DualPhaseNormalComp600KHz => 0,
            ModePinProgStatus::DualPhaseNormalComp800KHz => 1,
            ModePinProgStatus::DualPhaseSlowComp600KHz => 2,
            ModePinProgStatus::DualPhaseSlowComp800KHz => 3,
            ModePinProgStatus::SinglePhaseNormalComp600KHz => 4,
            ModePinProgStatus::SinglePhaseNormalComp800KHz => 5,
            ModePinProgStatus::SinglePhaseSlowComp600KHz => 6,
            ModePinProgStatus::SinglePhaseSlowComp800KHz => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ModePinProgStatus {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChgTmrCtrl {
    FiveHrs = 0,
    EightHrs = 1,
    TwelveHrs = 2,
    TwentyFourHrs = 3,
}
impl core::convert::TryFrom<u8> for ChgTmrCtrl {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::FiveHrs),
            1 => Ok(Self::EightHrs),
            2 => Ok(Self::TwelveHrs),
            3 => Ok(Self::TwentyFourHrs),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ChgTmrCtrl",
            }),
        }
    }
}
impl From<ChgTmrCtrl> for u8 {
    fn from(val: ChgTmrCtrl) -> Self {
        match val {
            ChgTmrCtrl::FiveHrs => 0,
            ChgTmrCtrl::EightHrs => 1,
            ChgTmrCtrl::TwelveHrs => 2,
            ChgTmrCtrl::TwentyFourHrs => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ChgTmrCtrl {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChgTmrSpeedCtrl {
    Normal = 0,
    HalfSpeed = 1,
}
impl core::convert::TryFrom<u8> for ChgTmrSpeedCtrl {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Normal),
            1 => Ok(Self::HalfSpeed),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ChgTmrSpeedCtrl",
            }),
        }
    }
}
impl From<ChgTmrSpeedCtrl> for u8 {
    fn from(val: ChgTmrSpeedCtrl) -> Self {
        match val {
            ChgTmrSpeedCtrl::Normal => 0,
            ChgTmrSpeedCtrl::HalfSpeed => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ChgTmrSpeedCtrl {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProchotStatusOverheat {
    NotTriggered = 0,
    Triggered = 1,
}
impl core::convert::TryFrom<u8> for ProchotStatusOverheat {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NotTriggered),
            1 => Ok(Self::Triggered),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ProchotStatusOverheat",
            }),
        }
    }
}
impl From<ProchotStatusOverheat> for u8 {
    fn from(val: ProchotStatusOverheat) -> Self {
        match val {
            ProchotStatusOverheat::NotTriggered => 0,
            ProchotStatusOverheat::Triggered => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ProchotStatusOverheat {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThermalDeglitchTime {
    Long = 0,
    Short = 1,
}
impl core::convert::TryFrom<u8> for ThermalDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Long),
            1 => Ok(Self::Short),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ThermalDeglitchTime",
            }),
        }
    }
}
impl From<ThermalDeglitchTime> for u8 {
    fn from(val: ThermalDeglitchTime) -> Self {
        match val {
            ThermalDeglitchTime::Long => 0,
            ThermalDeglitchTime::Short => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ThermalDeglitchTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AcovThreshold {
    TwentyVolts = 0,
    TwentyFiveVolts = 1,
    ThirtyThreeVolts = 2,
    FortyOneVolts = 3,
}
impl core::convert::TryFrom<u8> for AcovThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwentyVolts),
            1 => Ok(Self::TwentyFiveVolts),
            2 => Ok(Self::ThirtyThreeVolts),
            3 => Ok(Self::FortyOneVolts),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AcovThreshold",
            }),
        }
    }
}
impl From<AcovThreshold> for u8 {
    fn from(val: AcovThreshold) -> Self {
        match val {
            AcovThreshold::TwentyVolts => 0,
            AcovThreshold::TwentyFiveVolts => 1,
            AcovThreshold::ThirtyThreeVolts => 2,
            AcovThreshold::FortyOneVolts => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AcovThreshold {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WatchDogReset {
    Normal = 0,
    Reset = 1,
}
impl core::convert::TryFrom<u8> for WatchDogReset {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Reset),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "WatchDogReset",
            }),
        }
    }
}
impl From<WatchDogReset> for u8 {
    fn from(val: WatchDogReset) -> Self {
        match val {
            WatchDogReset::Normal => 0,
            WatchDogReset::Reset => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for WatchDogReset {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpinFuncSelect {
    Cmpmin = 0,
    Treg = 1,
}
impl core::convert::TryFrom<u8> for CmpinFuncSelect {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Cmpmin),
            1 => Ok(Self::Treg),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "CmpinFuncSelect",
            }),
        }
    }
}
impl From<CmpinFuncSelect> for u8 {
    fn from(val: CmpinFuncSelect) -> Self {
        match val {
            CmpinFuncSelect::Cmpmin => 0,
            CmpinFuncSelect::Treg => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for CmpinFuncSelect {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BatcocThreshold {
    Disable = 0,
    FiftyMillivolts = 1,
    SeventyFiveMillivolts = 2,
    OneHundredMillivolts = 3,
}
impl core::convert::TryFrom<u8> for BatcocThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disable),
            1 => Ok(Self::FiftyMillivolts),
            2 => Ok(Self::SeventyFiveMillivolts),
            3 => Ok(Self::OneHundredMillivolts),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "BatcocThreshold",
            }),
        }
    }
}
impl From<BatcocThreshold> for u8 {
    fn from(val: BatcocThreshold) -> Self {
        match val {
            BatcocThreshold::Disable => 0,
            BatcocThreshold::FiftyMillivolts => 1,
            BatcocThreshold::SeventyFiveMillivolts => 2,
            BatcocThreshold::OneHundredMillivolts => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for BatcocThreshold {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleDualTransThreshold {
    ForceDualPhase = 0,
    ThreeAmps = 1,
    FourAmps = 2,
    FiveAmps = 3,
    SixAmps = 4,
    SevenAmps = 5,
    EightAmps = 6,
    NineAmps = 7,
}
impl core::convert::TryFrom<u8> for SingleDualTransThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ForceDualPhase),
            1 => Ok(Self::ThreeAmps),
            2 => Ok(Self::FourAmps),
            3 => Ok(Self::FiveAmps),
            4 => Ok(Self::SixAmps),
            5 => Ok(Self::SevenAmps),
            6 => Ok(Self::EightAmps),
            7 => Ok(Self::NineAmps),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "SingleDualTransThreshold",
            }),
        }
    }
}
impl From<SingleDualTransThreshold> for u8 {
    fn from(val: SingleDualTransThreshold) -> Self {
        match val {
            SingleDualTransThreshold::ForceDualPhase => 0,
            SingleDualTransThreshold::ThreeAmps => 1,
            SingleDualTransThreshold::FourAmps => 2,
            SingleDualTransThreshold::FiveAmps => 3,
            SingleDualTransThreshold::SixAmps => 4,
            SingleDualTransThreshold::SevenAmps => 5,
            SingleDualTransThreshold::EightAmps => 6,
            SingleDualTransThreshold::NineAmps => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for SingleDualTransThreshold {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhaseAddingTransitionDeglitchTime {
    VeryShort = 0,
    Short = 1,
    Long = 2,
    VeryLong = 3,
}
impl core::convert::TryFrom<u8> for PhaseAddingTransitionDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::VeryShort),
            1 => Ok(Self::Short),
            2 => Ok(Self::Long),
            3 => Ok(Self::VeryLong),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PhaseAddingTransitionDeglitchTime",
            }),
        }
    }
}
impl From<PhaseAddingTransitionDeglitchTime> for u8 {
    fn from(val: PhaseAddingTransitionDeglitchTime) -> Self {
        match val {
            PhaseAddingTransitionDeglitchTime::VeryShort => 0,
            PhaseAddingTransitionDeglitchTime::Short => 1,
            PhaseAddingTransitionDeglitchTime::Long => 2,
            PhaseAddingTransitionDeglitchTime::VeryLong => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PhaseAddingTransitionDeglitchTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhaseDroppingTransitionDeglitchTime {
    VeryShort = 0,
    Short = 1,
    Long = 2,
    VeryLong = 3,
}
impl core::convert::TryFrom<u8> for PhaseDroppingTransitionDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::VeryShort),
            1 => Ok(Self::Short),
            2 => Ok(Self::Long),
            3 => Ok(Self::VeryLong),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PhaseDroppingTransitionDeglitchTime",
            }),
        }
    }
}
impl From<PhaseDroppingTransitionDeglitchTime> for u8 {
    fn from(val: PhaseDroppingTransitionDeglitchTime) -> Self {
        match val {
            PhaseDroppingTransitionDeglitchTime::VeryShort => 0,
            PhaseDroppingTransitionDeglitchTime::Short => 1,
            PhaseDroppingTransitionDeglitchTime::Long => 2,
            PhaseDroppingTransitionDeglitchTime::VeryLong => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for PhaseDroppingTransitionDeglitchTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "HIDRV1GateDriveStrengthAdjustment")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hidrv1GateDriveStrengthAdjustment {
    Scale0 = 0,
    Scale1 = 1,
    Scale2 = 2,
    Scale3 = 3,
    Scale4 = 4,
    Scale5 = 5,
    Scale6 = 6,
    Scale7 = 7,
}
impl core::convert::TryFrom<u8> for Hidrv1GateDriveStrengthAdjustment {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Scale0),
            1 => Ok(Self::Scale1),
            2 => Ok(Self::Scale2),
            3 => Ok(Self::Scale3),
            4 => Ok(Self::Scale4),
            5 => Ok(Self::Scale5),
            6 => Ok(Self::Scale6),
            7 => Ok(Self::Scale7),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Hidrv1GateDriveStrengthAdjustment",
            }),
        }
    }
}
impl From<Hidrv1GateDriveStrengthAdjustment> for u8 {
    fn from(val: Hidrv1GateDriveStrengthAdjustment) -> Self {
        match val {
            Hidrv1GateDriveStrengthAdjustment::Scale0 => 0,
            Hidrv1GateDriveStrengthAdjustment::Scale1 => 1,
            Hidrv1GateDriveStrengthAdjustment::Scale2 => 2,
            Hidrv1GateDriveStrengthAdjustment::Scale3 => 3,
            Hidrv1GateDriveStrengthAdjustment::Scale4 => 4,
            Hidrv1GateDriveStrengthAdjustment::Scale5 => 5,
            Hidrv1GateDriveStrengthAdjustment::Scale6 => 6,
            Hidrv1GateDriveStrengthAdjustment::Scale7 => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Hidrv1GateDriveStrengthAdjustment {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "LODRV1GateDriveStrengthAdjustment")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lodrv1GateDriveStrengthAdjustment {
    Scale0 = 0,
    Scale1 = 1,
    Scale2 = 2,
    Scale3 = 3,
    Scale4 = 4,
    Scale5 = 5,
    Scale6 = 6,
    Scale7 = 7,
}
impl core::convert::TryFrom<u8> for Lodrv1GateDriveStrengthAdjustment {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Scale0),
            1 => Ok(Self::Scale1),
            2 => Ok(Self::Scale2),
            3 => Ok(Self::Scale3),
            4 => Ok(Self::Scale4),
            5 => Ok(Self::Scale5),
            6 => Ok(Self::Scale6),
            7 => Ok(Self::Scale7),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Lodrv1GateDriveStrengthAdjustment",
            }),
        }
    }
}
impl From<Lodrv1GateDriveStrengthAdjustment> for u8 {
    fn from(val: Lodrv1GateDriveStrengthAdjustment) -> Self {
        match val {
            Lodrv1GateDriveStrengthAdjustment::Scale0 => 0,
            Lodrv1GateDriveStrengthAdjustment::Scale1 => 1,
            Lodrv1GateDriveStrengthAdjustment::Scale2 => 2,
            Lodrv1GateDriveStrengthAdjustment::Scale3 => 3,
            Lodrv1GateDriveStrengthAdjustment::Scale4 => 4,
            Lodrv1GateDriveStrengthAdjustment::Scale5 => 5,
            Lodrv1GateDriveStrengthAdjustment::Scale6 => 6,
            Lodrv1GateDriveStrengthAdjustment::Scale7 => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Lodrv1GateDriveStrengthAdjustment {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "HIDRV2GateDriveStrengthAdjustment")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hidrv2GateDriveStrengthAdjustment {
    Scale0 = 0,
    Scale1 = 1,
    Scale2 = 2,
    Scale3 = 3,
    Scale4 = 4,
    Scale5 = 5,
    Scale6 = 6,
    Scale7 = 7,
}
impl core::convert::TryFrom<u8> for Hidrv2GateDriveStrengthAdjustment {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Scale0),
            1 => Ok(Self::Scale1),
            2 => Ok(Self::Scale2),
            3 => Ok(Self::Scale3),
            4 => Ok(Self::Scale4),
            5 => Ok(Self::Scale5),
            6 => Ok(Self::Scale6),
            7 => Ok(Self::Scale7),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Hidrv2GateDriveStrengthAdjustment",
            }),
        }
    }
}
impl From<Hidrv2GateDriveStrengthAdjustment> for u8 {
    fn from(val: Hidrv2GateDriveStrengthAdjustment) -> Self {
        match val {
            Hidrv2GateDriveStrengthAdjustment::Scale0 => 0,
            Hidrv2GateDriveStrengthAdjustment::Scale1 => 1,
            Hidrv2GateDriveStrengthAdjustment::Scale2 => 2,
            Hidrv2GateDriveStrengthAdjustment::Scale3 => 3,
            Hidrv2GateDriveStrengthAdjustment::Scale4 => 4,
            Hidrv2GateDriveStrengthAdjustment::Scale5 => 5,
            Hidrv2GateDriveStrengthAdjustment::Scale6 => 6,
            Hidrv2GateDriveStrengthAdjustment::Scale7 => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Hidrv2GateDriveStrengthAdjustment {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "LODRV2GateDriveStrengthAdjustment")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lodrv2GateDriveStrengthAdjustment {
    Scale0 = 0,
    Scale1 = 1,
    Scale2 = 2,
    Scale3 = 3,
    Scale4 = 4,
    Scale5 = 5,
    Scale6 = 6,
    Scale7 = 7,
}
impl core::convert::TryFrom<u8> for Lodrv2GateDriveStrengthAdjustment {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Scale0),
            1 => Ok(Self::Scale1),
            2 => Ok(Self::Scale2),
            3 => Ok(Self::Scale3),
            4 => Ok(Self::Scale4),
            5 => Ok(Self::Scale5),
            6 => Ok(Self::Scale6),
            7 => Ok(Self::Scale7),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Lodrv2GateDriveStrengthAdjustment",
            }),
        }
    }
}
impl From<Lodrv2GateDriveStrengthAdjustment> for u8 {
    fn from(val: Lodrv2GateDriveStrengthAdjustment) -> Self {
        match val {
            Lodrv2GateDriveStrengthAdjustment::Scale0 => 0,
            Lodrv2GateDriveStrengthAdjustment::Scale1 => 1,
            Lodrv2GateDriveStrengthAdjustment::Scale2 => 2,
            Lodrv2GateDriveStrengthAdjustment::Scale3 => 3,
            Lodrv2GateDriveStrengthAdjustment::Scale4 => 4,
            Lodrv2GateDriveStrengthAdjustment::Scale5 => 5,
            Lodrv2GateDriveStrengthAdjustment::Scale6 => 6,
            Lodrv2GateDriveStrengthAdjustment::Scale7 => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Lodrv2GateDriveStrengthAdjustment {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaxDelay {
    Disable = 0,
    FiveSeconds = 1,
    EightyEightSeconds = 2,
    OneHundredSeventyFiveSeconds = 3,
}
impl core::convert::TryFrom<u8> for MaxDelay {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disable),
            1 => Ok(Self::FiveSeconds),
            2 => Ok(Self::EightyEightSeconds),
            3 => Ok(Self::OneHundredSeventyFiveSeconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "MaxDelay",
            }),
        }
    }
}
impl From<MaxDelay> for u8 {
    fn from(val: MaxDelay) -> Self {
        match val {
            MaxDelay::Disable => 0,
            MaxDelay::FiveSeconds => 1,
            MaxDelay::EightyEightSeconds => 2,
            MaxDelay::OneHundredSeventyFiveSeconds => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for MaxDelay {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwitchingFreq {
    EightHundredkHz = 0,
    SixHundredkHz = 1,
}
impl core::convert::TryFrom<u8> for SwitchingFreq {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightHundredkHz),
            1 => Ok(Self::SixHundredkHz),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "SwitchingFreq",
            }),
        }
    }
}
impl From<SwitchingFreq> for u8 {
    fn from(val: SwitchingFreq) -> Self {
        match val {
            SwitchingFreq::EightHundredkHz => 0,
            SwitchingFreq::SixHundredkHz => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for SwitchingFreq {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IadptGain {
    TwentyX = 0,
    FortyX = 1,
}
impl core::convert::TryFrom<u8> for IadptGain {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwentyX),
            1 => Ok(Self::FortyX),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IadptGain",
            }),
        }
    }
}
impl From<IadptGain> for u8 {
    fn from(val: IadptGain) -> Self {
        match val {
            IadptGain::TwentyX => 0,
            IadptGain::FortyX => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for IadptGain {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IbatGain {
    EightX = 0,
    SixtyFourX = 1,
}
impl core::convert::TryFrom<u8> for IbatGain {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightX),
            1 => Ok(Self::SixtyFourX),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IbatGain",
            }),
        }
    }
}
impl From<IbatGain> for u8 {
    fn from(val: IbatGain) -> Self {
        match val {
            IbatGain::EightX => 0,
            IbatGain::SixtyFourX => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for IbatGain {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
