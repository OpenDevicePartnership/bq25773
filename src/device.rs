/// Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u8,
}
impl<I> Device<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self {
            interface,
            base_address: 0,
        }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.charge_option_0_a().read()?;
        callback(0 + 0 * 0, "charge_option_0_a", reg.into());
        let reg = self.charge_option_0_b().read()?;
        callback(1 + 0 * 0, "charge_option_0_b", reg.into());
        let reg = self.charge_current().read()?;
        callback(2 + 0 * 0, "charge_current", reg.into());
        let reg = self.charge_voltage().read()?;
        callback(4 + 0 * 0, "charge_voltage", reg.into());
        let reg = self.iin_host().read()?;
        callback(6 + 0 * 0, "iin_host", reg.into());
        let reg = self.vindpm().read()?;
        callback(8 + 0 * 0, "vindpm", reg.into());
        let reg = self.otg_current().read()?;
        callback(10 + 0 * 0, "otg_current", reg.into());
        let reg = self.otg_voltage().read()?;
        callback(12 + 0 * 0, "otg_voltage", reg.into());
        let reg = self.vsys_min().read()?;
        callback(14 + 0 * 0, "vsys_min", reg.into());
        let reg = self.charge_profile_a().read()?;
        callback(16 + 0 * 0, "charge_profile_a", reg.into());
        let reg = self.charge_profile_b().read()?;
        callback(17 + 0 * 0, "charge_profile_b", reg.into());
        let reg = self.gate_drive_a().read()?;
        callback(18 + 0 * 0, "gate_drive_a", reg.into());
        let reg = self.gate_drive_b().read()?;
        callback(19 + 0 * 0, "gate_drive_b", reg.into());
        let reg = self.charge_option_5_a().read()?;
        callback(20 + 0 * 0, "charge_option_5_a", reg.into());
        let reg = self.charge_option_5_b().read()?;
        callback(21 + 0 * 0, "charge_option_5_b", reg.into());
        let reg = self.auto_charge_a().read()?;
        callback(22 + 0 * 0, "auto_charge_a", reg.into());
        let reg = self.auto_charge_b().read()?;
        callback(23 + 0 * 0, "auto_charge_b", reg.into());
        let reg = self.charger_status_0_a().read()?;
        callback(24 + 0 * 0, "charger_status_0_a", reg.into());
        let reg = self.charger_status_0_b().read()?;
        callback(25 + 0 * 0, "charger_status_0_b", reg.into());
        let reg = self.adc_vbat().read()?;
        callback(26 + 0 * 0, "adc_vbat", reg.into());
        let reg = self.adc_psys().read()?;
        callback(28 + 0 * 0, "adc_psys", reg.into());
        let reg = self.adc_cmpin_tr().read()?;
        callback(30 + 0 * 0, "adc_cmpin_tr", reg.into());
        let reg = self.charger_status_1_a().read()?;
        callback(32 + 0 * 0, "charger_status_1_a", reg.into());
        let reg = self.charger_status_1_b().read()?;
        callback(33 + 0 * 0, "charger_status_1_b", reg.into());
        let reg = self.prochot_status_reg_a().read()?;
        callback(34 + 0 * 0, "prochot_status_reg_a", reg.into());
        let reg = self.prochot_status_reg_b().read()?;
        callback(35 + 0 * 0, "prochot_status_reg_b", reg.into());
        let reg = self.iin_dpm().read()?;
        callback(36 + 0 * 0, "iin_dpm", reg.into());
        let reg = self.adc_vbus().read()?;
        callback(38 + 0 * 0, "adc_vbus", reg.into());
        let reg = self.adc_ibat().read()?;
        callback(40 + 0 * 0, "adc_ibat", reg.into());
        let reg = self.adc_iin().read()?;
        callback(42 + 0 * 0, "adc_iin", reg.into());
        let reg = self.adc_vsys().read()?;
        callback(44 + 0 * 0, "adc_vsys", reg.into());
        let reg = self.manufacture_id().read()?;
        callback(46 + 0 * 0, "manufacture_id", reg.into());
        let reg = self.device_id().read()?;
        callback(47 + 0 * 0, "device_id", reg.into());
        let reg = self.charge_option_1_a().read()?;
        callback(48 + 0 * 0, "charge_option_1_a", reg.into());
        let reg = self.charge_option_1_b().read()?;
        callback(49 + 0 * 0, "charge_option_1_b", reg.into());
        let reg = self.charge_option_2_a().read()?;
        callback(50 + 0 * 0, "charge_option_2_a", reg.into());
        let reg = self.charge_option_2_b().read()?;
        callback(51 + 0 * 0, "charge_option_2_b", reg.into());
        let reg = self.charge_option_3_a().read()?;
        callback(52 + 0 * 0, "charge_option_3_a", reg.into());
        let reg = self.charge_option_3_b().read()?;
        callback(53 + 0 * 0, "charge_option_3_b", reg.into());
        let reg = self.prochot_option_0_a().read()?;
        callback(54 + 0 * 0, "prochot_option_0_a", reg.into());
        let reg = self.prochot_option_0_b().read()?;
        callback(55 + 0 * 0, "prochot_option_0_b", reg.into());
        let reg = self.prochot_option_1_a().read()?;
        callback(56 + 0 * 0, "prochot_option_1_a", reg.into());
        let reg = self.prochot_option_1_b().read()?;
        callback(57 + 0 * 0, "prochot_option_1_b", reg.into());
        let reg = self.adc_option_a().read()?;
        callback(58 + 0 * 0, "adc_option_a", reg.into());
        let reg = self.adc_option_b().read()?;
        callback(59 + 0 * 0, "adc_option_b", reg.into());
        let reg = self.charge_option_4_a().read()?;
        callback(60 + 0 * 0, "charge_option_4_a", reg.into());
        let reg = self.charge_option_4_b().read()?;
        callback(61 + 0 * 0, "charge_option_4_b", reg.into());
        let reg = self.vmin_active_protection_a().read()?;
        callback(62 + 0 * 0, "vmin_active_protection_a", reg.into());
        let reg = self.vmin_active_protection_b().read()?;
        callback(63 + 0 * 0, "vmin_active_protection_b", reg.into());
        let reg = self.autotune_read_b().read()?;
        callback(96 + 0 * 0, "autotune_read_b", reg.into());
        let reg = self.autotune_read_a().read()?;
        callback(97 + 0 * 0, "autotune_read_a", reg.into());
        let reg = self.autotune_force_b().read()?;
        callback(98 + 0 * 0, "autotune_force_b", reg.into());
        let reg = self.autotune_force_a().read()?;
        callback(99 + 0 * 0, "autotune_force_a", reg.into());
        let reg = self.gm_adjust_force_a().read()?;
        callback(100 + 0 * 0, "gm_adjust_force_a", reg.into());
        let reg = self.gm_adjust_force_b().read()?;
        callback(101 + 0 * 0, "gm_adjust_force_b", reg.into());
        let reg = self.virtual_control_a().read()?;
        callback(128 + 0 * 0, "virtual_control_a", reg.into());
        let reg = self.virtual_control_b().read()?;
        callback(129 + 0 * 0, "virtual_control_b", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.charge_option_0_a().read_async().await?;
        callback(0 + 0 * 0, "charge_option_0_a", reg.into());
        let reg = self.charge_option_0_b().read_async().await?;
        callback(1 + 0 * 0, "charge_option_0_b", reg.into());
        let reg = self.charge_current().read_async().await?;
        callback(2 + 0 * 0, "charge_current", reg.into());
        let reg = self.charge_voltage().read_async().await?;
        callback(4 + 0 * 0, "charge_voltage", reg.into());
        let reg = self.iin_host().read_async().await?;
        callback(6 + 0 * 0, "iin_host", reg.into());
        let reg = self.vindpm().read_async().await?;
        callback(8 + 0 * 0, "vindpm", reg.into());
        let reg = self.otg_current().read_async().await?;
        callback(10 + 0 * 0, "otg_current", reg.into());
        let reg = self.otg_voltage().read_async().await?;
        callback(12 + 0 * 0, "otg_voltage", reg.into());
        let reg = self.vsys_min().read_async().await?;
        callback(14 + 0 * 0, "vsys_min", reg.into());
        let reg = self.charge_profile_a().read_async().await?;
        callback(16 + 0 * 0, "charge_profile_a", reg.into());
        let reg = self.charge_profile_b().read_async().await?;
        callback(17 + 0 * 0, "charge_profile_b", reg.into());
        let reg = self.gate_drive_a().read_async().await?;
        callback(18 + 0 * 0, "gate_drive_a", reg.into());
        let reg = self.gate_drive_b().read_async().await?;
        callback(19 + 0 * 0, "gate_drive_b", reg.into());
        let reg = self.charge_option_5_a().read_async().await?;
        callback(20 + 0 * 0, "charge_option_5_a", reg.into());
        let reg = self.charge_option_5_b().read_async().await?;
        callback(21 + 0 * 0, "charge_option_5_b", reg.into());
        let reg = self.auto_charge_a().read_async().await?;
        callback(22 + 0 * 0, "auto_charge_a", reg.into());
        let reg = self.auto_charge_b().read_async().await?;
        callback(23 + 0 * 0, "auto_charge_b", reg.into());
        let reg = self.charger_status_0_a().read_async().await?;
        callback(24 + 0 * 0, "charger_status_0_a", reg.into());
        let reg = self.charger_status_0_b().read_async().await?;
        callback(25 + 0 * 0, "charger_status_0_b", reg.into());
        let reg = self.adc_vbat().read_async().await?;
        callback(26 + 0 * 0, "adc_vbat", reg.into());
        let reg = self.adc_psys().read_async().await?;
        callback(28 + 0 * 0, "adc_psys", reg.into());
        let reg = self.adc_cmpin_tr().read_async().await?;
        callback(30 + 0 * 0, "adc_cmpin_tr", reg.into());
        let reg = self.charger_status_1_a().read_async().await?;
        callback(32 + 0 * 0, "charger_status_1_a", reg.into());
        let reg = self.charger_status_1_b().read_async().await?;
        callback(33 + 0 * 0, "charger_status_1_b", reg.into());
        let reg = self.prochot_status_reg_a().read_async().await?;
        callback(34 + 0 * 0, "prochot_status_reg_a", reg.into());
        let reg = self.prochot_status_reg_b().read_async().await?;
        callback(35 + 0 * 0, "prochot_status_reg_b", reg.into());
        let reg = self.iin_dpm().read_async().await?;
        callback(36 + 0 * 0, "iin_dpm", reg.into());
        let reg = self.adc_vbus().read_async().await?;
        callback(38 + 0 * 0, "adc_vbus", reg.into());
        let reg = self.adc_ibat().read_async().await?;
        callback(40 + 0 * 0, "adc_ibat", reg.into());
        let reg = self.adc_iin().read_async().await?;
        callback(42 + 0 * 0, "adc_iin", reg.into());
        let reg = self.adc_vsys().read_async().await?;
        callback(44 + 0 * 0, "adc_vsys", reg.into());
        let reg = self.manufacture_id().read_async().await?;
        callback(46 + 0 * 0, "manufacture_id", reg.into());
        let reg = self.device_id().read_async().await?;
        callback(47 + 0 * 0, "device_id", reg.into());
        let reg = self.charge_option_1_a().read_async().await?;
        callback(48 + 0 * 0, "charge_option_1_a", reg.into());
        let reg = self.charge_option_1_b().read_async().await?;
        callback(49 + 0 * 0, "charge_option_1_b", reg.into());
        let reg = self.charge_option_2_a().read_async().await?;
        callback(50 + 0 * 0, "charge_option_2_a", reg.into());
        let reg = self.charge_option_2_b().read_async().await?;
        callback(51 + 0 * 0, "charge_option_2_b", reg.into());
        let reg = self.charge_option_3_a().read_async().await?;
        callback(52 + 0 * 0, "charge_option_3_a", reg.into());
        let reg = self.charge_option_3_b().read_async().await?;
        callback(53 + 0 * 0, "charge_option_3_b", reg.into());
        let reg = self.prochot_option_0_a().read_async().await?;
        callback(54 + 0 * 0, "prochot_option_0_a", reg.into());
        let reg = self.prochot_option_0_b().read_async().await?;
        callback(55 + 0 * 0, "prochot_option_0_b", reg.into());
        let reg = self.prochot_option_1_a().read_async().await?;
        callback(56 + 0 * 0, "prochot_option_1_a", reg.into());
        let reg = self.prochot_option_1_b().read_async().await?;
        callback(57 + 0 * 0, "prochot_option_1_b", reg.into());
        let reg = self.adc_option_a().read_async().await?;
        callback(58 + 0 * 0, "adc_option_a", reg.into());
        let reg = self.adc_option_b().read_async().await?;
        callback(59 + 0 * 0, "adc_option_b", reg.into());
        let reg = self.charge_option_4_a().read_async().await?;
        callback(60 + 0 * 0, "charge_option_4_a", reg.into());
        let reg = self.charge_option_4_b().read_async().await?;
        callback(61 + 0 * 0, "charge_option_4_b", reg.into());
        let reg = self.vmin_active_protection_a().read_async().await?;
        callback(62 + 0 * 0, "vmin_active_protection_a", reg.into());
        let reg = self.vmin_active_protection_b().read_async().await?;
        callback(63 + 0 * 0, "vmin_active_protection_b", reg.into());
        let reg = self.autotune_read_b().read_async().await?;
        callback(96 + 0 * 0, "autotune_read_b", reg.into());
        let reg = self.autotune_read_a().read_async().await?;
        callback(97 + 0 * 0, "autotune_read_a", reg.into());
        let reg = self.autotune_force_b().read_async().await?;
        callback(98 + 0 * 0, "autotune_force_b", reg.into());
        let reg = self.autotune_force_a().read_async().await?;
        callback(99 + 0 * 0, "autotune_force_a", reg.into());
        let reg = self.gm_adjust_force_a().read_async().await?;
        callback(100 + 0 * 0, "gm_adjust_force_a", reg.into());
        let reg = self.gm_adjust_force_b().read_async().await?;
        callback(101 + 0 * 0, "gm_adjust_force_b", reg.into());
        let reg = self.virtual_control_a().read_async().await?;
        callback(128 + 0 * 0, "virtual_control_a", reg.into());
        let reg = self.virtual_control_b().read_async().await?;
        callback(129 + 0 * 0, "virtual_control_b", reg.into());
        Ok(())
    }
    pub fn charge_option_0_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption0A, ::device_driver::RW> {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption0A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption0A::new,
        )
    }
    pub fn charge_option_0_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption0B, ::device_driver::RW> {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption0B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption0B::new,
        )
    }
    pub fn charge_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeCurrent, ::device_driver::RW> {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeCurrent, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeCurrent::new,
        )
    }
    pub fn charge_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeVoltage, ::device_driver::RW> {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeVoltage, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeVoltage::new,
        )
    }
    pub fn iin_host(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::IinHost, ::device_driver::RW> {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::IinHost, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::IinHost::new,
        )
    }
    pub fn vindpm(&mut self) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::Vindpm, ::device_driver::RW> {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::Vindpm, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::Vindpm::new,
        )
    }
    pub fn otg_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::OtgCurrent, ::device_driver::RW> {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::OtgCurrent, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::OtgCurrent::new,
        )
    }
    pub fn otg_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::OtgVoltage, ::device_driver::RW> {
        let address = self.base_address + 12;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::OtgVoltage, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::OtgVoltage::new,
        )
    }
    pub fn vsys_min(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VsysMin, ::device_driver::RW> {
        let address = self.base_address + 14;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VsysMin, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VsysMin::new,
        )
    }
    pub fn charge_profile_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeProfileA, ::device_driver::RW> {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeProfileA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeProfileA::new,
        )
    }
    pub fn charge_profile_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeProfileB, ::device_driver::RW> {
        let address = self.base_address + 17;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeProfileB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeProfileB::new,
        )
    }
    pub fn gate_drive_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::GateDriveA, ::device_driver::RW> {
        let address = self.base_address + 18;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::GateDriveA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::GateDriveA::new,
        )
    }
    pub fn gate_drive_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::GateDriveB, ::device_driver::RW> {
        let address = self.base_address + 19;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::GateDriveB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::GateDriveB::new,
        )
    }
    pub fn charge_option_5_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption5A, ::device_driver::RW> {
        let address = self.base_address + 20;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption5A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption5A::new,
        )
    }
    pub fn charge_option_5_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption5B, ::device_driver::RW> {
        let address = self.base_address + 21;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption5B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption5B::new,
        )
    }
    pub fn auto_charge_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AutoChargeA, ::device_driver::RW> {
        let address = self.base_address + 22;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AutoChargeA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AutoChargeA::new,
        )
    }
    pub fn auto_charge_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AutoChargeB, ::device_driver::RW> {
        let address = self.base_address + 23;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AutoChargeB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AutoChargeB::new,
        )
    }
    pub fn charger_status_0_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargerStatus0A, ::device_driver::RO> {
        let address = self.base_address + 24;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargerStatus0A, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ChargerStatus0A::new,
        )
    }
    pub fn charger_status_0_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargerStatus0B, ::device_driver::RO> {
        let address = self.base_address + 25;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargerStatus0B, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ChargerStatus0B::new,
        )
    }
    pub fn adc_vbat(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcVbat, ::device_driver::RO> {
        let address = self.base_address + 26;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcVbat, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcVbat::new,
        )
    }
    pub fn adc_psys(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcPsys, ::device_driver::RO> {
        let address = self.base_address + 28;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcPsys, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcPsys::new,
        )
    }
    pub fn adc_cmpin_tr(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcCmpinTr, ::device_driver::RO> {
        let address = self.base_address + 30;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcCmpinTr, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcCmpinTr::new,
        )
    }
    pub fn charger_status_1_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargerStatus1A, ::device_driver::RW> {
        let address = self.base_address + 32;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargerStatus1A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargerStatus1A::new,
        )
    }
    pub fn charger_status_1_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargerStatus1B, ::device_driver::RO> {
        let address = self.base_address + 33;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargerStatus1B, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ChargerStatus1B::new,
        )
    }
    pub fn prochot_status_reg_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotStatusRegA, ::device_driver::RW> {
        let address = self.base_address + 34;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotStatusRegA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotStatusRegA::new,
        )
    }
    pub fn prochot_status_reg_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotStatusRegB, ::device_driver::RW> {
        let address = self.base_address + 35;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotStatusRegB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotStatusRegB::new,
        )
    }
    pub fn iin_dpm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::IinDpm, ::device_driver::RW> {
        let address = self.base_address + 36;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::IinDpm, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::IinDpm::new,
        )
    }
    pub fn adc_vbus(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcVbus, ::device_driver::RO> {
        let address = self.base_address + 38;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcVbus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcVbus::new,
        )
    }
    pub fn adc_ibat(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcIbat, ::device_driver::RO> {
        let address = self.base_address + 40;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcIbat, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcIbat::new,
        )
    }
    pub fn adc_iin(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcIin, ::device_driver::RO> {
        let address = self.base_address + 42;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcIin, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcIin::new,
        )
    }
    pub fn adc_vsys(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcVsys, ::device_driver::RO> {
        let address = self.base_address + 44;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcVsys, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcVsys::new,
        )
    }
    pub fn manufacture_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ManufactureId, ::device_driver::RW> {
        let address = self.base_address + 46;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ManufactureId, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ManufactureId::new,
        )
    }
    pub fn device_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::DeviceId, ::device_driver::RW> {
        let address = self.base_address + 47;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::DeviceId, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::DeviceId::new,
        )
    }
    pub fn charge_option_1_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption1A, ::device_driver::RW> {
        let address = self.base_address + 48;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption1A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption1A::new,
        )
    }
    pub fn charge_option_1_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption1B, ::device_driver::RW> {
        let address = self.base_address + 49;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption1B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption1B::new,
        )
    }
    pub fn charge_option_2_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption2A, ::device_driver::RW> {
        let address = self.base_address + 50;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption2A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption2A::new,
        )
    }
    pub fn charge_option_2_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption2B, ::device_driver::RW> {
        let address = self.base_address + 51;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption2B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption2B::new,
        )
    }
    pub fn charge_option_3_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption3A, ::device_driver::RW> {
        let address = self.base_address + 52;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption3A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption3A::new,
        )
    }
    pub fn charge_option_3_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption3B, ::device_driver::RW> {
        let address = self.base_address + 53;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption3B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption3B::new,
        )
    }
    pub fn prochot_option_0_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption0A, ::device_driver::RW> {
        let address = self.base_address + 54;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption0A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption0A::new,
        )
    }
    pub fn prochot_option_0_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption0B, ::device_driver::RW> {
        let address = self.base_address + 55;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption0B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption0B::new,
        )
    }
    pub fn prochot_option_1_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption1A, ::device_driver::RW> {
        let address = self.base_address + 56;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption1A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption1A::new,
        )
    }
    pub fn prochot_option_1_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption1B, ::device_driver::RW> {
        let address = self.base_address + 57;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption1B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption1B::new,
        )
    }
    pub fn adc_option_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcOptionA, ::device_driver::RW> {
        let address = self.base_address + 58;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcOptionA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AdcOptionA::new,
        )
    }
    pub fn adc_option_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcOptionB, ::device_driver::RW> {
        let address = self.base_address + 59;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcOptionB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AdcOptionB::new,
        )
    }
    pub fn charge_option_4_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption4A, ::device_driver::RW> {
        let address = self.base_address + 60;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption4A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption4A::new,
        )
    }
    pub fn charge_option_4_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption4B, ::device_driver::RW> {
        let address = self.base_address + 61;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption4B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption4B::new,
        )
    }
    pub fn vmin_active_protection_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VminActiveProtectionA, ::device_driver::RW> {
        let address = self.base_address + 62;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VminActiveProtectionA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VminActiveProtectionA::new,
        )
    }
    pub fn vmin_active_protection_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VminActiveProtectionB, ::device_driver::RW> {
        let address = self.base_address + 63;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VminActiveProtectionB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VminActiveProtectionB::new,
        )
    }
    pub fn autotune_read_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AutotuneReadB, ::device_driver::RO> {
        let address = self.base_address + 96;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AutotuneReadB, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AutotuneReadB::new,
        )
    }
    pub fn autotune_read_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AutotuneReadA, ::device_driver::RO> {
        let address = self.base_address + 97;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AutotuneReadA, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AutotuneReadA::new,
        )
    }
    pub fn autotune_force_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AutotuneForceB, ::device_driver::RW> {
        let address = self.base_address + 98;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AutotuneForceB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AutotuneForceB::new,
        )
    }
    pub fn autotune_force_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AutotuneForceA, ::device_driver::RW> {
        let address = self.base_address + 99;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AutotuneForceA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AutotuneForceA::new,
        )
    }
    pub fn gm_adjust_force_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::GmAdjustForceA, ::device_driver::RW> {
        let address = self.base_address + 100;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::GmAdjustForceA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::GmAdjustForceA::new,
        )
    }
    pub fn gm_adjust_force_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::GmAdjustForceB, ::device_driver::RW> {
        let address = self.base_address + 101;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::GmAdjustForceB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::GmAdjustForceB::new,
        )
    }
    pub fn virtual_control_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VirtualControlA, ::device_driver::RW> {
        let address = self.base_address + 128;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VirtualControlA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VirtualControlA::new,
        )
    }
    pub fn virtual_control_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VirtualControlB, ::device_driver::RW> {
        let address = self.base_address + 129;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VirtualControlB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VirtualControlB::new,
        )
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption0A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption0A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption0A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [14] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `chrg_inhibit` field of the register.
        ///
        /// Charge Inhibit. When bit is 0 battery charging will start with valid values in the CHARGE_VOLTAGE() and CHARGE_CURRENT().
        pub fn chrg_inhibit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_iin_dpm` field of the register.
        ///
        /// IIN_DPM Enable. Host writes this bit to enable IIN_DPM regulation loop. When the IIN_DPM is disabled by the charger (refer to IIN_DPM_AUTO_DISABLE), this bit goes LOW. Under OTG mode, this bit is also used to enable/disable IOTG regulation.
        pub fn en_iin_dpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_ldo` field of the register.
        ///
        /// LDO Mode Enable. When battery voltage is below VSYS_MIN(), the charger is in pre-charge with LDO mode enabled.
        pub fn en_ldo(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `ibat_gain` field of the register.
        ///
        /// IBAT Amplifier Ratio. The ratio of voltage on IBAT and voltage across SRP and SRN.
        pub fn ibat_gain(&self) -> super::IbatGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `iadpt_gain` field of the register.
        ///
        /// IADPT Amplifier Ratio. The ratio of voltage on IADPT and voltage across ACP and ACN.
        pub fn iadpt_gain(&self) -> super::IadptGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_learn` field of the register.
        ///
        /// LEARN mode function enable.
        pub fn en_learn(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `vsys_uvp_enz` field of the register.
        ///
        /// Disable system under voltage protection.
        pub fn vsys_uvp_enz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_cmp_latch` field of the register.
        ///
        /// Enable Latch of Independent Comparator. Comparator output with effective low. If enabled in PROCHOT profile PP_CMP=1b, STAT_COMP bit keep 1b after triggered until read by host and clear. host can clear CMPOUT pin by toggling this EN_CMP_LATCH bit.
        pub fn en_cmp_latch(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `chrg_inhibit` field of the register.
        ///
        /// Charge Inhibit. When bit is 0 battery charging will start with valid values in the CHARGE_VOLTAGE() and CHARGE_CURRENT().
        pub fn set_chrg_inhibit(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_iin_dpm` field of the register.
        ///
        /// IIN_DPM Enable. Host writes this bit to enable IIN_DPM regulation loop. When the IIN_DPM is disabled by the charger (refer to IIN_DPM_AUTO_DISABLE), this bit goes LOW. Under OTG mode, this bit is also used to enable/disable IOTG regulation.
        pub fn set_en_iin_dpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_ldo` field of the register.
        ///
        /// LDO Mode Enable. When battery voltage is below VSYS_MIN(), the charger is in pre-charge with LDO mode enabled.
        pub fn set_en_ldo(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `ibat_gain` field of the register.
        ///
        /// IBAT Amplifier Ratio. The ratio of voltage on IBAT and voltage across SRP and SRN.
        pub fn set_ibat_gain(&mut self, value: super::IbatGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `iadpt_gain` field of the register.
        ///
        /// IADPT Amplifier Ratio. The ratio of voltage on IADPT and voltage across ACP and ACN.
        pub fn set_iadpt_gain(&mut self, value: super::IadptGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_learn` field of the register.
        ///
        /// LEARN mode function enable.
        pub fn set_en_learn(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `vsys_uvp_enz` field of the register.
        ///
        /// Disable system under voltage protection.
        pub fn set_vsys_uvp_enz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_cmp_latch` field of the register.
        ///
        /// Enable Latch of Independent Comparator. Comparator output with effective low. If enabled in PROCHOT profile PP_CMP=1b, STAT_COMP bit keep 1b after triggered until read by host and clear. host can clear CMPOUT pin by toggling this EN_CMP_LATCH bit.
        pub fn set_en_cmp_latch(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption0A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption0A> for [u8; 1] {
        fn from(val: ChargeOption0A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption0A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption0A");
            d.field("chrg_inhibit", &self.chrg_inhibit());
            d.field("en_iin_dpm", &self.en_iin_dpm());
            d.field("en_ldo", &self.en_ldo());
            d.field("ibat_gain", &self.ibat_gain());
            d.field("iadpt_gain", &self.iadpt_gain());
            d.field("en_learn", &self.en_learn());
            d.field("vsys_uvp_enz", &self.vsys_uvp_enz());
            d.field("en_cmp_latch", &self.en_cmp_latch());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption0A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption0A {{ ");
            defmt::write!(f, "chrg_inhibit: {=bool}, ", &self.chrg_inhibit());
            defmt::write!(f, "en_iin_dpm: {=bool}, ", &self.en_iin_dpm());
            defmt::write!(f, "en_ldo: {=bool}, ", &self.en_ldo());
            defmt::write!(f, "ibat_gain: {}, ", &self.ibat_gain());
            defmt::write!(f, "iadpt_gain: {}, ", &self.iadpt_gain());
            defmt::write!(f, "en_learn: {=bool}, ", &self.en_learn());
            defmt::write!(f, "vsys_uvp_enz: {=bool}, ", &self.vsys_uvp_enz());
            defmt::write!(f, "en_cmp_latch: {=bool}, ", &self.en_cmp_latch());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption0A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption0A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption0A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption0A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption0A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption0A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption0A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption0B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption0B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption0B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [231] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_batovp` field of the register.
        ///
        /// Enable BATOVP protection.
        pub fn en_batovp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `pwm_freq` field of the register.
        ///
        /// Switching Frequency Selection.
        pub fn pwm_freq(&self) -> super::SwitchingFreq {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_ooa` field of the register.
        ///
        /// Out-of-Audio Enable.
        pub fn en_ooa(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `otg_on_chrgok` field of the register.
        ///
        /// Add OTG to CHRG_OK.
        pub fn otg_on_chrgok(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `iin_dpm_auto_disable` field of the register.
        ///
        /// IIN_DPM Auto Disable.
        pub fn iin_dpm_auto_disable(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `wdtmr_adj` field of the register.
        ///
        /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command.
        pub fn wdtmr_adj(&self) -> super::MaxDelay {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 7) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_lwpwr` field of the register.
        ///
        /// Low Power Mode enable.
        pub fn en_lwpwr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_batovp` field of the register.
        ///
        /// Enable BATOVP protection.
        pub fn set_en_batovp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `pwm_freq` field of the register.
        ///
        /// Switching Frequency Selection.
        pub fn set_pwm_freq(&mut self, value: super::SwitchingFreq) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_ooa` field of the register.
        ///
        /// Out-of-Audio Enable.
        pub fn set_en_ooa(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `otg_on_chrgok` field of the register.
        ///
        /// Add OTG to CHRG_OK.
        pub fn set_otg_on_chrgok(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `iin_dpm_auto_disable` field of the register.
        ///
        /// IIN_DPM Auto Disable.
        pub fn set_iin_dpm_auto_disable(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `wdtmr_adj` field of the register.
        ///
        /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command.
        pub fn set_wdtmr_adj(&mut self, value: super::MaxDelay) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 7, &mut self.bits) };
        }
        ///Write the `en_lwpwr` field of the register.
        ///
        /// Low Power Mode enable.
        pub fn set_en_lwpwr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption0B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption0B> for [u8; 1] {
        fn from(val: ChargeOption0B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption0B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption0B");
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
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption0B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption0B {{ ");
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
    impl core::ops::BitAnd for ChargeOption0B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption0B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption0B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption0B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption0B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption0B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption0B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeCurrent {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeCurrent {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeCurrent {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `charge_current` field of the register.
        ///
        /// Charge current setting with 5mΩ sense resistor.
        pub fn charge_current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 3, 14) };
            raw
        }
        ///Write the `charge_current` field of the register.
        ///
        /// Charge current setting with 5mΩ sense resistor.
        pub fn set_charge_current(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 3, 14, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeVoltage {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeVoltage {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeVoltage {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `charge_voltage` field of the register.
        ///
        /// Charge voltage setting.
        pub fn charge_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 2, 15) };
            raw
        }
        ///Write the `charge_voltage` field of the register.
        ///
        /// Charge voltage setting.
        pub fn set_charge_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 2, 15, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IinHost {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for IinHost {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl IinHost {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [32, 3] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `iin_host` field of the register.
        ///
        /// Maximum input current limit with 10mΩ sense resistor.
        pub fn iin_host(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 2, 11) };
            raw
        }
        ///Write the `iin_host` field of the register.
        ///
        /// Maximum input current limit with 10mΩ sense resistor.
        pub fn set_iin_host(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 2, 11, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vindpm {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for Vindpm {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Vindpm {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [128, 2] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vindpm` field of the register.
        ///
        /// Input voltage limit.
        pub fn vindpm(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 2, 13) };
            raw
        }
        ///Write the `vindpm` field of the register.
        ///
        /// Input voltage limit.
        pub fn set_vindpm(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 2, 13, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtgCurrent {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for OtgCurrent {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl OtgCurrent {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [224, 1] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `otg_current` field of the register.
        ///
        /// OTG output current limit with 10mΩ Rac current sense.
        pub fn otg_current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 2, 11) };
            raw
        }
        ///Write the `otg_current` field of the register.
        ///
        /// OTG output current limit with 10mΩ Rac current sense.
        pub fn set_otg_current(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 2, 11, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtgVoltage {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for OtgVoltage {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl OtgVoltage {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [232, 3] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `otg_voltage` field of the register.
        ///
        /// OTG output voltage regulation.
        pub fn otg_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 2, 13) };
            raw
        }
        ///Write the `otg_voltage` field of the register.
        ///
        /// OTG output voltage regulation.
        pub fn set_otg_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 2, 13, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VsysMin {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for VsysMin {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl VsysMin {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [40, 5] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vsys_min` field of the register.
        ///
        /// Minimum system voltage configuration register.
        pub fn vsys_min(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 13) };
            raw
        }
        ///Write the `vsys_min` field of the register.
        ///
        /// Minimum system voltage configuration register.
        pub fn set_vsys_min(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 13, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeProfileA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeProfileA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeProfileA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [32] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `iterm` field of the register.
        ///
        /// Termination current setting with 5mΩ sense resistor.
        pub fn iterm(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `iterm` field of the register.
        ///
        /// Termination current setting with 5mΩ sense resistor.
        pub fn set_iterm(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeProfileA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeProfileA> for [u8; 1] {
        fn from(val: ChargeProfileA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeProfileA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeProfileA");
            d.field("iterm", &self.iterm());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeProfileA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeProfileA {{ ");
            defmt::write!(f, "iterm: {=u8}, ", &self.iterm());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeProfileA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeProfileA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeProfileA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeProfileA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeProfileA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeProfileA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeProfileA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeProfileB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeProfileB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeProfileB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [48] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `iprechg` field of the register.
        ///
        /// Maximum precharge current clamp setting with 5mΩ sense resistor (The lower setting of CHARGE_CURRENT() and IPRECHG determine the practical precharge current when VBAT< VSYS_MIN()).
        pub fn iprechg(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `iprechg` field of the register.
        ///
        /// Maximum precharge current clamp setting with 5mΩ sense resistor (The lower setting of CHARGE_CURRENT() and IPRECHG determine the practical precharge current when VBAT< VSYS_MIN()).
        pub fn set_iprechg(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeProfileB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeProfileB> for [u8; 1] {
        fn from(val: ChargeProfileB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeProfileB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeProfileB");
            d.field("iprechg", &self.iprechg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeProfileB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeProfileB {{ ");
            defmt::write!(f, "iprechg: {=u8}, ", &self.iprechg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeProfileB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeProfileB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeProfileB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeProfileB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeProfileB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeProfileB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeProfileB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GateDriveA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for GateDriveA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GateDriveA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [108] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `vsys_reg_slow` field of the register.
        ///
        /// System regulation loop bandwidth slow down to reduce input current overshoot during load transient.
        pub fn vsys_reg_slow(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `lodrv_2_stat` field of the register.
        ///
        /// Suggested LODRV2 LS MOSFET gate drive strength adjustment for both turn on and turn off.
        pub fn lodrv_2_stat(&self) -> super::Lodrv2GateDriveStrengthAdjustment {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `hidrv_2_stat` field of the register.
        ///
        /// Suggested HIDRV2 HS MOSFET gate drive strength adjustment for both turn on and turn off.
        pub fn hidrv_2_stat(&self) -> super::Hidrv2GateDriveStrengthAdjustment {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `vsys_reg_slow` field of the register.
        ///
        /// System regulation loop bandwidth slow down to reduce input current overshoot during load transient.
        pub fn set_vsys_reg_slow(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `lodrv_2_stat` field of the register.
        ///
        /// Suggested LODRV2 LS MOSFET gate drive strength adjustment for both turn on and turn off.
        pub fn set_lodrv_2_stat(&mut self, value: super::Lodrv2GateDriveStrengthAdjustment) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 5, &mut self.bits) };
        }
        ///Write the `hidrv_2_stat` field of the register.
        ///
        /// Suggested HIDRV2 HS MOSFET gate drive strength adjustment for both turn on and turn off.
        pub fn set_hidrv_2_stat(&mut self, value: super::Hidrv2GateDriveStrengthAdjustment) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for GateDriveA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<GateDriveA> for [u8; 1] {
        fn from(val: GateDriveA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for GateDriveA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("GateDriveA");
            d.field("vsys_reg_slow", &self.vsys_reg_slow());
            d.field("lodrv_2_stat", &self.lodrv_2_stat());
            d.field("hidrv_2_stat", &self.hidrv_2_stat());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for GateDriveA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GateDriveA {{ ");
            defmt::write!(f, "vsys_reg_slow: {=bool}, ", &self.vsys_reg_slow());
            defmt::write!(f, "lodrv_2_stat: {}, ", &self.lodrv_2_stat());
            defmt::write!(f, "hidrv_2_stat: {}, ", &self.hidrv_2_stat());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for GateDriveA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for GateDriveA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for GateDriveA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for GateDriveA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for GateDriveA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for GateDriveA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for GateDriveA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GateDriveB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for GateDriveB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GateDriveB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [108] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `batovp_extend` field of the register.
        ///
        /// Enable BATOVP for both charge enable and disable scenarios including AC+battery and battery only.
        pub fn batovp_extend(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `lodrv_1_stat` field of the register.
        ///
        /// Suggested LODRV1_A and LODRV1_B LS MOSFET gate drive strength adjustment for both turn on and turn off.
        pub fn lodrv_1_stat(&self) -> super::Lodrv1GateDriveStrengthAdjustment {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `hidrv_1_stat` field of the register.
        ///
        /// Suggested HIDRV1_A and HIDRV1_B HS MOSFET gate drive strength adjustment for both turn on and turn off.
        pub fn hidrv_1_stat(&self) -> super::Hidrv1GateDriveStrengthAdjustment {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `batovp_extend` field of the register.
        ///
        /// Enable BATOVP for both charge enable and disable scenarios including AC+battery and battery only.
        pub fn set_batovp_extend(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `lodrv_1_stat` field of the register.
        ///
        /// Suggested LODRV1_A and LODRV1_B LS MOSFET gate drive strength adjustment for both turn on and turn off.
        pub fn set_lodrv_1_stat(&mut self, value: super::Lodrv1GateDriveStrengthAdjustment) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 5, &mut self.bits) };
        }
        ///Write the `hidrv_1_stat` field of the register.
        ///
        /// Suggested HIDRV1_A and HIDRV1_B HS MOSFET gate drive strength adjustment for both turn on and turn off.
        pub fn set_hidrv_1_stat(&mut self, value: super::Hidrv1GateDriveStrengthAdjustment) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for GateDriveB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<GateDriveB> for [u8; 1] {
        fn from(val: GateDriveB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for GateDriveB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("GateDriveB");
            d.field("batovp_extend", &self.batovp_extend());
            d.field("lodrv_1_stat", &self.lodrv_1_stat());
            d.field("hidrv_1_stat", &self.hidrv_1_stat());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for GateDriveB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GateDriveB {{ ");
            defmt::write!(f, "batovp_extend: {=bool}, ", &self.batovp_extend());
            defmt::write!(f, "lodrv_1_stat: {}, ", &self.lodrv_1_stat());
            defmt::write!(f, "hidrv_1_stat: {}, ", &self.hidrv_1_stat());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for GateDriveB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for GateDriveB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for GateDriveB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for GateDriveB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for GateDriveB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for GateDriveB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for GateDriveB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption5A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption5A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption5A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [133] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `ph_drop_deg` field of the register.
        ///
        /// Adjust dual phase to single phase (phase dropping transition) deglitch time.
        pub fn ph_drop_deg(&self) -> super::PhaseDroppingTransitionDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ph_add_deg` field of the register.
        ///
        /// Adjust single phase to dual phase (phase adding transition) deglitch time.
        pub fn ph_add_deg(&self) -> super::PhaseAddingTransitionDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `force_single` field of the register.
        ///
        /// Force single phase operation under buck mode when quasi dual phase is chosen through MODE pin programming.
        pub fn force_single(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `single_dual_trans_th` field of the register.
        ///
        /// Buck mode single to dual phase transition threshold adjustment based on output load current.
        pub fn single_dual_trans_th(&self) -> super::SingleDualTransThreshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `ph_drop_deg` field of the register.
        ///
        /// Adjust dual phase to single phase (phase dropping transition) deglitch time.
        pub fn set_ph_drop_deg(&mut self, value: super::PhaseDroppingTransitionDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `ph_add_deg` field of the register.
        ///
        /// Adjust single phase to dual phase (phase adding transition) deglitch time.
        pub fn set_ph_add_deg(&mut self, value: super::PhaseAddingTransitionDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 4, &mut self.bits) };
        }
        ///Write the `force_single` field of the register.
        ///
        /// Force single phase operation under buck mode when quasi dual phase is chosen through MODE pin programming.
        pub fn set_force_single(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `single_dual_trans_th` field of the register.
        ///
        /// Buck mode single to dual phase transition threshold adjustment based on output load current.
        pub fn set_single_dual_trans_th(&mut self, value: super::SingleDualTransThreshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption5A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption5A> for [u8; 1] {
        fn from(val: ChargeOption5A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption5A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption5A");
            d.field("ph_drop_deg", &self.ph_drop_deg());
            d.field("ph_add_deg", &self.ph_add_deg());
            d.field("force_single", &self.force_single());
            d.field("single_dual_trans_th", &self.single_dual_trans_th());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption5A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption5A {{ ");
            defmt::write!(f, "ph_drop_deg: {}, ", &self.ph_drop_deg());
            defmt::write!(f, "ph_add_deg: {}, ", &self.ph_add_deg());
            defmt::write!(f, "force_single: {=bool}, ", &self.force_single());
            defmt::write!(f, "single_dual_trans_th: {}, ", &self.single_dual_trans_th());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption5A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption5A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption5A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption5A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption5A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption5A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption5A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption5B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption5B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption5B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [6] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `batcoc_config` field of the register.
        ///
        /// Disable BATCOC and configure BATCOC thresholds across SRP-SRN.
        pub fn batcoc_config(&self) -> super::BatcocThreshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_regn_lwpwr` field of the register.
        ///
        /// Enable REGN with scale down current 5mA capability under battery only and low power mode.
        pub fn en_regn_lwpwr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `regn_ext` field of the register.
        ///
        /// Enable external 5V overdrive for REGN.
        pub fn regn_ext(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `cmpin_tr_select` field of the register.
        ///
        /// CMPIN_TR pin function selection.
        pub fn cmpin_tr_select(&self) -> super::CmpinFuncSelect {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `wd_rst` field of the register.
        ///
        /// Reset watch dog timer control.
        pub fn wd_rst(&self) -> super::WatchDogReset {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ptm_exit_light_load` field of the register.
        ///
        /// Enable PTM auto exit under light load.
        pub fn ptm_exit_light_load(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `batcoc_config` field of the register.
        ///
        /// Disable BATCOC and configure BATCOC thresholds across SRP-SRN.
        pub fn set_batcoc_config(&mut self, value: super::BatcocThreshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 3, &mut self.bits) };
        }
        ///Write the `en_regn_lwpwr` field of the register.
        ///
        /// Enable REGN with scale down current 5mA capability under battery only and low power mode.
        pub fn set_en_regn_lwpwr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `regn_ext` field of the register.
        ///
        /// Enable external 5V overdrive for REGN.
        pub fn set_regn_ext(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `cmpin_tr_select` field of the register.
        ///
        /// CMPIN_TR pin function selection.
        pub fn set_cmpin_tr_select(&mut self, value: super::CmpinFuncSelect) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<i8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `wd_rst` field of the register.
        ///
        /// Reset watch dog timer control.
        pub fn set_wd_rst(&mut self, value: super::WatchDogReset) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<i8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `ptm_exit_light_load` field of the register.
        ///
        /// Enable PTM auto exit under light load.
        pub fn set_ptm_exit_light_load(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption5B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption5B> for [u8; 1] {
        fn from(val: ChargeOption5B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption5B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption5B");
            d.field("batcoc_config", &self.batcoc_config());
            d.field("en_regn_lwpwr", &self.en_regn_lwpwr());
            d.field("regn_ext", &self.regn_ext());
            d.field("cmpin_tr_select", &self.cmpin_tr_select());
            d.field("wd_rst", &self.wd_rst());
            d.field("ptm_exit_light_load", &self.ptm_exit_light_load());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption5B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption5B {{ ");
            defmt::write!(f, "batcoc_config: {}, ", &self.batcoc_config());
            defmt::write!(f, "en_regn_lwpwr: {=bool}, ", &self.en_regn_lwpwr());
            defmt::write!(f, "regn_ext: {=bool}, ", &self.regn_ext());
            defmt::write!(f, "cmpin_tr_select: {}, ", &self.cmpin_tr_select());
            defmt::write!(f, "wd_rst: {}, ", &self.wd_rst());
            defmt::write!(f, "ptm_exit_light_load: {=bool}, ", &self.ptm_exit_light_load());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption5B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption5B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption5B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption5B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption5B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption5B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption5B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutoChargeA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AutoChargeA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AutoChargeA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [194] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `acov_adj` field of the register.
        ///
        /// ACOV protection threshold adjustment.
        pub fn acov_adj(&self) -> super::AcovThreshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `thermal_deg` field of the register.
        ///
        /// Adjust TREG thermal deglitch time to trigger prochot profile pull down pulse.
        pub fn thermal_deg(&self) -> super::ThermalDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `stat_thermal` field of the register.
        ///
        /// PROCHOT profile status bit for TREG thermal overheat (CMPIN_TR< 1.1V). The status is latched until a read from host.
        pub fn stat_thermal(&self) -> super::ProchotStatusOverheat {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pp_thermal` field of the register.
        ///
        /// Enable temperature regulation(TREG) for PROCHOT profile.
        pub fn pp_thermal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `en_treg` field of the register.
        ///
        /// Enable temperature regulation function.
        pub fn en_treg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `en_chg_tmr` field of the register.
        ///
        /// Enable charge safety timer.
        pub fn en_chg_tmr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_tmr_2_x` field of the register.
        ///
        /// Charge Safety Timer speed control (Note changing the state of EN_TMR2X only impacts the rate at which the counter is counting and has no effect on any existing accumulated count).
        pub fn en_tmr_2_x(&self) -> super::ChgTmrSpeedCtrl {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `acov_adj` field of the register.
        ///
        /// ACOV protection threshold adjustment.
        pub fn set_acov_adj(&mut self, value: super::AcovThreshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `thermal_deg` field of the register.
        ///
        /// Adjust TREG thermal deglitch time to trigger prochot profile pull down pulse.
        pub fn set_thermal_deg(&mut self, value: super::ThermalDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `pp_thermal` field of the register.
        ///
        /// Enable temperature regulation(TREG) for PROCHOT profile.
        pub fn set_pp_thermal(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_treg` field of the register.
        ///
        /// Enable temperature regulation function.
        pub fn set_en_treg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_chg_tmr` field of the register.
        ///
        /// Enable charge safety timer.
        pub fn set_en_chg_tmr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_tmr_2_x` field of the register.
        ///
        /// Charge Safety Timer speed control (Note changing the state of EN_TMR2X only impacts the rate at which the counter is counting and has no effect on any existing accumulated count).
        pub fn set_en_tmr_2_x(&mut self, value: super::ChgTmrSpeedCtrl) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<i8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for AutoChargeA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AutoChargeA> for [u8; 1] {
        fn from(val: AutoChargeA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AutoChargeA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AutoChargeA");
            d.field("acov_adj", &self.acov_adj());
            d.field("thermal_deg", &self.thermal_deg());
            d.field("stat_thermal", &self.stat_thermal());
            d.field("pp_thermal", &self.pp_thermal());
            d.field("en_treg", &self.en_treg());
            d.field("en_chg_tmr", &self.en_chg_tmr());
            d.field("en_tmr_2_x", &self.en_tmr_2_x());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AutoChargeA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AutoChargeA {{ ");
            defmt::write!(f, "acov_adj: {}, ", &self.acov_adj());
            defmt::write!(f, "thermal_deg: {}, ", &self.thermal_deg());
            defmt::write!(f, "stat_thermal: {}, ", &self.stat_thermal());
            defmt::write!(f, "pp_thermal: {=bool}, ", &self.pp_thermal());
            defmt::write!(f, "en_treg: {=bool}, ", &self.en_treg());
            defmt::write!(f, "en_chg_tmr: {=bool}, ", &self.en_chg_tmr());
            defmt::write!(f, "en_tmr_2_x: {}, ", &self.en_tmr_2_x());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AutoChargeA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AutoChargeA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AutoChargeA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AutoChargeA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AutoChargeA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AutoChargeA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AutoChargeA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutoChargeB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AutoChargeB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AutoChargeB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [1] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `chg_tmr` field of the register.
        ///
        /// Automatic Charge Safety Timer control.
        pub fn chg_tmr(&self) -> super::ChgTmrCtrl {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vrechg` field of the register.
        ///
        /// Battery automatic recharge threshold below CHARGE_VOLTAGE().
        pub fn vrechg(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 6) };
            raw
        }
        ///Read the `chrg_ok_int` field of the register.
        ///
        /// Enable CHRG_OK pin for interrupt function.
        pub fn chrg_ok_int(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_auto_chg` field of the register.
        ///
        /// Automatic charge control (recharge and terminate battery charging automatically).
        pub fn en_auto_chg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `chg_tmr` field of the register.
        ///
        /// Automatic Charge Safety Timer control.
        pub fn set_chg_tmr(&mut self, value: super::ChgTmrCtrl) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `vrechg` field of the register.
        ///
        /// Battery automatic recharge threshold below CHARGE_VOLTAGE().
        pub fn set_vrechg(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 6, &mut self.bits) };
        }
        ///Write the `chrg_ok_int` field of the register.
        ///
        /// Enable CHRG_OK pin for interrupt function.
        pub fn set_chrg_ok_int(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_auto_chg` field of the register.
        ///
        /// Automatic charge control (recharge and terminate battery charging automatically).
        pub fn set_en_auto_chg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for AutoChargeB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AutoChargeB> for [u8; 1] {
        fn from(val: AutoChargeB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AutoChargeB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AutoChargeB");
            d.field("chg_tmr", &self.chg_tmr());
            d.field("vrechg", &self.vrechg());
            d.field("chrg_ok_int", &self.chrg_ok_int());
            d.field("en_auto_chg", &self.en_auto_chg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AutoChargeB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AutoChargeB {{ ");
            defmt::write!(f, "chg_tmr: {}, ", &self.chg_tmr());
            defmt::write!(f, "vrechg: {=u8}, ", &self.vrechg());
            defmt::write!(f, "chrg_ok_int: {=bool}, ", &self.chrg_ok_int());
            defmt::write!(f, "en_auto_chg: {=bool}, ", &self.en_auto_chg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AutoChargeB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AutoChargeB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AutoChargeB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AutoChargeB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AutoChargeB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AutoChargeB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AutoChargeB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargerStatus0A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargerStatus0A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargerStatus0A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `fault_regn` field of the register.
        ///
        /// REGN fault detected.
        pub fn fault_regn(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `fault_ocp` field of the register.
        ///
        /// OCP fault detected.
        pub fn fault_ocp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `fault_batovp` field of the register.
        ///
        /// BATOVP fault detected.
        pub fn fault_batovp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
    }
    impl From<[u8; 1]> for ChargerStatus0A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargerStatus0A> for [u8; 1] {
        fn from(val: ChargerStatus0A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargerStatus0A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargerStatus0A");
            d.field("fault_regn", &self.fault_regn());
            d.field("fault_ocp", &self.fault_ocp());
            d.field("fault_batovp", &self.fault_batovp());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargerStatus0A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargerStatus0A {{ ");
            defmt::write!(f, "fault_regn: {=bool}, ", &self.fault_regn());
            defmt::write!(f, "fault_ocp: {=bool}, ", &self.fault_ocp());
            defmt::write!(f, "fault_batovp: {=bool}, ", &self.fault_batovp());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargerStatus0A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargerStatus0A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargerStatus0A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargerStatus0A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargerStatus0A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargerStatus0A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargerStatus0A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargerStatus0B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargerStatus0B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargerStatus0B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `mode_stat` field of the register.
        ///
        /// MODE pin program status.
        pub fn mode_stat(&self) -> super::ModePinProgStatus {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `treg_stat` field of the register.
        ///
        /// Temperature regulation status.
        pub fn treg_stat(&self) -> super::TempRegulationStat {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `chg_tmr_stat` field of the register.
        ///
        /// Charge safety timer status.
        pub fn chg_tmr_stat(&self) -> super::ChrgSafetyTimerStat {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `chrg_stat` field of the register.
        ///
        /// Charge Cycle Status.
        pub fn chrg_stat(&self) -> super::ChrgCycleStat {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
    }
    impl From<[u8; 1]> for ChargerStatus0B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargerStatus0B> for [u8; 1] {
        fn from(val: ChargerStatus0B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargerStatus0B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargerStatus0B");
            d.field("mode_stat", &self.mode_stat());
            d.field("treg_stat", &self.treg_stat());
            d.field("chg_tmr_stat", &self.chg_tmr_stat());
            d.field("chrg_stat", &self.chrg_stat());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargerStatus0B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargerStatus0B {{ ");
            defmt::write!(f, "mode_stat: {}, ", &self.mode_stat());
            defmt::write!(f, "treg_stat: {}, ", &self.treg_stat());
            defmt::write!(f, "chg_tmr_stat: {}, ", &self.chg_tmr_stat());
            defmt::write!(f, "chrg_stat: {}, ", &self.chrg_stat());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargerStatus0B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargerStatus0B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargerStatus0B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargerStatus0B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargerStatus0B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargerStatus0B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargerStatus0B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcVbat {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcVbat {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcVbat {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_vbat` field of the register.
        ///
        /// VBAT ADC reading.
        pub fn adc_vbat(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcPsys {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcPsys {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcPsys {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_psys` field of the register.
        ///
        /// System Power PSYS ADC reading.
        pub fn adc_psys(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcCmpinTr {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcCmpinTr {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcCmpinTr {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_cmpin_tr` field of the register.
        ///
        /// CMPIN_TR pin voltage ADC reading.
        pub fn adc_cmpin_tr(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargerStatus1A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargerStatus1A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargerStatus1A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `fault_otg_uvp` field of the register.
        ///
        /// OTG_UVP fault detected.
        pub fn fault_otg_uvp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `fault_otg_ovp` field of the register.
        ///
        /// OTG_OVP fault detected.
        pub fn fault_otg_ovp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `fault_frc_conv_off` field of the register.
        ///
        /// OTG_OVP fault detected. Force converter off when independent comparator is triggered low effective.
        pub fn fault_frc_conv_off(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `fault_vsys_uvp` field of the register.
        ///
        /// VSYS_UVP fault status and clear. It is latched until a clear from host by writing this bit to 0.
        pub fn fault_vsys_uvp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `fault_sysovp` field of the register.
        ///
        /// SYSOVP fault status and Clear. When the SYSOVP occurs, this bit is set HIGH. As long as this bit is high, the converter is disabled. After the SYSOVP is removed, the user must write a 0 to this bit or unplug the adapter to clear the SYSOVP condition to enable the converter again.
        pub fn fault_sysovp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `fault_acoc` field of the register.
        ///
        /// ACOC fault detected.
        pub fn fault_acoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `fault_batdoc` field of the register.
        ///
        /// BATDOC fault detected.
        pub fn fault_batdoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `fault_acov` field of the register.
        ///
        /// ACOV fault detected.
        pub fn fault_acov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `fault_vsys_uvp` field of the register.
        ///
        /// VSYS_UVP fault status and clear. It is latched until a clear from host by writing this bit to 0.
        pub fn set_fault_vsys_uvp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `fault_sysovp` field of the register.
        ///
        /// SYSOVP fault status and Clear. When the SYSOVP occurs, this bit is set HIGH. As long as this bit is high, the converter is disabled. After the SYSOVP is removed, the user must write a 0 to this bit or unplug the adapter to clear the SYSOVP condition to enable the converter again.
        pub fn set_fault_sysovp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargerStatus1A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargerStatus1A> for [u8; 1] {
        fn from(val: ChargerStatus1A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargerStatus1A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargerStatus1A");
            d.field("fault_otg_uvp", &self.fault_otg_uvp());
            d.field("fault_otg_ovp", &self.fault_otg_ovp());
            d.field("fault_frc_conv_off", &self.fault_frc_conv_off());
            d.field("fault_vsys_uvp", &self.fault_vsys_uvp());
            d.field("fault_sysovp", &self.fault_sysovp());
            d.field("fault_acoc", &self.fault_acoc());
            d.field("fault_batdoc", &self.fault_batdoc());
            d.field("fault_acov", &self.fault_acov());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargerStatus1A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargerStatus1A {{ ");
            defmt::write!(f, "fault_otg_uvp: {=bool}, ", &self.fault_otg_uvp());
            defmt::write!(f, "fault_otg_ovp: {=bool}, ", &self.fault_otg_ovp());
            defmt::write!(f, "fault_frc_conv_off: {=bool}, ", &self.fault_frc_conv_off());
            defmt::write!(f, "fault_vsys_uvp: {=bool}, ", &self.fault_vsys_uvp());
            defmt::write!(f, "fault_sysovp: {=bool}, ", &self.fault_sysovp());
            defmt::write!(f, "fault_acoc: {=bool}, ", &self.fault_acoc());
            defmt::write!(f, "fault_batdoc: {=bool}, ", &self.fault_batdoc());
            defmt::write!(f, "fault_acov: {=bool}, ", &self.fault_acov());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargerStatus1A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargerStatus1A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargerStatus1A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargerStatus1A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargerStatus1A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargerStatus1A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargerStatus1A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargerStatus1B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargerStatus1B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargerStatus1B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `in_otg` field of the register.
        ///
        /// In OTG?
        pub fn in_otg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `fault_batcoc` field of the register.
        ///
        /// BATCOC fault detected.
        pub fn fault_batcoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `fault_sc_vbusacp` field of the register.
        ///
        /// VBUSACP fault detected.
        pub fn fault_sc_vbusacp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `in_iin_dpm` field of the register.
        ///
        /// In IIN_DPM or current regulation during OTG mode?
        pub fn in_iin_dpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `in_vindpm` field of the register.
        ///
        /// In VINDPM or boltage regulation during OTG mode?
        pub fn in_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `in_vap` field of the register.
        ///
        /// VAP (Vmin Active Protection) enabled?
        pub fn in_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `ico_done` field of the register.
        ///
        /// After the ICO routine is successfully executed, the bit goes 1.
        pub fn ico_done(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `stat_ac` field of the register.
        ///
        /// Input source status, STAT_AC is active as long as valid VBUS source exist.
        pub fn stat_ac(&self) -> super::InputSrcStat {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
    }
    impl From<[u8; 1]> for ChargerStatus1B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargerStatus1B> for [u8; 1] {
        fn from(val: ChargerStatus1B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargerStatus1B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargerStatus1B");
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
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargerStatus1B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargerStatus1B {{ ");
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
    impl core::ops::BitAnd for ChargerStatus1B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargerStatus1B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargerStatus1B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargerStatus1B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargerStatus1B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargerStatus1B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargerStatus1B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotStatusRegA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotStatusRegA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotStatusRegA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `stat_adapter_removal` field of the register.
        ///
        /// Adapter removed?
        pub fn stat_adapter_removal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `stat_battery_removal` field of the register.
        ///
        /// Battery removed?
        pub fn stat_battery_removal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `stat_vsys` field of the register.
        ///
        /// VSYS status triggered?
        pub fn stat_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `stat_idchg_1` field of the register.
        ///
        /// IDCHG1 status triggered?
        pub fn stat_idchg_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `stat_inom` field of the register.
        ///
        /// INOM status triggered?
        pub fn stat_inom(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `stat_icrit` field of the register.
        ///
        /// ICRIT status triggered?
        pub fn stat_icrit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `stat_comp` field of the register.
        ///
        /// COMP status triggered?
        pub fn stat_comp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `stat_vindpm` field of the register.
        ///
        /// PROCHOT Profile VINDPM status bit, once triggered 1b, PROCHOT pin is low until host writes this status bit to 0b when PP_VINDPM = 1b.
        pub fn stat_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `stat_vindpm` field of the register.
        ///
        /// PROCHOT Profile VINDPM status bit, once triggered 1b, PROCHOT pin is low until host writes this status bit to 0b when PP_VINDPM = 1b.
        pub fn set_stat_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotStatusRegA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotStatusRegA> for [u8; 1] {
        fn from(val: ProchotStatusRegA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotStatusRegA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotStatusRegA");
            d.field("stat_adapter_removal", &self.stat_adapter_removal());
            d.field("stat_battery_removal", &self.stat_battery_removal());
            d.field("stat_vsys", &self.stat_vsys());
            d.field("stat_idchg_1", &self.stat_idchg_1());
            d.field("stat_inom", &self.stat_inom());
            d.field("stat_icrit", &self.stat_icrit());
            d.field("stat_comp", &self.stat_comp());
            d.field("stat_vindpm", &self.stat_vindpm());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotStatusRegA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotStatusRegA {{ ");
            defmt::write!(f, "stat_adapter_removal: {=bool}, ", &self.stat_adapter_removal());
            defmt::write!(f, "stat_battery_removal: {=bool}, ", &self.stat_battery_removal());
            defmt::write!(f, "stat_vsys: {=bool}, ", &self.stat_vsys());
            defmt::write!(f, "stat_idchg_1: {=bool}, ", &self.stat_idchg_1());
            defmt::write!(f, "stat_inom: {=bool}, ", &self.stat_inom());
            defmt::write!(f, "stat_icrit: {=bool}, ", &self.stat_icrit());
            defmt::write!(f, "stat_comp: {=bool}, ", &self.stat_comp());
            defmt::write!(f, "stat_vindpm: {=bool}, ", &self.stat_vindpm());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotStatusRegA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotStatusRegA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotStatusRegA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotStatusRegA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotStatusRegA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotStatusRegA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotStatusRegA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotStatusRegB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotStatusRegB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotStatusRegB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [56] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `stat_exit_vap` field of the register.
        ///
        /// PROCHOT_EXIT_VAP is active?
        pub fn stat_exit_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `stat_vap_fail` field of the register.
        ///
        /// In VAP failure?
        pub fn stat_vap_fail(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `tshut` field of the register.
        ///
        /// TSHUT triggered?
        pub fn tshut(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `prochot_clear` field of the register.
        ///
        /// PROCHOT Pulse Clear. Clear PROCHOT pulse when EN_PROCHOT_EXT=0b.
        pub fn prochot_clear(&self) -> super::ProchotClear {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `prochot_width` field of the register.
        ///
        /// PROCHOT Pulse Width when EN_PROCHOT_EXT = 0b.
        pub fn prochot_width(&self) -> super::ProchotPulseWidth {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_prochot_ext` field of the register.
        ///
        /// PROCHOT Pulse Extension Enable. When pulse extension is enabled, keep the PROCHOT pin voltage LOW until host writes PROCHOT_CLEAR= 0b.
        pub fn en_prochot_ext(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Write the `stat_exit_vap` field of the register.
        ///
        /// PROCHOT_EXIT_VAP is active?
        pub fn set_stat_exit_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `stat_vap_fail` field of the register.
        ///
        /// In VAP failure?
        pub fn set_stat_vap_fail(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `prochot_clear` field of the register.
        ///
        /// PROCHOT Pulse Clear. Clear PROCHOT pulse when EN_PROCHOT_EXT=0b.
        pub fn set_prochot_clear(&mut self, value: super::ProchotClear) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `prochot_width` field of the register.
        ///
        /// PROCHOT Pulse Width when EN_PROCHOT_EXT = 0b.
        pub fn set_prochot_width(&mut self, value: super::ProchotPulseWidth) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 6, &mut self.bits) };
        }
        ///Write the `en_prochot_ext` field of the register.
        ///
        /// PROCHOT Pulse Extension Enable. When pulse extension is enabled, keep the PROCHOT pin voltage LOW until host writes PROCHOT_CLEAR= 0b.
        pub fn set_en_prochot_ext(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotStatusRegB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotStatusRegB> for [u8; 1] {
        fn from(val: ProchotStatusRegB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotStatusRegB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotStatusRegB");
            d.field("stat_exit_vap", &self.stat_exit_vap());
            d.field("stat_vap_fail", &self.stat_vap_fail());
            d.field("tshut", &self.tshut());
            d.field("prochot_clear", &self.prochot_clear());
            d.field("prochot_width", &self.prochot_width());
            d.field("en_prochot_ext", &self.en_prochot_ext());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotStatusRegB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotStatusRegB {{ ");
            defmt::write!(f, "stat_exit_vap: {=bool}, ", &self.stat_exit_vap());
            defmt::write!(f, "stat_vap_fail: {=bool}, ", &self.stat_vap_fail());
            defmt::write!(f, "tshut: {=bool}, ", &self.tshut());
            defmt::write!(f, "prochot_clear: {}, ", &self.prochot_clear());
            defmt::write!(f, "prochot_width: {}, ", &self.prochot_width());
            defmt::write!(f, "en_prochot_ext: {=bool}, ", &self.en_prochot_ext());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotStatusRegB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotStatusRegB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotStatusRegB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotStatusRegB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotStatusRegB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotStatusRegB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotStatusRegB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IinDpm {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for IinDpm {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl IinDpm {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [32, 3] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `iin_host` field of the register.
        ///
        /// Input current setting with 10mΩ sense resistor.
        pub fn iin_host(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 2, 11) };
            raw
        }
        ///Write the `iin_host` field of the register.
        ///
        /// Input current setting with 10mΩ sense resistor.
        pub fn set_iin_host(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 2, 11, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcVbus {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcVbus {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcVbus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_vbus` field of the register.
        ///
        /// VBUS ADC reading.
        pub fn adc_vbus(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcIbat {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcIbat {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcIbat {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_ibat` field of the register.
        ///
        /// IBAT ADC reading with 5mΩ sense resistor. Note the charger only measures discharging current (negative voltage) under battery only or OTG modes, and only measure charging current(positive voltage) when valid adapter is plugged in.
        pub fn adc_ibat(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcIin {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcIin {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcIin {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_iin` field of the register.
        ///
        /// IIN ADC reading with 10mΩ sense resistor. Current flowing from the adapter to the converter (like in forward mode) is represented as positive and current flowing to the adapter (like in OTG mode) is negative.
        pub fn adc_iin(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcVsys {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcVsys {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcVsys {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_vsys` field of the register.
        ///
        /// VSYS ADC reading.
        pub fn adc_vsys(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ManufactureId {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ManufactureId {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ManufactureId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [64] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `manufacture_id` field of the register.
        ///
        /// Manufacture ID.
        pub fn manufacture_id(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `manufacture_id` field of the register.
        ///
        /// Manufacture ID.
        pub fn set_manufacture_id(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DeviceId {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DeviceId {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DeviceId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [9] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `device_id` field of the register.
        ///
        /// Device ID.
        pub fn device_id(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `device_id` field of the register.
        ///
        /// Device ID.
        pub fn set_device_id(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
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
    #[cfg(feature = "defmt-03")]
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption1A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption1A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption1A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [1] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_sc_vbusacp` field of the register.
        ///
        /// SC_VBUSACP protection enable.
        pub fn en_sc_vbusacp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_ship_dchg` field of the register.
        ///
        /// Discharge SRN for Shipping Mode. Used to discharge SRN pin capacitor voltage which is necessary for battery gauge device shipping mode.
        pub fn en_ship_dchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_ptm` field of the register.
        ///
        /// PTM enable register bit, it will automatically reset to zero.
        pub fn en_ptm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `frc_conv_off` field of the register.
        ///
        /// Force Power Path Off.
        pub fn frc_conv_off(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `cmp_deg` field of the register.
        ///
        /// Independent comparator deglitch time, only applied to the falling edge of CMPOUT (HIGH to LOW).
        pub fn cmp_deg(&self) -> super::ComparatorDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `cmp_pol` field of the register.
        ///
        /// Independent Comparator output Polarity
        pub fn cmp_pol(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `sysovp_max` field of the register.
        ///
        /// Force SYSOVP protection threshold to 27V neglecting CELL_BATPRES pin configuration.
        pub fn sysovp_max(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_sc_vbusacp` field of the register.
        ///
        /// SC_VBUSACP protection enable.
        pub fn set_en_sc_vbusacp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_ship_dchg` field of the register.
        ///
        /// Discharge SRN for Shipping Mode. Used to discharge SRN pin capacitor voltage which is necessary for battery gauge device shipping mode.
        pub fn set_en_ship_dchg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_ptm` field of the register.
        ///
        /// PTM enable register bit, it will automatically reset to zero.
        pub fn set_en_ptm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `frc_conv_off` field of the register.
        ///
        /// Force Power Path Off.
        pub fn set_frc_conv_off(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `cmp_deg` field of the register.
        ///
        /// Independent comparator deglitch time, only applied to the falling edge of CMPOUT (HIGH to LOW).
        pub fn set_cmp_deg(&mut self, value: super::ComparatorDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 6, &mut self.bits) };
        }
        ///Write the `cmp_pol` field of the register.
        ///
        /// Independent Comparator output Polarity
        pub fn set_cmp_pol(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `sysovp_max` field of the register.
        ///
        /// Force SYSOVP protection threshold to 27V neglecting CELL_BATPRES pin configuration.
        pub fn set_sysovp_max(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption1A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption1A> for [u8; 1] {
        fn from(val: ChargeOption1A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption1A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption1A");
            d.field("en_sc_vbusacp", &self.en_sc_vbusacp());
            d.field("en_ship_dchg", &self.en_ship_dchg());
            d.field("en_ptm", &self.en_ptm());
            d.field("frc_conv_off", &self.frc_conv_off());
            d.field("cmp_deg", &self.cmp_deg());
            d.field("cmp_pol", &self.cmp_pol());
            d.field("sysovp_max", &self.sysovp_max());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption1A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption1A {{ ");
            defmt::write!(f, "en_sc_vbusacp: {=bool}, ", &self.en_sc_vbusacp());
            defmt::write!(f, "en_ship_dchg: {=bool}, ", &self.en_ship_dchg());
            defmt::write!(f, "en_ptm: {=bool}, ", &self.en_ptm());
            defmt::write!(f, "frc_conv_off: {=bool}, ", &self.frc_conv_off());
            defmt::write!(f, "cmp_deg: {}, ", &self.cmp_deg());
            defmt::write!(f, "cmp_pol: {=bool}, ", &self.cmp_pol());
            defmt::write!(f, "sysovp_max: {=bool}, ", &self.sysovp_max());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption1A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption1A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption1A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption1A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption1A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption1A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption1A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption1B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption1B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption1B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [50] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_otg_big_cap` field of the register.
        ///
        /// Enable OTG compensation for VBUS effective capacitance larger than 60uF.
        pub fn en_otg_big_cap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `psys_ratio` field of the register.
        ///
        /// PSYS Gain. Ratio of PSYS output current vs total input and battery power.
        pub fn psys_ratio(&self) -> super::PsysGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `rsns_rsr` field of the register.
        ///
        /// Charge sense resistor RSR. Not recommend to change this value during ICHG/IPRECHG/BATFET_CLAMP1/ BATFET_CLAMP2/BAT_SHORT regulation.
        pub fn rsns_rsr(&self) -> super::ChargeSenseResistorRsr {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `rsns_rac` field of the register.
        ///
        /// Input sense resistor RAC. Not recommend to change this value during IINDPM/IOTG regulation.
        pub fn rsns_rac(&self) -> super::InputSenseResistorRac {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `psys_config` field of the register.
        ///
        /// PSYS Enable and Definition Register. Enable PSYS sensing circuit and output buffer (whole PSYS circuit). In low power mode (EN_LWPWR=1b), PSYS sensing and buffer are always disabled regardless of this bit value.
        pub fn psys_config(&self) -> super::PsysEnable {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_lwpwr_cmp` field of the register.
        ///
        /// Independent Comparator Enable.
        pub fn en_lwpwr_cmp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_ibat` field of the register.
        ///
        /// IBAT Enable. In low power mode (EN_LWPWR=1b), IBAT buffer is always disabled regardless of this bit value.
        pub fn en_ibat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_otg_big_cap` field of the register.
        ///
        /// Enable OTG compensation for VBUS effective capacitance larger than 60uF.
        pub fn set_en_otg_big_cap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `psys_ratio` field of the register.
        ///
        /// PSYS Gain. Ratio of PSYS output current vs total input and battery power.
        pub fn set_psys_ratio(&mut self, value: super::PsysGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `rsns_rsr` field of the register.
        ///
        /// Charge sense resistor RSR. Not recommend to change this value during ICHG/IPRECHG/BATFET_CLAMP1/ BATFET_CLAMP2/BAT_SHORT regulation.
        pub fn set_rsns_rsr(&mut self, value: super::ChargeSenseResistorRsr) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `rsns_rac` field of the register.
        ///
        /// Input sense resistor RAC. Not recommend to change this value during IINDPM/IOTG regulation.
        pub fn set_rsns_rac(&mut self, value: super::InputSenseResistorRac) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `psys_config` field of the register.
        ///
        /// PSYS Enable and Definition Register. Enable PSYS sensing circuit and output buffer (whole PSYS circuit). In low power mode (EN_LWPWR=1b), PSYS sensing and buffer are always disabled regardless of this bit value.
        pub fn set_psys_config(&mut self, value: super::PsysEnable) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 6, &mut self.bits) };
        }
        ///Write the `en_lwpwr_cmp` field of the register.
        ///
        /// Independent Comparator Enable.
        pub fn set_en_lwpwr_cmp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_ibat` field of the register.
        ///
        /// IBAT Enable. In low power mode (EN_LWPWR=1b), IBAT buffer is always disabled regardless of this bit value.
        pub fn set_en_ibat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption1B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption1B> for [u8; 1] {
        fn from(val: ChargeOption1B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption1B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption1B");
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
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption1B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption1B {{ ");
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
    impl core::ops::BitAnd for ChargeOption1B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption1B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption1B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption1B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption1B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption1B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption1B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption2A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption2A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption2A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [183] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `batdoc_vth` field of the register.
        ///
        /// Set battery discharge overcurrent threshold as percentage of PROCHOT battery discharge current limit.
        pub fn batdoc_vth(&self) -> super::BatdocVth {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_batdoc` field of the register.
        ///
        /// Battery discharge overcurrent (BATDOC) protection enable.
        pub fn en_batdoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `acoc_vth` field of the register.
        ///
        /// ACOC Limit. Set ACOC threshold as percentage of ILIM2_VTH with current sensed from RAC.
        pub fn acoc_vth(&self) -> super::AcocLimit {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_acoc` field of the register.
        ///
        /// Input overcurrent (ACOC) protection enable.
        pub fn en_acoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `ocp_sw_1_x_high_range` field of the register.
        ///
        /// Over current protection threshold by sensing RAC resistor across voltage.
        pub fn ocp_sw_1_x_high_range(&self) -> super::OverCurrentThresholdRac {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ocp_sw_2_high_range` field of the register.
        ///
        /// Over current protection threshold by sensing Q4 Vds.
        pub fn ocp_sw_2_high_range(&self) -> super::OverCurrentThresholdQ4Vds {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_ichg_idchg` field of the register.
        ///
        /// IBAT pin monitor selection for discharge current and charge current.
        pub fn en_ichg_idchg(&self) -> super::IBatPinSelect {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_extilim` field of the register.
        ///
        /// Enable ILIM_HIZ pin to set input current limit.
        pub fn en_extilim(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `batdoc_vth` field of the register.
        ///
        /// Set battery discharge overcurrent threshold as percentage of PROCHOT battery discharge current limit.
        pub fn set_batdoc_vth(&mut self, value: super::BatdocVth) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_batdoc` field of the register.
        ///
        /// Battery discharge overcurrent (BATDOC) protection enable.
        pub fn set_en_batdoc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `acoc_vth` field of the register.
        ///
        /// ACOC Limit. Set ACOC threshold as percentage of ILIM2_VTH with current sensed from RAC.
        pub fn set_acoc_vth(&mut self, value: super::AcocLimit) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_acoc` field of the register.
        ///
        /// Input overcurrent (ACOC) protection enable.
        pub fn set_en_acoc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `ocp_sw_1_x_high_range` field of the register.
        ///
        /// Over current protection threshold by sensing RAC resistor across voltage.
        pub fn set_ocp_sw_1_x_high_range(&mut self, value: super::OverCurrentThresholdRac) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `ocp_sw_2_high_range` field of the register.
        ///
        /// Over current protection threshold by sensing Q4 Vds.
        pub fn set_ocp_sw_2_high_range(&mut self, value: super::OverCurrentThresholdQ4Vds) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_ichg_idchg` field of the register.
        ///
        /// IBAT pin monitor selection for discharge current and charge current.
        pub fn set_en_ichg_idchg(&mut self, value: super::IBatPinSelect) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_extilim` field of the register.
        ///
        /// Enable ILIM_HIZ pin to set input current limit.
        pub fn set_en_extilim(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption2A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption2A> for [u8; 1] {
        fn from(val: ChargeOption2A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption2A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption2A");
            d.field("batdoc_vth", &self.batdoc_vth());
            d.field("en_batdoc", &self.en_batdoc());
            d.field("acoc_vth", &self.acoc_vth());
            d.field("en_acoc", &self.en_acoc());
            d.field("ocp_sw_1_x_high_range", &self.ocp_sw_1_x_high_range());
            d.field("ocp_sw_2_high_range", &self.ocp_sw_2_high_range());
            d.field("en_ichg_idchg", &self.en_ichg_idchg());
            d.field("en_extilim", &self.en_extilim());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption2A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption2A {{ ");
            defmt::write!(f, "batdoc_vth: {}, ", &self.batdoc_vth());
            defmt::write!(f, "en_batdoc: {=bool}, ", &self.en_batdoc());
            defmt::write!(f, "acoc_vth: {}, ", &self.acoc_vth());
            defmt::write!(f, "en_acoc: {=bool}, ", &self.en_acoc());
            defmt::write!(f, "ocp_sw_1_x_high_range: {}, ", &self.ocp_sw_1_x_high_range());
            defmt::write!(f, "ocp_sw_2_high_range: {}, ", &self.ocp_sw_2_high_range());
            defmt::write!(f, "en_ichg_idchg: {}, ", &self.en_ichg_idchg());
            defmt::write!(f, "en_extilim: {=bool}, ", &self.en_extilim());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption2A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption2A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption2A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption2A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption2A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption2A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption2A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption2B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption2B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption2B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `pkpwr_tmax` field of the register.
        ///
        /// Peak power mode overload and relax cycle time.
        pub fn pkpwr_tmax(&self) -> super::PkpwrTmax {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `stat_pkpwr_relax` field of the register.
        ///
        /// Device is in relaxation cycle?
        pub fn stat_pkpwr_relax(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `stat_pkpwr_ovld` field of the register.
        ///
        /// Device is in overloading cycle?
        pub fn stat_pkpwr_ovld(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `en_pkpwr_vsys` field of the register.
        ///
        /// Enable Peak Power Mode triggered by system voltage under-shoot.
        pub fn en_pkpwr_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `en_pkpwr_iin_dp` field of the register.
        ///
        /// Enable Peak Power Mode triggered by input current overshoot.
        pub fn en_pkpwr_iin_dp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `pkpwr_tovld_deg` field of the register.
        ///
        /// Input Overload time in Peak Power Mode.
        pub fn pkpwr_tovld_deg(&self) -> super::PkpwrTovldDeg {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `pkpwr_tmax` field of the register.
        ///
        /// Peak power mode overload and relax cycle time.
        pub fn set_pkpwr_tmax(&mut self, value: super::PkpwrTmax) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `stat_pkpwr_relax` field of the register.
        ///
        /// Device is in relaxation cycle?
        pub fn set_stat_pkpwr_relax(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `stat_pkpwr_ovld` field of the register.
        ///
        /// Device is in overloading cycle?
        pub fn set_stat_pkpwr_ovld(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `en_pkpwr_vsys` field of the register.
        ///
        /// Enable Peak Power Mode triggered by system voltage under-shoot.
        pub fn set_en_pkpwr_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_pkpwr_iin_dp` field of the register.
        ///
        /// Enable Peak Power Mode triggered by input current overshoot.
        pub fn set_en_pkpwr_iin_dp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `pkpwr_tovld_deg` field of the register.
        ///
        /// Input Overload time in Peak Power Mode.
        pub fn set_pkpwr_tovld_deg(&mut self, value: super::PkpwrTovldDeg) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption2B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption2B> for [u8; 1] {
        fn from(val: ChargeOption2B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption2B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption2B");
            d.field("pkpwr_tmax", &self.pkpwr_tmax());
            d.field("stat_pkpwr_relax", &self.stat_pkpwr_relax());
            d.field("stat_pkpwr_ovld", &self.stat_pkpwr_ovld());
            d.field("en_pkpwr_vsys", &self.en_pkpwr_vsys());
            d.field("en_pkpwr_iin_dp", &self.en_pkpwr_iin_dp());
            d.field("pkpwr_tovld_deg", &self.pkpwr_tovld_deg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption2B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption2B {{ ");
            defmt::write!(f, "pkpwr_tmax: {}, ", &self.pkpwr_tmax());
            defmt::write!(f, "stat_pkpwr_relax: {=bool}, ", &self.stat_pkpwr_relax());
            defmt::write!(f, "stat_pkpwr_ovld: {=bool}, ", &self.stat_pkpwr_ovld());
            defmt::write!(f, "en_pkpwr_vsys: {=bool}, ", &self.en_pkpwr_vsys());
            defmt::write!(f, "en_pkpwr_iin_dp: {=bool}, ", &self.en_pkpwr_iin_dp());
            defmt::write!(f, "pkpwr_tovld_deg: {}, ", &self.pkpwr_tovld_deg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption2B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption2B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption2B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption2B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption2B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption2B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption2B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption3A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption3A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption3A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [52] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `psys_otg_idchg` field of the register.
        ///
        /// PSYS definition during OTG mode.
        pub fn psys_otg_idchg(&self) -> super::PsysOtg {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `batfetoff_hiz` field of the register.
        ///
        /// BATFET off during HIZ mode?
        pub fn batfetoff_hiz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `cmp_en` field of the register.
        ///
        /// Enable Independent Comparator with effective low.
        pub fn cmp_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `il_avg` field of the register.
        ///
        /// Inductor average current clamp.
        pub fn il_avg(&self) -> super::IlAvgClamp {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `otg_vap_mode` field of the register.
        ///
        /// The selection of the external EN_OTG pin control.
        pub fn otg_vap_mode(&self) -> super::EnOtgPinSelect {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pkpwr_tovld_deg` field of the register.
        ///
        /// Force turn off BATFET under battery only low power mode.
        pub fn pkpwr_tovld_deg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `psys_otg_idchg` field of the register.
        ///
        /// PSYS definition during OTG mode.
        pub fn set_psys_otg_idchg(&mut self, value: super::PsysOtg) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `batfetoff_hiz` field of the register.
        ///
        /// BATFET off during HIZ mode?
        pub fn set_batfetoff_hiz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `cmp_en` field of the register.
        ///
        /// Enable Independent Comparator with effective low.
        pub fn set_cmp_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `il_avg` field of the register.
        ///
        /// Inductor average current clamp.
        pub fn set_il_avg(&mut self, value: super::IlAvgClamp) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `otg_vap_mode` field of the register.
        ///
        /// The selection of the external EN_OTG pin control.
        pub fn set_otg_vap_mode(&mut self, value: super::EnOtgPinSelect) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `pkpwr_tovld_deg` field of the register.
        ///
        /// Force turn off BATFET under battery only low power mode.
        pub fn set_pkpwr_tovld_deg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption3A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption3A> for [u8; 1] {
        fn from(val: ChargeOption3A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption3A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption3A");
            d.field("psys_otg_idchg", &self.psys_otg_idchg());
            d.field("batfetoff_hiz", &self.batfetoff_hiz());
            d.field("cmp_en", &self.cmp_en());
            d.field("il_avg", &self.il_avg());
            d.field("otg_vap_mode", &self.otg_vap_mode());
            d.field("pkpwr_tovld_deg", &self.pkpwr_tovld_deg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption3A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption3A {{ ");
            defmt::write!(f, "psys_otg_idchg: {}, ", &self.psys_otg_idchg());
            defmt::write!(f, "batfetoff_hiz: {=bool}, ", &self.batfetoff_hiz());
            defmt::write!(f, "cmp_en: {=bool}, ", &self.cmp_en());
            defmt::write!(f, "il_avg: {}, ", &self.il_avg());
            defmt::write!(f, "otg_vap_mode: {}, ", &self.otg_vap_mode());
            defmt::write!(f, "pkpwr_tovld_deg: {=bool}, ", &self.pkpwr_tovld_deg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption3A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption3A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption3A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption3A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption3A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption3A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption3A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption3B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption3B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption3B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [5] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_vsys_min_soft_sr` field of the register.
        ///
        /// VSYS_MIN soft slew rate control for VSYS_MIN step up transition. Note for step down doesn't need the soft transition.
        pub fn en_vsys_min_soft_sr(&self) -> super::VsysMinSoftSlewRate {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_port_ctrl` field of the register.
        ///
        /// Enable BATFET control for dual port application.
        pub fn en_port_ctrl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `en_ico_mode` field of the register.
        ///
        /// Enable ICO Algorithm.
        pub fn en_ico_mode(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `en_otg` field of the register.
        ///
        /// OTG Mode Enable. Enable device in OTG mode when EN_OTG pin is HIGH.
        pub fn en_otg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `detect_vindpm` field of the register.
        ///
        /// Set VINDPM threshold based on VBUS measurement result minus 1.28V, Converter is disabled to measure VBUS.
        pub fn detect_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `reg_reset` field of the register.
        ///
        /// Factory Reset Registers.
        pub fn reg_reset(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_hiz` field of the register.
        ///
        /// Device Hi-Z Mode Enable. When the charger is in Hi-Z mode, the device draws minimal quiescent current. With VBUS above UVLO. REGN LDO stays on, and system powers from battery.
        pub fn en_hiz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_vsys_min_soft_sr` field of the register.
        ///
        /// VSYS_MIN soft slew rate control for VSYS_MIN step up transition. Note for step down doesn't need the soft transition.
        pub fn set_en_vsys_min_soft_sr(&mut self, value: super::VsysMinSoftSlewRate) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `en_port_ctrl` field of the register.
        ///
        /// Enable BATFET control for dual port application.
        pub fn set_en_port_ctrl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_ico_mode` field of the register.
        ///
        /// Enable ICO Algorithm.
        pub fn set_en_ico_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `en_otg` field of the register.
        ///
        /// OTG Mode Enable. Enable device in OTG mode when EN_OTG pin is HIGH.
        pub fn set_en_otg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `detect_vindpm` field of the register.
        ///
        /// Set VINDPM threshold based on VBUS measurement result minus 1.28V, Converter is disabled to measure VBUS.
        pub fn set_detect_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `reg_reset` field of the register.
        ///
        /// Factory Reset Registers.
        pub fn set_reg_reset(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_hiz` field of the register.
        ///
        /// Device Hi-Z Mode Enable. When the charger is in Hi-Z mode, the device draws minimal quiescent current. With VBUS above UVLO. REGN LDO stays on, and system powers from battery.
        pub fn set_en_hiz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption3B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption3B> for [u8; 1] {
        fn from(val: ChargeOption3B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption3B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption3B");
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
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption3B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption3B {{ ");
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
    impl core::ops::BitAnd for ChargeOption3B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption3B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption3B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption3B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption3B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption3B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption3B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption0A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotOption0A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotOption0A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [57] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `lower_prochot_vindpm` field of the register.
        ///
        /// Enable lower threshold of PROCHOT_VINDPM comparator.
        pub fn lower_prochot_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `inom_deg` field of the register.
        ///
        /// INOM deglitch time.
        pub fn inom_deg(&self) -> super::InomDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vsys_th_1` field of the register.
        ///
        /// VSYS threshold to trigger discharging VBUS in VAP mode.
        pub fn vsys_th_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Write the `lower_prochot_vindpm` field of the register.
        ///
        /// Enable lower threshold of PROCHOT_VINDPM comparator.
        pub fn set_lower_prochot_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `inom_deg` field of the register.
        ///
        /// INOM deglitch time.
        pub fn set_inom_deg(&mut self, value: super::InomDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `vsys_th_1` field of the register.
        ///
        /// VSYS threshold to trigger discharging VBUS in VAP mode.
        pub fn set_vsys_th_1(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotOption0A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption0A> for [u8; 1] {
        fn from(val: ProchotOption0A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption0A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption0A");
            d.field("lower_prochot_vindpm", &self.lower_prochot_vindpm());
            d.field("inom_deg", &self.inom_deg());
            d.field("vsys_th_1", &self.vsys_th_1());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption0A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption0A {{ ");
            defmt::write!(f, "lower_prochot_vindpm: {=bool}, ", &self.lower_prochot_vindpm());
            defmt::write!(f, "inom_deg: {}, ", &self.inom_deg());
            defmt::write!(f, "vsys_th_1: {=u8}, ", &self.vsys_th_1());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption0A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption0A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption0A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption0A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption0A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption0A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption0A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption0B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotOption0B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotOption0B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [74] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `prochot_vindpm_80_90` field of the register.
        ///
        /// Lower threshold of the PROCHOT_VINDPM comparator. When LOWER_PROCHOT_VINDPM=1, the threshold of PROCHOT_VINDPM is determined by this setting.
        pub fn prochot_vindpm_80_90(&self) -> super::Threshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `icrit_deg` field of the register.
        ///
        /// ICRIT deglitch time to trigger PROCHOT.
        pub fn icrit_deg(&self) -> super::IcritDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ilim_2_vth` field of the register.
        ///
        /// ILIM2 Threshold.
        pub fn ilim_2_vth(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 8) };
            raw
        }
        ///Write the `prochot_vindpm_80_90` field of the register.
        ///
        /// Lower threshold of the PROCHOT_VINDPM comparator. When LOWER_PROCHOT_VINDPM=1, the threshold of PROCHOT_VINDPM is determined by this setting.
        pub fn set_prochot_vindpm_80_90(&mut self, value: super::Threshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `icrit_deg` field of the register.
        ///
        /// ICRIT deglitch time to trigger PROCHOT.
        pub fn set_icrit_deg(&mut self, value: super::IcritDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 3, &mut self.bits) };
        }
        ///Write the `ilim_2_vth` field of the register.
        ///
        /// ILIM2 Threshold.
        pub fn set_ilim_2_vth(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotOption0B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption0B> for [u8; 1] {
        fn from(val: ProchotOption0B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption0B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption0B");
            d.field("prochot_vindpm_80_90", &self.prochot_vindpm_80_90());
            d.field("icrit_deg", &self.icrit_deg());
            d.field("ilim_2_vth", &self.ilim_2_vth());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption0B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption0B {{ ");
            defmt::write!(f, "prochot_vindpm_80_90: {}, ", &self.prochot_vindpm_80_90());
            defmt::write!(f, "icrit_deg: {}, ", &self.icrit_deg());
            defmt::write!(f, "ilim_2_vth: {=u8}, ", &self.ilim_2_vth());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption0B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption0B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption0B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption0B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption0B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption0B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption0B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption1A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotOption1A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotOption1A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [160] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `pp_acok` field of the register.
        ///
        /// Adapter removal PROCHOT profile enable.
        pub fn pp_acok(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `pp_batpres` field of the register.
        ///
        /// Battery removal PROCHOT profile enable.
        pub fn pp_batpres(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `pp_vsys` field of the register.
        ///
        /// VSYS PROCHOT profile enable.
        pub fn pp_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `pp_idchg_1` field of the register.
        ///
        /// IDCHG1 PROCHOT profile enable.
        pub fn pp_idchg_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `pp_inom` field of the register.
        ///
        /// INOM PROCHOT profile enable.
        pub fn pp_inom(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `pp_icrit` field of the register.
        ///
        /// ICRIT PROCHOT profile enable.
        pub fn pp_icrit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `pp_cmp` field of the register.
        ///
        /// COMP PROCHOT profile enable.
        pub fn pp_cmp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `pp_vindpm` field of the register.
        ///
        /// VINDPM PROCHOT profile enable.
        pub fn pp_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `pp_acok` field of the register.
        ///
        /// Adapter removal PROCHOT profile enable.
        pub fn set_pp_acok(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `pp_batpres` field of the register.
        ///
        /// Battery removal PROCHOT profile enable.
        pub fn set_pp_batpres(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `pp_vsys` field of the register.
        ///
        /// VSYS PROCHOT profile enable.
        pub fn set_pp_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `pp_idchg_1` field of the register.
        ///
        /// IDCHG1 PROCHOT profile enable.
        pub fn set_pp_idchg_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `pp_inom` field of the register.
        ///
        /// INOM PROCHOT profile enable.
        pub fn set_pp_inom(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `pp_icrit` field of the register.
        ///
        /// ICRIT PROCHOT profile enable.
        pub fn set_pp_icrit(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `pp_cmp` field of the register.
        ///
        /// COMP PROCHOT profile enable.
        pub fn set_pp_cmp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `pp_vindpm` field of the register.
        ///
        /// VINDPM PROCHOT profile enable.
        pub fn set_pp_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotOption1A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption1A> for [u8; 1] {
        fn from(val: ProchotOption1A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption1A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption1A");
            d.field("pp_acok", &self.pp_acok());
            d.field("pp_batpres", &self.pp_batpres());
            d.field("pp_vsys", &self.pp_vsys());
            d.field("pp_idchg_1", &self.pp_idchg_1());
            d.field("pp_inom", &self.pp_inom());
            d.field("pp_icrit", &self.pp_icrit());
            d.field("pp_cmp", &self.pp_cmp());
            d.field("pp_vindpm", &self.pp_vindpm());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption1A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption1A {{ ");
            defmt::write!(f, "pp_acok: {=bool}, ", &self.pp_acok());
            defmt::write!(f, "pp_batpres: {=bool}, ", &self.pp_batpres());
            defmt::write!(f, "pp_vsys: {=bool}, ", &self.pp_vsys());
            defmt::write!(f, "pp_idchg_1: {=bool}, ", &self.pp_idchg_1());
            defmt::write!(f, "pp_inom: {=bool}, ", &self.pp_inom());
            defmt::write!(f, "pp_icrit: {=bool}, ", &self.pp_icrit());
            defmt::write!(f, "pp_cmp: {=bool}, ", &self.pp_cmp());
            defmt::write!(f, "pp_vindpm: {=bool}, ", &self.pp_vindpm());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption1A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption1A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption1A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption1A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption1A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption1A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption1A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption1B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotOption1B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotOption1B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [65] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `idchg_deg_1` field of the register.
        ///
        /// IDCHG deglitch time.
        pub fn idchg_deg_1(&self) -> super::IdchgDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `idchg_th_1` field of the register.
        ///
        /// IDCHG level 1 Threshold.
        pub fn idchg_th_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Write the `idchg_deg_1` field of the register.
        ///
        /// IDCHG deglitch time.
        pub fn set_idchg_deg_1(&mut self, value: super::IdchgDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `idchg_th_1` field of the register.
        ///
        /// IDCHG level 1 Threshold.
        pub fn set_idchg_th_1(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotOption1B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption1B> for [u8; 1] {
        fn from(val: ProchotOption1B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption1B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption1B");
            d.field("idchg_deg_1", &self.idchg_deg_1());
            d.field("idchg_th_1", &self.idchg_th_1());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption1B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption1B {{ ");
            defmt::write!(f, "idchg_deg_1: {}, ", &self.idchg_deg_1());
            defmt::write!(f, "idchg_th_1: {=u8}, ", &self.idchg_th_1());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption1B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption1B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption1B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption1B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption1B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption1B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption1B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcOptionA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcOptionA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcOptionA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_adc_vbat` field of the register.
        ///
        /// Enable SRN pin Voltage ADC Channel.
        pub fn en_adc_vbat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_adc_vsys` field of the register.
        ///
        /// Enable VSYS pin Voltage ADC Channel.
        pub fn en_adc_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_adc_ibat` field of the register.
        ///
        /// Enable ICHG ADC Channel.
        pub fn en_adc_ibat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `en_adc_iin` field of the register.
        ///
        /// Enable IIN ADC Channel.
        pub fn en_adc_iin(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `en_adc_psys` field of the register.
        ///
        /// Enable PSYS pin Voltage ADC Channel.
        pub fn en_adc_psys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `en_adc_vbus` field of the register.
        ///
        /// Enable VBUS pin Voltage ADC Channel.
        pub fn en_adc_vbus(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_adc_cmpin` field of the register.
        ///
        /// Enable CMPIN_TR pin Voltage ADC Channel.
        pub fn en_adc_cmpin(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_adc_vbat` field of the register.
        ///
        /// Enable SRN pin Voltage ADC Channel.
        pub fn set_en_adc_vbat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_adc_vsys` field of the register.
        ///
        /// Enable VSYS pin Voltage ADC Channel.
        pub fn set_en_adc_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_adc_ibat` field of the register.
        ///
        /// Enable ICHG ADC Channel.
        pub fn set_en_adc_ibat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_adc_iin` field of the register.
        ///
        /// Enable IIN ADC Channel.
        pub fn set_en_adc_iin(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_adc_psys` field of the register.
        ///
        /// Enable PSYS pin Voltage ADC Channel.
        pub fn set_en_adc_psys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_adc_vbus` field of the register.
        ///
        /// Enable VBUS pin Voltage ADC Channel.
        pub fn set_en_adc_vbus(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_adc_cmpin` field of the register.
        ///
        /// Enable CMPIN_TR pin Voltage ADC Channel.
        pub fn set_en_adc_cmpin(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for AdcOptionA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcOptionA> for [u8; 1] {
        fn from(val: AdcOptionA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcOptionA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcOptionA");
            d.field("en_adc_vbat", &self.en_adc_vbat());
            d.field("en_adc_vsys", &self.en_adc_vsys());
            d.field("en_adc_ibat", &self.en_adc_ibat());
            d.field("en_adc_iin", &self.en_adc_iin());
            d.field("en_adc_psys", &self.en_adc_psys());
            d.field("en_adc_vbus", &self.en_adc_vbus());
            d.field("en_adc_cmpin", &self.en_adc_cmpin());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcOptionA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcOptionA {{ ");
            defmt::write!(f, "en_adc_vbat: {=bool}, ", &self.en_adc_vbat());
            defmt::write!(f, "en_adc_vsys: {=bool}, ", &self.en_adc_vsys());
            defmt::write!(f, "en_adc_ibat: {=bool}, ", &self.en_adc_ibat());
            defmt::write!(f, "en_adc_iin: {=bool}, ", &self.en_adc_iin());
            defmt::write!(f, "en_adc_psys: {=bool}, ", &self.en_adc_psys());
            defmt::write!(f, "en_adc_vbus: {=bool}, ", &self.en_adc_vbus());
            defmt::write!(f, "en_adc_cmpin: {=bool}, ", &self.en_adc_cmpin());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcOptionA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcOptionA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcOptionA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcOptionA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcOptionA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcOptionA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcOptionA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcOptionB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcOptionB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcOptionB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [144] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_avg_init` field of the register.
        ///
        /// ADC average initial value control.
        pub fn adc_avg_init(&self) -> super::AdcAvgInit {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `adc_avg` field of the register.
        ///
        /// ADC average control.
        pub fn adc_avg(&self) -> super::AdcAvgCtrl {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `adc_sample` field of the register.
        ///
        /// ADC sample resolution selection, each channel conversion time is also determined based on resolution.
        pub fn adc_sample(&self) -> super::AdcResolution {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `adc_en` field of the register.
        ///
        /// ADC conversion enable command.
        pub fn adc_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `adc_rate` field of the register.
        ///
        /// ADC conversion type selection. Typical conversion time is determined by resolution accuracy.
        pub fn adc_rate(&self) -> super::AdcRateSelect {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `adc_avg_init` field of the register.
        ///
        /// ADC average initial value control.
        pub fn set_adc_avg_init(&mut self, value: super::AdcAvgInit) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `adc_avg` field of the register.
        ///
        /// ADC average control.
        pub fn set_adc_avg(&mut self, value: super::AdcAvgCtrl) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `adc_sample` field of the register.
        ///
        /// ADC sample resolution selection, each channel conversion time is also determined based on resolution.
        pub fn set_adc_sample(&mut self, value: super::AdcResolution) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 6, &mut self.bits) };
        }
        ///Write the `adc_en` field of the register.
        ///
        /// ADC conversion enable command.
        pub fn set_adc_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `adc_rate` field of the register.
        ///
        /// ADC conversion type selection. Typical conversion time is determined by resolution accuracy.
        pub fn set_adc_rate(&mut self, value: super::AdcRateSelect) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for AdcOptionB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcOptionB> for [u8; 1] {
        fn from(val: AdcOptionB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcOptionB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcOptionB");
            d.field("adc_avg_init", &self.adc_avg_init());
            d.field("adc_avg", &self.adc_avg());
            d.field("adc_sample", &self.adc_sample());
            d.field("adc_en", &self.adc_en());
            d.field("adc_rate", &self.adc_rate());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcOptionB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcOptionB {{ ");
            defmt::write!(f, "adc_avg_init: {}, ", &self.adc_avg_init());
            defmt::write!(f, "adc_avg: {}, ", &self.adc_avg());
            defmt::write!(f, "adc_sample: {}, ", &self.adc_sample());
            defmt::write!(f, "adc_en: {=bool}, ", &self.adc_en());
            defmt::write!(f, "adc_rate: {}, ", &self.adc_rate());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcOptionB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcOptionB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcOptionB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcOptionB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcOptionB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcOptionB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcOptionB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption4A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption4A {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption4A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [72] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `stat_ptm` field of the register.
        ///
        /// PTM operation status active.
        pub fn stat_ptm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `stat_idchg_2` field of the register.
        ///
        /// IDCHG2 status triggered.
        pub fn stat_idchg_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `pp_idchg_2` field of the register.
        ///
        /// Enable IDCHG_TH2 PROCHOT Profile.
        pub fn pp_idchg_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `idchg_th_2` field of the register.
        ///
        /// Battery discharge current limit2 based on percentage of IDCHG_TH1.
        pub fn idchg_th_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 6) };
            raw
        }
        ///Read the `idchg_deg_2` field of the register.
        ///
        /// Battery discharge current limit 2 deglitch time.
        pub fn idchg_deg_2(&self) -> super::IdchgDeglitchTime2 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `stat_ptm` field of the register.
        ///
        /// PTM operation status active.
        pub fn set_stat_ptm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `stat_idchg_2` field of the register.
        ///
        /// IDCHG2 status triggered.
        pub fn set_stat_idchg_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `pp_idchg_2` field of the register.
        ///
        /// Enable IDCHG_TH2 PROCHOT Profile.
        pub fn set_pp_idchg_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `idchg_th_2` field of the register.
        ///
        /// Battery discharge current limit2 based on percentage of IDCHG_TH1.
        pub fn set_idchg_th_2(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 6, &mut self.bits) };
        }
        ///Write the `idchg_deg_2` field of the register.
        ///
        /// Battery discharge current limit 2 deglitch time.
        pub fn set_idchg_deg_2(&mut self, value: super::IdchgDeglitchTime2) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption4A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption4A> for [u8; 1] {
        fn from(val: ChargeOption4A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption4A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption4A");
            d.field("stat_ptm", &self.stat_ptm());
            d.field("stat_idchg_2", &self.stat_idchg_2());
            d.field("pp_idchg_2", &self.pp_idchg_2());
            d.field("idchg_th_2", &self.idchg_th_2());
            d.field("idchg_deg_2", &self.idchg_deg_2());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption4A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption4A {{ ");
            defmt::write!(f, "stat_ptm: {=bool}, ", &self.stat_ptm());
            defmt::write!(f, "stat_idchg_2: {=bool}, ", &self.stat_idchg_2());
            defmt::write!(f, "pp_idchg_2: {=bool}, ", &self.pp_idchg_2());
            defmt::write!(f, "idchg_th_2: {=u8}, ", &self.idchg_th_2());
            defmt::write!(f, "idchg_deg_2: {}, ", &self.idchg_deg_2());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption4A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption4A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption4A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption4A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption4A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption4A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption4A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption4B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption4B {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption4B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `stat_vbus_vap` field of the register.
        ///
        /// VBUS_VAP status triggered.
        pub fn stat_vbus_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `pp_vbus_vap` field of the register.
        ///
        /// Enable VBUS_VAP PROCHOT Profile.
        pub fn pp_vbus_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `vsys_uvp_no_hiccup` field of the register.
        ///
        /// Disable VSYS_UVP Hiccup mode operation.
        pub fn vsys_uvp_no_hiccup(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `en_dither` field of the register.
        ///
        /// Frequency Dither configuration.
        pub fn en_dither(&self) -> super::DitherConfig {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vsys_uvp` field of the register.
        ///
        /// VSYS Under Voltage Lock Out. After UVP is triggered the charger enters hiccup mode, and then the charger is latched off if the restart fails 7 times in 90s.
        pub fn vsys_uvp(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 8) };
            raw
        }
        ///Write the `stat_vbus_vap` field of the register.
        ///
        /// VBUS_VAP status triggered.
        pub fn set_stat_vbus_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `pp_vbus_vap` field of the register.
        ///
        /// Enable VBUS_VAP PROCHOT Profile.
        pub fn set_pp_vbus_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `vsys_uvp_no_hiccup` field of the register.
        ///
        /// Disable VSYS_UVP Hiccup mode operation.
        pub fn set_vsys_uvp_no_hiccup(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_dither` field of the register.
        ///
        /// Frequency Dither configuration.
        pub fn set_en_dither(&mut self, value: super::DitherConfig) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `vsys_uvp` field of the register.
        ///
        /// VSYS Under Voltage Lock Out. After UVP is triggered the charger enters hiccup mode, and then the charger is latched off if the restart fails 7 times in 90s.
        pub fn set_vsys_uvp(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption4B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption4B> for [u8; 1] {
        fn from(val: ChargeOption4B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption4B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption4B");
            d.field("stat_vbus_vap", &self.stat_vbus_vap());
            d.field("pp_vbus_vap", &self.pp_vbus_vap());
            d.field("vsys_uvp_no_hiccup", &self.vsys_uvp_no_hiccup());
            d.field("en_dither", &self.en_dither());
            d.field("vsys_uvp", &self.vsys_uvp());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption4B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption4B {{ ");
            defmt::write!(f, "stat_vbus_vap: {=bool}, ", &self.stat_vbus_vap());
            defmt::write!(f, "pp_vbus_vap: {=bool}, ", &self.pp_vbus_vap());
            defmt::write!(f, "vsys_uvp_no_hiccup: {=bool}, ", &self.vsys_uvp_no_hiccup());
            defmt::write!(f, "en_dither: {}, ", &self.en_dither());
            defmt::write!(f, "vsys_uvp: {=u8}, ", &self.vsys_uvp());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption4B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption4B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption4B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption4B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption4B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption4B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption4B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VminActiveProtectionA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for VminActiveProtectionA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl VminActiveProtectionA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [36] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_frs` field of the register.
        ///
        /// Fast Role Swap Feature Enable.
        pub fn en_frs(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_vsysth_2_follow_vsysth_1` field of the register.
        ///
        /// Enable internal VSYS_TH2 follow VSYS_TH1 setting neglecting register VSYS_TH2 setting.
        pub fn en_vsysth_2_follow_vsysth_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `vsys_th_2` field of the register.
        ///
        /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
        pub fn vsys_th_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Write the `en_frs` field of the register.
        ///
        /// Fast Role Swap Feature Enable.
        pub fn set_en_frs(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_vsysth_2_follow_vsysth_1` field of the register.
        ///
        /// Enable internal VSYS_TH2 follow VSYS_TH1 setting neglecting register VSYS_TH2 setting.
        pub fn set_en_vsysth_2_follow_vsysth_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `vsys_th_2` field of the register.
        ///
        /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
        pub fn set_vsys_th_2(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for VminActiveProtectionA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<VminActiveProtectionA> for [u8; 1] {
        fn from(val: VminActiveProtectionA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VminActiveProtectionA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VminActiveProtectionA");
            d.field("en_frs", &self.en_frs());
            d.field("en_vsysth_2_follow_vsysth_1", &self.en_vsysth_2_follow_vsysth_1());
            d.field("vsys_th_2", &self.vsys_th_2());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VminActiveProtectionA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VminActiveProtectionA {{ ");
            defmt::write!(f, "en_frs: {=bool}, ", &self.en_frs());
            defmt::write!(
                f,
                "en_vsysth_2_follow_vsysth_1: {=bool}, ",
                &self.en_vsysth_2_follow_vsysth_1()
            );
            defmt::write!(f, "vsys_th_2: {=u8}, ", &self.vsys_th_2());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for VminActiveProtectionA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VminActiveProtectionA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VminActiveProtectionA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VminActiveProtectionA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VminActiveProtectionA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VminActiveProtectionA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VminActiveProtectionA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VminActiveProtectionB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for VminActiveProtectionB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl VminActiveProtectionB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `dis_batovp_20_ma` field of the register.
        ///
        /// Disable BATOVP 20mA discharge current through VSYS pin.
        pub fn dis_batovp_20_ma(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `vbus_vap_th` field of the register.
        ///
        /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
        pub fn vbus_vap_th(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 8) };
            raw
        }
        ///Write the `dis_batovp_20_ma` field of the register.
        ///
        /// Disable BATOVP 20mA discharge current through VSYS pin.
        pub fn set_dis_batovp_20_ma(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `vbus_vap_th` field of the register.
        ///
        /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
        pub fn set_vbus_vap_th(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for VminActiveProtectionB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<VminActiveProtectionB> for [u8; 1] {
        fn from(val: VminActiveProtectionB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VminActiveProtectionB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VminActiveProtectionB");
            d.field("dis_batovp_20_ma", &self.dis_batovp_20_ma());
            d.field("vbus_vap_th", &self.vbus_vap_th());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VminActiveProtectionB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VminActiveProtectionB {{ ");
            defmt::write!(f, "dis_batovp_20_ma: {=bool}, ", &self.dis_batovp_20_ma());
            defmt::write!(f, "vbus_vap_th: {=u8}, ", &self.vbus_vap_th());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for VminActiveProtectionB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VminActiveProtectionB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VminActiveProtectionB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VminActiveProtectionB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VminActiveProtectionB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VminActiveProtectionB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VminActiveProtectionB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutotuneReadB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AutotuneReadB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AutotuneReadB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `autotune_b` field of the register.
        ///
        /// Phase B inductor time constant L(uH)/DCR(mΩ) value.
        pub fn autotune_b(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for AutotuneReadB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AutotuneReadB> for [u8; 1] {
        fn from(val: AutotuneReadB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AutotuneReadB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AutotuneReadB");
            d.field("autotune_b", &self.autotune_b());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AutotuneReadB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AutotuneReadB {{ ");
            defmt::write!(f, "autotune_b: {=u8}, ", &self.autotune_b());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AutotuneReadB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AutotuneReadB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AutotuneReadB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AutotuneReadB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AutotuneReadB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AutotuneReadB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AutotuneReadB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutotuneReadA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AutotuneReadA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AutotuneReadA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `autotune_a` field of the register.
        ///
        /// Phase A inductor time constant L(uH)/DCR(mΩ) value.
        pub fn autotune_a(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for AutotuneReadA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AutotuneReadA> for [u8; 1] {
        fn from(val: AutotuneReadA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AutotuneReadA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AutotuneReadA");
            d.field("autotune_a", &self.autotune_a());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AutotuneReadA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AutotuneReadA {{ ");
            defmt::write!(f, "autotune_a: {=u8}, ", &self.autotune_a());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AutotuneReadA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AutotuneReadA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AutotuneReadA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AutotuneReadA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AutotuneReadA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AutotuneReadA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AutotuneReadA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutotuneForceB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AutotuneForceB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AutotuneForceB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [200] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `force_autotune_b` field of the register.
        ///
        /// Force value for phase B inductor time constant L(uH)/DCR(mΩ).
        pub fn force_autotune_b(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `force_autotune_b` field of the register.
        ///
        /// Force value for phase B inductor time constant L(uH)/DCR(mΩ).
        pub fn set_force_autotune_b(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for AutotuneForceB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AutotuneForceB> for [u8; 1] {
        fn from(val: AutotuneForceB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AutotuneForceB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AutotuneForceB");
            d.field("force_autotune_b", &self.force_autotune_b());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AutotuneForceB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AutotuneForceB {{ ");
            defmt::write!(f, "force_autotune_b: {=u8}, ", &self.force_autotune_b());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AutotuneForceB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AutotuneForceB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AutotuneForceB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AutotuneForceB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AutotuneForceB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AutotuneForceB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AutotuneForceB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutotuneForceA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AutotuneForceA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AutotuneForceA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [200] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `force_autotune_a` field of the register.
        ///
        /// Force value for phase A inductor time constant L(uH)/DCR(mΩ).
        pub fn force_autotune_a(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `force_autotune_a` field of the register.
        ///
        /// Force value for phase A inductor time constant L(uH)/DCR(mΩ).
        pub fn set_force_autotune_a(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for AutotuneForceA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AutotuneForceA> for [u8; 1] {
        fn from(val: AutotuneForceA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AutotuneForceA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AutotuneForceA");
            d.field("force_autotune_a", &self.force_autotune_a());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AutotuneForceA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AutotuneForceA {{ ");
            defmt::write!(f, "force_autotune_a: {=u8}, ", &self.force_autotune_a());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AutotuneForceA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AutotuneForceA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AutotuneForceA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AutotuneForceA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AutotuneForceA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AutotuneForceA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AutotuneForceA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GmAdjustForceA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for GmAdjustForceA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GmAdjustForceA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [199] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `force_autotune_en` field of the register.
        ///
        /// Enable FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B effective for inductor DCR current sense.
        pub fn force_autotune_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `force_gm_adjust_en` field of the register.
        ///
        /// Enable FORCE_GM_ADJUST effective for inductor DCR current sense.
        pub fn force_gm_adjust_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `force_gm_adjust` field of the register.
        ///
        /// Force GM adjustment value for inductor DCR.
        pub fn force_gm_adjust(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Write the `force_autotune_en` field of the register.
        ///
        /// Enable FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B effective for inductor DCR current sense.
        pub fn set_force_autotune_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `force_gm_adjust_en` field of the register.
        ///
        /// Enable FORCE_GM_ADJUST effective for inductor DCR current sense.
        pub fn set_force_gm_adjust_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `force_gm_adjust` field of the register.
        ///
        /// Force GM adjustment value for inductor DCR.
        pub fn set_force_gm_adjust(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for GmAdjustForceA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<GmAdjustForceA> for [u8; 1] {
        fn from(val: GmAdjustForceA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for GmAdjustForceA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("GmAdjustForceA");
            d.field("force_autotune_en", &self.force_autotune_en());
            d.field("force_gm_adjust_en", &self.force_gm_adjust_en());
            d.field("force_gm_adjust", &self.force_gm_adjust());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for GmAdjustForceA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GmAdjustForceA {{ ");
            defmt::write!(f, "force_autotune_en: {=bool}, ", &self.force_autotune_en());
            defmt::write!(f, "force_gm_adjust_en: {=bool}, ", &self.force_gm_adjust_en());
            defmt::write!(f, "force_gm_adjust: {=u8}, ", &self.force_gm_adjust());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for GmAdjustForceA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for GmAdjustForceA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for GmAdjustForceA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for GmAdjustForceA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for GmAdjustForceA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for GmAdjustForceA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for GmAdjustForceA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GmAdjustForceB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for GmAdjustForceB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GmAdjustForceB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `force_update` field of the register.
        ///
        /// Update FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B, FORCE_GM_ADJUST value to be effective for inductor DCR current sense.
        pub fn force_update(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `gm_adjust` field of the register.
        ///
        /// Auto adaptive adjustment value for inductor DCR.
        pub fn gm_adjust(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Write the `force_update` field of the register.
        ///
        /// Update FORCE_AUTOTUNE_A, FORCE_AUTOTUNE_B, FORCE_GM_ADJUST value to be effective for inductor DCR current sense.
        pub fn set_force_update(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for GmAdjustForceB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<GmAdjustForceB> for [u8; 1] {
        fn from(val: GmAdjustForceB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for GmAdjustForceB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("GmAdjustForceB");
            d.field("force_update", &self.force_update());
            d.field("gm_adjust", &self.gm_adjust());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for GmAdjustForceB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GmAdjustForceB {{ ");
            defmt::write!(f, "force_update: {=bool}, ", &self.force_update());
            defmt::write!(f, "gm_adjust: {=u8}, ", &self.gm_adjust());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for GmAdjustForceB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for GmAdjustForceB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for GmAdjustForceB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for GmAdjustForceB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for GmAdjustForceB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for GmAdjustForceB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for GmAdjustForceB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VirtualControlA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for VirtualControlA {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl VirtualControlA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [19] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `wdtmr_adj` field of the register.
        ///
        /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command. If device does not receive a write on the CHARGE_VOLTAGE() or the CHARGE_CURRENT() within the watchdog time period, the charger will be suspended by setting the CHARGE_CURRENT() to 0 mA.
        pub fn wdtmr_adj(&self) -> super::WdtmrAdj {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `wd_rst` field of the register.
        ///
        /// Reset watch dog timer control.
        pub fn wd_rst(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `en_extilim` field of the register.
        ///
        /// Enable ILIM_HIZ pin to set input current limit.
        pub fn en_extilim(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `reg_reset` field of the register.
        ///
        /// Factory Reset Registers.
        pub fn reg_reset(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `wdtmr_adj` field of the register.
        ///
        /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command. If device does not receive a write on the CHARGE_VOLTAGE() or the CHARGE_CURRENT() within the watchdog time period, the charger will be suspended by setting the CHARGE_CURRENT() to 0 mA.
        pub fn set_wdtmr_adj(&mut self, value: super::WdtmrAdj) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `wd_rst` field of the register.
        ///
        /// Reset watch dog timer control.
        pub fn set_wd_rst(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_extilim` field of the register.
        ///
        /// Enable ILIM_HIZ pin to set input current limit.
        pub fn set_en_extilim(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `reg_reset` field of the register.
        ///
        /// Factory Reset Registers.
        pub fn set_reg_reset(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for VirtualControlA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<VirtualControlA> for [u8; 1] {
        fn from(val: VirtualControlA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VirtualControlA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VirtualControlA");
            d.field("wdtmr_adj", &self.wdtmr_adj());
            d.field("wd_rst", &self.wd_rst());
            d.field("en_extilim", &self.en_extilim());
            d.field("reg_reset", &self.reg_reset());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VirtualControlA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VirtualControlA {{ ");
            defmt::write!(f, "wdtmr_adj: {}, ", &self.wdtmr_adj());
            defmt::write!(f, "wd_rst: {=bool}, ", &self.wd_rst());
            defmt::write!(f, "en_extilim: {=bool}, ", &self.en_extilim());
            defmt::write!(f, "reg_reset: {=bool}, ", &self.reg_reset());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for VirtualControlA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VirtualControlA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VirtualControlA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VirtualControlA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VirtualControlA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VirtualControlA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VirtualControlA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VirtualControlB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for VirtualControlB {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl VirtualControlB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_otg` field of the register.
        ///
        /// OTG Mode Enable.
        pub fn en_otg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_auto_chg` field of the register.
        ///
        /// Automatic charge control(recharge and terminate battery charging automatically).
        pub fn en_auto_chg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_otg` field of the register.
        ///
        /// OTG Mode Enable.
        pub fn set_en_otg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_auto_chg` field of the register.
        ///
        /// Automatic charge control(recharge and terminate battery charging automatically).
        pub fn set_en_auto_chg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for VirtualControlB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<VirtualControlB> for [u8; 1] {
        fn from(val: VirtualControlB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VirtualControlB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VirtualControlB");
            d.field("en_otg", &self.en_otg());
            d.field("en_auto_chg", &self.en_auto_chg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VirtualControlB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VirtualControlB {{ ");
            defmt::write!(f, "en_otg: {=bool}, ", &self.en_otg());
            defmt::write!(f, "en_auto_chg: {=bool}, ", &self.en_auto_chg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for VirtualControlB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VirtualControlB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VirtualControlB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VirtualControlB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VirtualControlB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VirtualControlB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VirtualControlB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        ChargeOption0A(ChargeOption0A),
        ChargeOption0B(ChargeOption0B),
        ChargeCurrent(ChargeCurrent),
        ChargeVoltage(ChargeVoltage),
        IinHost(IinHost),
        Vindpm(Vindpm),
        OtgCurrent(OtgCurrent),
        OtgVoltage(OtgVoltage),
        VsysMin(VsysMin),
        ChargeProfileA(ChargeProfileA),
        ChargeProfileB(ChargeProfileB),
        GateDriveA(GateDriveA),
        GateDriveB(GateDriveB),
        ChargeOption5A(ChargeOption5A),
        ChargeOption5B(ChargeOption5B),
        AutoChargeA(AutoChargeA),
        AutoChargeB(AutoChargeB),
        ChargerStatus0A(ChargerStatus0A),
        ChargerStatus0B(ChargerStatus0B),
        AdcVbat(AdcVbat),
        AdcPsys(AdcPsys),
        AdcCmpinTr(AdcCmpinTr),
        ChargerStatus1A(ChargerStatus1A),
        ChargerStatus1B(ChargerStatus1B),
        ProchotStatusRegA(ProchotStatusRegA),
        ProchotStatusRegB(ProchotStatusRegB),
        IinDpm(IinDpm),
        AdcVbus(AdcVbus),
        AdcIbat(AdcIbat),
        AdcIin(AdcIin),
        AdcVsys(AdcVsys),
        ManufactureId(ManufactureId),
        DeviceId(DeviceId),
        ChargeOption1A(ChargeOption1A),
        ChargeOption1B(ChargeOption1B),
        ChargeOption2A(ChargeOption2A),
        ChargeOption2B(ChargeOption2B),
        ChargeOption3A(ChargeOption3A),
        ChargeOption3B(ChargeOption3B),
        ProchotOption0A(ProchotOption0A),
        ProchotOption0B(ProchotOption0B),
        ProchotOption1A(ProchotOption1A),
        ProchotOption1B(ProchotOption1B),
        AdcOptionA(AdcOptionA),
        AdcOptionB(AdcOptionB),
        ChargeOption4A(ChargeOption4A),
        ChargeOption4B(ChargeOption4B),
        VminActiveProtectionA(VminActiveProtectionA),
        VminActiveProtectionB(VminActiveProtectionB),
        AutotuneReadB(AutotuneReadB),
        AutotuneReadA(AutotuneReadA),
        AutotuneForceB(AutotuneForceB),
        AutotuneForceA(AutotuneForceA),
        GmAdjustForceA(GmAdjustForceA),
        GmAdjustForceB(GmAdjustForceB),
        VirtualControlA(VirtualControlA),
        VirtualControlB(VirtualControlB),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::ChargeOption0A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption0B(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::IinHost(val) => core::fmt::Debug::fmt(val, f),
                Self::Vindpm(val) => core::fmt::Debug::fmt(val, f),
                Self::OtgCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::OtgVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::VsysMin(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeProfileA(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeProfileB(val) => core::fmt::Debug::fmt(val, f),
                Self::GateDriveA(val) => core::fmt::Debug::fmt(val, f),
                Self::GateDriveB(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption5A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption5B(val) => core::fmt::Debug::fmt(val, f),
                Self::AutoChargeA(val) => core::fmt::Debug::fmt(val, f),
                Self::AutoChargeB(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargerStatus0A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargerStatus0B(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcVbat(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcPsys(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcCmpinTr(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargerStatus1A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargerStatus1B(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotStatusRegA(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotStatusRegB(val) => core::fmt::Debug::fmt(val, f),
                Self::IinDpm(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcVbus(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcIbat(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcIin(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcVsys(val) => core::fmt::Debug::fmt(val, f),
                Self::ManufactureId(val) => core::fmt::Debug::fmt(val, f),
                Self::DeviceId(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption1A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption1B(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption2A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption2B(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption3A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption3B(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption0A(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption0B(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption1A(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption1B(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcOptionA(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcOptionB(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption4A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption4B(val) => core::fmt::Debug::fmt(val, f),
                Self::VminActiveProtectionA(val) => core::fmt::Debug::fmt(val, f),
                Self::VminActiveProtectionB(val) => core::fmt::Debug::fmt(val, f),
                Self::AutotuneReadB(val) => core::fmt::Debug::fmt(val, f),
                Self::AutotuneReadA(val) => core::fmt::Debug::fmt(val, f),
                Self::AutotuneForceB(val) => core::fmt::Debug::fmt(val, f),
                Self::AutotuneForceA(val) => core::fmt::Debug::fmt(val, f),
                Self::GmAdjustForceA(val) => core::fmt::Debug::fmt(val, f),
                Self::GmAdjustForceB(val) => core::fmt::Debug::fmt(val, f),
                Self::VirtualControlA(val) => core::fmt::Debug::fmt(val, f),
                Self::VirtualControlB(val) => core::fmt::Debug::fmt(val, f),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::ChargeOption0A(val) => defmt::Format::format(val, f),
                Self::ChargeOption0B(val) => defmt::Format::format(val, f),
                Self::ChargeCurrent(val) => defmt::Format::format(val, f),
                Self::ChargeVoltage(val) => defmt::Format::format(val, f),
                Self::IinHost(val) => defmt::Format::format(val, f),
                Self::Vindpm(val) => defmt::Format::format(val, f),
                Self::OtgCurrent(val) => defmt::Format::format(val, f),
                Self::OtgVoltage(val) => defmt::Format::format(val, f),
                Self::VsysMin(val) => defmt::Format::format(val, f),
                Self::ChargeProfileA(val) => defmt::Format::format(val, f),
                Self::ChargeProfileB(val) => defmt::Format::format(val, f),
                Self::GateDriveA(val) => defmt::Format::format(val, f),
                Self::GateDriveB(val) => defmt::Format::format(val, f),
                Self::ChargeOption5A(val) => defmt::Format::format(val, f),
                Self::ChargeOption5B(val) => defmt::Format::format(val, f),
                Self::AutoChargeA(val) => defmt::Format::format(val, f),
                Self::AutoChargeB(val) => defmt::Format::format(val, f),
                Self::ChargerStatus0A(val) => defmt::Format::format(val, f),
                Self::ChargerStatus0B(val) => defmt::Format::format(val, f),
                Self::AdcVbat(val) => defmt::Format::format(val, f),
                Self::AdcPsys(val) => defmt::Format::format(val, f),
                Self::AdcCmpinTr(val) => defmt::Format::format(val, f),
                Self::ChargerStatus1A(val) => defmt::Format::format(val, f),
                Self::ChargerStatus1B(val) => defmt::Format::format(val, f),
                Self::ProchotStatusRegA(val) => defmt::Format::format(val, f),
                Self::ProchotStatusRegB(val) => defmt::Format::format(val, f),
                Self::IinDpm(val) => defmt::Format::format(val, f),
                Self::AdcVbus(val) => defmt::Format::format(val, f),
                Self::AdcIbat(val) => defmt::Format::format(val, f),
                Self::AdcIin(val) => defmt::Format::format(val, f),
                Self::AdcVsys(val) => defmt::Format::format(val, f),
                Self::ManufactureId(val) => defmt::Format::format(val, f),
                Self::DeviceId(val) => defmt::Format::format(val, f),
                Self::ChargeOption1A(val) => defmt::Format::format(val, f),
                Self::ChargeOption1B(val) => defmt::Format::format(val, f),
                Self::ChargeOption2A(val) => defmt::Format::format(val, f),
                Self::ChargeOption2B(val) => defmt::Format::format(val, f),
                Self::ChargeOption3A(val) => defmt::Format::format(val, f),
                Self::ChargeOption3B(val) => defmt::Format::format(val, f),
                Self::ProchotOption0A(val) => defmt::Format::format(val, f),
                Self::ProchotOption0B(val) => defmt::Format::format(val, f),
                Self::ProchotOption1A(val) => defmt::Format::format(val, f),
                Self::ProchotOption1B(val) => defmt::Format::format(val, f),
                Self::AdcOptionA(val) => defmt::Format::format(val, f),
                Self::AdcOptionB(val) => defmt::Format::format(val, f),
                Self::ChargeOption4A(val) => defmt::Format::format(val, f),
                Self::ChargeOption4B(val) => defmt::Format::format(val, f),
                Self::VminActiveProtectionA(val) => defmt::Format::format(val, f),
                Self::VminActiveProtectionB(val) => defmt::Format::format(val, f),
                Self::AutotuneReadB(val) => defmt::Format::format(val, f),
                Self::AutotuneReadA(val) => defmt::Format::format(val, f),
                Self::AutotuneForceB(val) => defmt::Format::format(val, f),
                Self::AutotuneForceA(val) => defmt::Format::format(val, f),
                Self::GmAdjustForceA(val) => defmt::Format::format(val, f),
                Self::GmAdjustForceB(val) => defmt::Format::format(val, f),
                Self::VirtualControlA(val) => defmt::Format::format(val, f),
                Self::VirtualControlB(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<ChargeOption0A> for FieldSetValue {
        fn from(val: ChargeOption0A) -> Self {
            Self::ChargeOption0A(val)
        }
    }
    impl From<ChargeOption0B> for FieldSetValue {
        fn from(val: ChargeOption0B) -> Self {
            Self::ChargeOption0B(val)
        }
    }
    impl From<ChargeCurrent> for FieldSetValue {
        fn from(val: ChargeCurrent) -> Self {
            Self::ChargeCurrent(val)
        }
    }
    impl From<ChargeVoltage> for FieldSetValue {
        fn from(val: ChargeVoltage) -> Self {
            Self::ChargeVoltage(val)
        }
    }
    impl From<IinHost> for FieldSetValue {
        fn from(val: IinHost) -> Self {
            Self::IinHost(val)
        }
    }
    impl From<Vindpm> for FieldSetValue {
        fn from(val: Vindpm) -> Self {
            Self::Vindpm(val)
        }
    }
    impl From<OtgCurrent> for FieldSetValue {
        fn from(val: OtgCurrent) -> Self {
            Self::OtgCurrent(val)
        }
    }
    impl From<OtgVoltage> for FieldSetValue {
        fn from(val: OtgVoltage) -> Self {
            Self::OtgVoltage(val)
        }
    }
    impl From<VsysMin> for FieldSetValue {
        fn from(val: VsysMin) -> Self {
            Self::VsysMin(val)
        }
    }
    impl From<ChargeProfileA> for FieldSetValue {
        fn from(val: ChargeProfileA) -> Self {
            Self::ChargeProfileA(val)
        }
    }
    impl From<ChargeProfileB> for FieldSetValue {
        fn from(val: ChargeProfileB) -> Self {
            Self::ChargeProfileB(val)
        }
    }
    impl From<GateDriveA> for FieldSetValue {
        fn from(val: GateDriveA) -> Self {
            Self::GateDriveA(val)
        }
    }
    impl From<GateDriveB> for FieldSetValue {
        fn from(val: GateDriveB) -> Self {
            Self::GateDriveB(val)
        }
    }
    impl From<ChargeOption5A> for FieldSetValue {
        fn from(val: ChargeOption5A) -> Self {
            Self::ChargeOption5A(val)
        }
    }
    impl From<ChargeOption5B> for FieldSetValue {
        fn from(val: ChargeOption5B) -> Self {
            Self::ChargeOption5B(val)
        }
    }
    impl From<AutoChargeA> for FieldSetValue {
        fn from(val: AutoChargeA) -> Self {
            Self::AutoChargeA(val)
        }
    }
    impl From<AutoChargeB> for FieldSetValue {
        fn from(val: AutoChargeB) -> Self {
            Self::AutoChargeB(val)
        }
    }
    impl From<ChargerStatus0A> for FieldSetValue {
        fn from(val: ChargerStatus0A) -> Self {
            Self::ChargerStatus0A(val)
        }
    }
    impl From<ChargerStatus0B> for FieldSetValue {
        fn from(val: ChargerStatus0B) -> Self {
            Self::ChargerStatus0B(val)
        }
    }
    impl From<AdcVbat> for FieldSetValue {
        fn from(val: AdcVbat) -> Self {
            Self::AdcVbat(val)
        }
    }
    impl From<AdcPsys> for FieldSetValue {
        fn from(val: AdcPsys) -> Self {
            Self::AdcPsys(val)
        }
    }
    impl From<AdcCmpinTr> for FieldSetValue {
        fn from(val: AdcCmpinTr) -> Self {
            Self::AdcCmpinTr(val)
        }
    }
    impl From<ChargerStatus1A> for FieldSetValue {
        fn from(val: ChargerStatus1A) -> Self {
            Self::ChargerStatus1A(val)
        }
    }
    impl From<ChargerStatus1B> for FieldSetValue {
        fn from(val: ChargerStatus1B) -> Self {
            Self::ChargerStatus1B(val)
        }
    }
    impl From<ProchotStatusRegA> for FieldSetValue {
        fn from(val: ProchotStatusRegA) -> Self {
            Self::ProchotStatusRegA(val)
        }
    }
    impl From<ProchotStatusRegB> for FieldSetValue {
        fn from(val: ProchotStatusRegB) -> Self {
            Self::ProchotStatusRegB(val)
        }
    }
    impl From<IinDpm> for FieldSetValue {
        fn from(val: IinDpm) -> Self {
            Self::IinDpm(val)
        }
    }
    impl From<AdcVbus> for FieldSetValue {
        fn from(val: AdcVbus) -> Self {
            Self::AdcVbus(val)
        }
    }
    impl From<AdcIbat> for FieldSetValue {
        fn from(val: AdcIbat) -> Self {
            Self::AdcIbat(val)
        }
    }
    impl From<AdcIin> for FieldSetValue {
        fn from(val: AdcIin) -> Self {
            Self::AdcIin(val)
        }
    }
    impl From<AdcVsys> for FieldSetValue {
        fn from(val: AdcVsys) -> Self {
            Self::AdcVsys(val)
        }
    }
    impl From<ManufactureId> for FieldSetValue {
        fn from(val: ManufactureId) -> Self {
            Self::ManufactureId(val)
        }
    }
    impl From<DeviceId> for FieldSetValue {
        fn from(val: DeviceId) -> Self {
            Self::DeviceId(val)
        }
    }
    impl From<ChargeOption1A> for FieldSetValue {
        fn from(val: ChargeOption1A) -> Self {
            Self::ChargeOption1A(val)
        }
    }
    impl From<ChargeOption1B> for FieldSetValue {
        fn from(val: ChargeOption1B) -> Self {
            Self::ChargeOption1B(val)
        }
    }
    impl From<ChargeOption2A> for FieldSetValue {
        fn from(val: ChargeOption2A) -> Self {
            Self::ChargeOption2A(val)
        }
    }
    impl From<ChargeOption2B> for FieldSetValue {
        fn from(val: ChargeOption2B) -> Self {
            Self::ChargeOption2B(val)
        }
    }
    impl From<ChargeOption3A> for FieldSetValue {
        fn from(val: ChargeOption3A) -> Self {
            Self::ChargeOption3A(val)
        }
    }
    impl From<ChargeOption3B> for FieldSetValue {
        fn from(val: ChargeOption3B) -> Self {
            Self::ChargeOption3B(val)
        }
    }
    impl From<ProchotOption0A> for FieldSetValue {
        fn from(val: ProchotOption0A) -> Self {
            Self::ProchotOption0A(val)
        }
    }
    impl From<ProchotOption0B> for FieldSetValue {
        fn from(val: ProchotOption0B) -> Self {
            Self::ProchotOption0B(val)
        }
    }
    impl From<ProchotOption1A> for FieldSetValue {
        fn from(val: ProchotOption1A) -> Self {
            Self::ProchotOption1A(val)
        }
    }
    impl From<ProchotOption1B> for FieldSetValue {
        fn from(val: ProchotOption1B) -> Self {
            Self::ProchotOption1B(val)
        }
    }
    impl From<AdcOptionA> for FieldSetValue {
        fn from(val: AdcOptionA) -> Self {
            Self::AdcOptionA(val)
        }
    }
    impl From<AdcOptionB> for FieldSetValue {
        fn from(val: AdcOptionB) -> Self {
            Self::AdcOptionB(val)
        }
    }
    impl From<ChargeOption4A> for FieldSetValue {
        fn from(val: ChargeOption4A) -> Self {
            Self::ChargeOption4A(val)
        }
    }
    impl From<ChargeOption4B> for FieldSetValue {
        fn from(val: ChargeOption4B) -> Self {
            Self::ChargeOption4B(val)
        }
    }
    impl From<VminActiveProtectionA> for FieldSetValue {
        fn from(val: VminActiveProtectionA) -> Self {
            Self::VminActiveProtectionA(val)
        }
    }
    impl From<VminActiveProtectionB> for FieldSetValue {
        fn from(val: VminActiveProtectionB) -> Self {
            Self::VminActiveProtectionB(val)
        }
    }
    impl From<AutotuneReadB> for FieldSetValue {
        fn from(val: AutotuneReadB) -> Self {
            Self::AutotuneReadB(val)
        }
    }
    impl From<AutotuneReadA> for FieldSetValue {
        fn from(val: AutotuneReadA) -> Self {
            Self::AutotuneReadA(val)
        }
    }
    impl From<AutotuneForceB> for FieldSetValue {
        fn from(val: AutotuneForceB) -> Self {
            Self::AutotuneForceB(val)
        }
    }
    impl From<AutotuneForceA> for FieldSetValue {
        fn from(val: AutotuneForceA) -> Self {
            Self::AutotuneForceA(val)
        }
    }
    impl From<GmAdjustForceA> for FieldSetValue {
        fn from(val: GmAdjustForceA) -> Self {
            Self::GmAdjustForceA(val)
        }
    }
    impl From<GmAdjustForceB> for FieldSetValue {
        fn from(val: GmAdjustForceB) -> Self {
            Self::GmAdjustForceB(val)
        }
    }
    impl From<VirtualControlA> for FieldSetValue {
        fn from(val: VirtualControlA) -> Self {
            Self::VirtualControlA(val)
        }
    }
    impl From<VirtualControlB> for FieldSetValue {
        fn from(val: VirtualControlB) -> Self {
            Self::VirtualControlB(val)
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(i8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum CmpinFuncSelect {
    Cmpmin = 0,
    Treg = 1,
}
impl core::convert::TryFrom<i8> for CmpinFuncSelect {
    type Error = ::device_driver::ConversionError<i8>;
    fn try_from(val: i8) -> Result<Self, Self::Error> {
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
impl From<CmpinFuncSelect> for i8 {
    fn from(val: CmpinFuncSelect) -> Self {
        match val {
            CmpinFuncSelect::Cmpmin => 0,
            CmpinFuncSelect::Treg => 1,
        }
    }
}
#[repr(i8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum WatchDogReset {
    Normal = 0,
    Reset = 1,
}
impl core::convert::TryFrom<i8> for WatchDogReset {
    type Error = ::device_driver::ConversionError<i8>;
    fn try_from(val: i8) -> Result<Self, Self::Error> {
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
impl From<WatchDogReset> for i8 {
    fn from(val: WatchDogReset) -> Self {
        match val {
            WatchDogReset::Normal => 0,
            WatchDogReset::Reset => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(i8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChgTmrSpeedCtrl {
    Normal = 0,
    HalfSpeed = 1,
}
impl core::convert::TryFrom<i8> for ChgTmrSpeedCtrl {
    type Error = ::device_driver::ConversionError<i8>;
    fn try_from(val: i8) -> Result<Self, Self::Error> {
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
impl From<ChgTmrSpeedCtrl> for i8 {
    fn from(val: ChgTmrSpeedCtrl) -> Self {
        match val {
            ChgTmrSpeedCtrl::Normal => 0,
            ChgTmrSpeedCtrl::HalfSpeed => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ModePinProgStatus {
    DualPhaseNormalComp600KHz = 0,
    DualPhaseNormalComp800KHz = 1,
    DualPhaseSlowComp600KHz = 2,
    DualPhaseSlowComp800KHz = 3,
    SinglePhaseNormalComp600KHz = 4,
    SinglePhaseNormalComp800KHz = 5,
    SinglePhaseSlowComp600KHz = 6,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChrgCycleStat {
    NotChrging = 0,
    Trickle = 1,
    PreChrg = 2,
    FastChrgCc = 3,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PsysGain {
    ZeroPoint25UAperW = 0,
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
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
