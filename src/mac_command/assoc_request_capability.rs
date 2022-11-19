#[doc = "Association request capabilities"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct R {
    bits: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct W {
    bits: u8,
}
impl core::ops::Deref for W {
    type Target = u8;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bits
    }
}
#[doc = "Set to one if the device is an FFD, otherwise it is an RFD."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceTypeA {
    #[doc = ""]
    FfdDevice = 1,
    #[doc = ""]
    RfdDevice = 0,
}
impl From<DeviceTypeA> for bool {
    #[inline(always)]
    fn from(variant: DeviceTypeA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DeviceType` reader - Set to one if the device is an FFD, otherwise it is an RFD."]
pub struct DeviceTypeR(crate::FieldReader<bool, DeviceTypeA>);
impl DeviceTypeR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DeviceTypeR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> DeviceTypeA {
        match self.bits {
            true => DeviceTypeA::FfdDevice,
            false => DeviceTypeA::RfdDevice,
        }
    }
    #[doc = "Checks if the value of the `DeviceType` field is `FfdDevice`"]
    #[inline(always)]
    pub fn is_ffd_device(&self) -> bool {
        **self == DeviceTypeA::FfdDevice
    }
    #[doc = "Checks if the value of the `DeviceType` field is `RfdDevice`"]
    #[inline(always)]
    pub fn is_rfd_device(&self) -> bool {
        **self == DeviceTypeA::RfdDevice
    }
}
impl core::ops::Deref for DeviceTypeR {
    type Target = crate::FieldReader<bool, DeviceTypeA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct DeviceTypeW<'a> {
    w: &'a mut W,
}
impl<'a> DeviceTypeW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: DeviceTypeA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `DeviceType` field to `FfdDevice`"]
    #[inline(always)]
    pub fn ffd_device(self) -> &'a mut W {
        self.variant(DeviceTypeA::FfdDevice)
    }
    #[doc = "Set the value of the `DeviceType` field to `RfdDevice`"]
    #[inline(always)]
    pub fn rfd_device(self) -> &'a mut W {
        self.variant(DeviceTypeA::RfdDevice)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Set to one if the device is connected to Alternating Current, otherwise it is a battery device."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerSourceA {
    #[doc = "The device is connected to alternative current mains."]
    MainsPowered = 1,
    #[doc = "The device is powered by a battery pack."]
    BatteryPowered = 0,
}
impl From<PowerSourceA> for bool {
    #[inline(always)]
    fn from(variant: PowerSourceA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PowerSource` reader - Set to one if the device is connected to Alternating Current, otherwise it is a battery device."]
pub struct PowerSourceR(crate::FieldReader<bool, PowerSourceA>);
impl PowerSourceR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PowerSourceR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> PowerSourceA {
        match self.bits {
            true => PowerSourceA::MainsPowered,
            false => PowerSourceA::BatteryPowered,
        }
    }
    #[doc = "Checks if the value of the `PowerSource` field is `MainsPowered`"]
    #[inline(always)]
    pub fn is_mains_powered(&self) -> bool {
        **self == PowerSourceA::MainsPowered
    }
    #[doc = "Checks if the value of the `PowerSource` field is `BatteryPowered`"]
    #[inline(always)]
    pub fn is_battery_powered(&self) -> bool {
        **self == PowerSourceA::BatteryPowered
    }
}
impl core::ops::Deref for PowerSourceR {
    type Target = crate::FieldReader<bool, PowerSourceA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct PowerSourceW<'a> {
    w: &'a mut W,
}
impl<'a> PowerSourceW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: PowerSourceA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `PowerSource` field to `MainsPowered`"]
    #[inline(always)]
    pub fn mains_powered(self) -> &'a mut W {
        self.variant(PowerSourceA::MainsPowered)
    }
    #[doc = "Set the value of the `PowerSource` field to `BatteryPowered`"]
    #[inline(always)]
    pub fn battery_powered(self) -> &'a mut W {
        self.variant(PowerSourceA::BatteryPowered)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "The device does not disable its receiver to conserve power during idle periods."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReceiverOnWhenIdleA {
    #[doc = "The device does not disable its receiver during idle periods."]
    ReceivesOnIdle = 1,
    #[doc = "The device disables its receiver to conserve power during idle periods."]
    DisablesOnIdle = 0,
}
impl From<ReceiverOnWhenIdleA> for bool {
    #[inline(always)]
    fn from(variant: ReceiverOnWhenIdleA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ReceiverOnWhenIdle` reader - The device does not disable its receiver to conserve power during idle periods."]
pub struct ReceiverOnWhenIdleR(crate::FieldReader<bool, ReceiverOnWhenIdleA>);
impl ReceiverOnWhenIdleR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ReceiverOnWhenIdleR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> ReceiverOnWhenIdleA {
        match self.bits {
            true => ReceiverOnWhenIdleA::ReceivesOnIdle,
            false => ReceiverOnWhenIdleA::DisablesOnIdle,
        }
    }
    #[doc = "Checks if the value of the `ReceiverOnWhenIdle` field is `ReceivesOnIdle`"]
    #[inline(always)]
    pub fn is_receives_on_idle(&self) -> bool {
        **self == ReceiverOnWhenIdleA::ReceivesOnIdle
    }
    #[doc = "Checks if the value of the `ReceiverOnWhenIdle` field is `DisablesOnIdle`"]
    #[inline(always)]
    pub fn is_disables_on_idle(&self) -> bool {
        **self == ReceiverOnWhenIdleA::DisablesOnIdle
    }
}
impl core::ops::Deref for ReceiverOnWhenIdleR {
    type Target = crate::FieldReader<bool, ReceiverOnWhenIdleA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct ReceiverOnWhenIdleW<'a> {
    w: &'a mut W,
}
impl<'a> ReceiverOnWhenIdleW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: ReceiverOnWhenIdleA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `ReceiverOnWhenIdle` field to `ReceivesOnIdle`"]
    #[inline(always)]
    pub fn receives_on_idle(self) -> &'a mut W {
        self.variant(ReceiverOnWhenIdleA::ReceivesOnIdle)
    }
    #[doc = "Set the value of the `ReceiverOnWhenIdle` field to `DisablesOnIdle`"]
    #[inline(always)]
    pub fn disables_on_idle(self) -> &'a mut W {
        self.variant(ReceiverOnWhenIdleA::DisablesOnIdle)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Set to one if the device requests fast association."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssociationTypeA {
    #[doc = ""]
    FastAssociation = 1,
    #[doc = ""]
    SlowAssociation = 0,
}
impl From<AssociationTypeA> for bool {
    #[inline(always)]
    fn from(variant: AssociationTypeA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AssociationType` reader - Set to one if the device requests fast association."]
pub struct AssociationTypeR(crate::FieldReader<bool, AssociationTypeA>);
impl AssociationTypeR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AssociationTypeR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> AssociationTypeA {
        match self.bits {
            true => AssociationTypeA::FastAssociation,
            false => AssociationTypeA::SlowAssociation,
        }
    }
    #[doc = "Checks if the value of the `AssociationType` field is `FastAssociation`"]
    #[inline(always)]
    pub fn is_fast_association(&self) -> bool {
        **self == AssociationTypeA::FastAssociation
    }
    #[doc = "Checks if the value of the `AssociationType` field is `SlowAssociation`"]
    #[inline(always)]
    pub fn is_slow_association(&self) -> bool {
        **self == AssociationTypeA::SlowAssociation
    }
}
impl core::ops::Deref for AssociationTypeR {
    type Target = crate::FieldReader<bool, AssociationTypeA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct AssociationTypeW<'a> {
    w: &'a mut W,
}
impl<'a> AssociationTypeW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: AssociationTypeA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `AssociationType` field to `FastAssociation`"]
    #[inline(always)]
    pub fn fast_association(self) -> &'a mut W {
        self.variant(AssociationTypeA::FastAssociation)
    }
    #[doc = "Set the value of the `AssociationType` field to `SlowAssociation`"]
    #[inline(always)]
    pub fn slow_association(self) -> &'a mut W {
        self.variant(AssociationTypeA::SlowAssociation)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Determines if the device is capable of sending and receiving cryptographically protected MAC frames."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SecurityCapabilityA {
    #[doc = "The device is capable of sending and receiving cryptographically protected MAC frames."]
    Secure = 1,
    #[doc = "The device is incapable of sending and receiving cryptographically protected MAC frames."]
    Unsecure = 0,
}
impl From<SecurityCapabilityA> for bool {
    #[inline(always)]
    fn from(variant: SecurityCapabilityA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecurityCapability` reader - Determines if the device is capable of sending and receiving cryptographically protected MAC frames."]
pub struct SecurityCapabilityR(crate::FieldReader<bool, SecurityCapabilityA>);
impl SecurityCapabilityR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SecurityCapabilityR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> SecurityCapabilityA {
        match self.bits {
            true => SecurityCapabilityA::Secure,
            false => SecurityCapabilityA::Unsecure,
        }
    }
    #[doc = "Checks if the value of the `SecurityCapability` field is `Secure`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == SecurityCapabilityA::Secure
    }
    #[doc = "Checks if the value of the `SecurityCapability` field is `Unsecure`"]
    #[inline(always)]
    pub fn is_unsecure(&self) -> bool {
        **self == SecurityCapabilityA::Unsecure
    }
}
impl core::ops::Deref for SecurityCapabilityR {
    type Target = crate::FieldReader<bool, SecurityCapabilityA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct SecurityCapabilityW<'a> {
    w: &'a mut W,
}
impl<'a> SecurityCapabilityW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: SecurityCapabilityA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `SecurityCapability` field to `Secure`"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(SecurityCapabilityA::Secure)
    }
    #[doc = "Set the value of the `SecurityCapability` field to `Unsecure`"]
    #[inline(always)]
    pub fn unsecure(self) -> &'a mut W {
        self.variant(SecurityCapabilityA::Unsecure)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Determines if the coordinator should allocate a short address as a result of the allocation procedure."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AllocateAddressA {
    #[doc = "The device wishes the coordinator to allocate a short address."]
    RequestAddress = 1,
    #[doc = "The device does not request the coordinator to allocate a short address."]
    NoRequest = 0,
}
impl From<AllocateAddressA> for bool {
    #[inline(always)]
    fn from(variant: AllocateAddressA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AllocateAddress` reader - Determines if the coordinator should allocate a short address as a result of the allocation procedure."]
pub struct AllocateAddressR(crate::FieldReader<bool, AllocateAddressA>);
impl AllocateAddressR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AllocateAddressR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> AllocateAddressA {
        match self.bits {
            true => AllocateAddressA::RequestAddress,
            false => AllocateAddressA::NoRequest,
        }
    }
    #[doc = "Checks if the value of the `AllocateAddress` field is `RequestAddress`"]
    #[inline(always)]
    pub fn is_request_address(&self) -> bool {
        **self == AllocateAddressA::RequestAddress
    }
    #[doc = "Checks if the value of the `AllocateAddress` field is `NoRequest`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        **self == AllocateAddressA::NoRequest
    }
}
impl core::ops::Deref for AllocateAddressR {
    type Target = crate::FieldReader<bool, AllocateAddressA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct AllocateAddressW<'a> {
    w: &'a mut W,
}
impl<'a> AllocateAddressW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: AllocateAddressA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `AllocateAddress` field to `RequestAddress`"]
    #[inline(always)]
    pub fn request_address(self) -> &'a mut W {
        self.variant(AllocateAddressA::RequestAddress)
    }
    #[doc = "Set the value of the `AllocateAddress` field to `NoRequest`"]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(AllocateAddressA::NoRequest)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        R { bits }
    }
    #[doc = "Read the `DeviceType` field."]
    #[inline(always)]
    pub fn device_type(&self) -> DeviceTypeR {
        DeviceTypeR::new((self.bits & 0x02) != 0)
    }
    #[doc = "Read the `PowerSource` field."]
    #[inline(always)]
    pub fn power_source(&self) -> PowerSourceR {
        PowerSourceR::new((self.bits & 0x04) != 0)
    }
    #[doc = "Read the `ReceiverOnWhenIdle` field."]
    #[inline(always)]
    pub fn receiver_on_when_idle(&self) -> ReceiverOnWhenIdleR {
        ReceiverOnWhenIdleR::new((self.bits & 0x08) != 0)
    }
    #[doc = "Read the `AssociationType` field."]
    #[inline(always)]
    pub fn association_type(&self) -> AssociationTypeR {
        AssociationTypeR::new((self.bits & 0x10) != 0)
    }
    #[doc = "Read the `SecurityCapability` field."]
    #[inline(always)]
    pub fn security_capability(&self) -> SecurityCapabilityR {
        SecurityCapabilityR::new((self.bits & 0x40) != 0)
    }
    #[doc = "Read the `AllocateAddress` field."]
    #[inline(always)]
    pub fn allocate_address(&self) -> AllocateAddressR {
        AllocateAddressR::new((self.bits & 0x80) != 0)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `DeviceType` field."]
    #[inline(always)]
    pub fn device_type(&mut self) -> DeviceTypeW {
        DeviceTypeW { w: self }
    }
    #[doc = "Set the `PowerSource` field."]
    #[inline(always)]
    pub fn power_source(&mut self) -> PowerSourceW {
        PowerSourceW { w: self }
    }
    #[doc = "Set the `ReceiverOnWhenIdle` field."]
    #[inline(always)]
    pub fn receiver_on_when_idle(&mut self) -> ReceiverOnWhenIdleW {
        ReceiverOnWhenIdleW { w: self }
    }
    #[doc = "Set the `AssociationType` field."]
    #[inline(always)]
    pub fn association_type(&mut self) -> AssociationTypeW {
        AssociationTypeW { w: self }
    }
    #[doc = "Set the `SecurityCapability` field."]
    #[inline(always)]
    pub fn security_capability(&mut self) -> SecurityCapabilityW {
        SecurityCapabilityW { w: self }
    }
    #[doc = "Set the `AllocateAddress` field."]
    #[inline(always)]
    pub fn allocate_address(&mut self) -> AllocateAddressW {
        AllocateAddressW { w: self }
    }
}
