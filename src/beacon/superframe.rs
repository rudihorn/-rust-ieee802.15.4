#[doc = "Superframe specification field."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct R {
    bits: u16,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct W {
    bits: u16,
}
impl core::ops::Deref for W {
    type Target = u16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bits
    }
}
#[doc = "Set if the frames transmitted are required to start before battery life extended periods."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BattLifeExtA {
    #[doc = "Battery life extension is not required."]
    BleNotSet = 0,
    #[doc = "Battery life extension is required and packets must be sent before macBattlifeExtPeriods full backoff periods afetr the IFS period following the beacon."]
    BleSet = 1,
}
impl From<BattLifeExtA> for bool {
    #[inline(always)]
    fn from(variant: BattLifeExtA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BattLifeExt` reader - Set if the frames transmitted are required to start before battery life extended periods."]
pub struct BattLifeExtR(crate::FieldReader<bool, BattLifeExtA>);
impl BattLifeExtR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BattLifeExtR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> BattLifeExtA {
        match self.bits {
            false => BattLifeExtA::BleNotSet,
            true => BattLifeExtA::BleSet,
        }
    }
    #[doc = "Checks if the value of the `BattLifeExt` field is `BleNotSet`"]
    #[inline(always)]
    pub fn is_ble_not_set(&self) -> bool {
        **self == BattLifeExtA::BleNotSet
    }
    #[doc = "Checks if the value of the `BattLifeExt` field is `BleSet`"]
    #[inline(always)]
    pub fn is_ble_set(&self) -> bool {
        **self == BattLifeExtA::BleSet
    }
}
impl core::ops::Deref for BattLifeExtR {
    type Target = crate::FieldReader<bool, BattLifeExtA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct BattLifeExtW<'a> {
    w: &'a mut W,
}
impl<'a> BattLifeExtW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: BattLifeExtA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `BattLifeExt` field to `BleNotSet`"]
    #[inline(always)]
    pub fn ble_not_set(self) -> &'a mut W {
        self.variant(BattLifeExtA::BleNotSet)
    }
    #[doc = "Set the value of the `BattLifeExt` field to `BleSet`"]
    #[inline(always)]
    pub fn ble_set(self) -> &'a mut W {
        self.variant(BattLifeExtA::BleSet)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Specifies if the sender is a PAN coordinator."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PanCoordinatorA {
    #[doc = "The transmitting device is not a PAN coordinator."]
    NotPanCoordinator = 0,
    #[doc = "The transmitting device is a PAN coordinator"]
    PanCoordinator = 1,
}
impl From<PanCoordinatorA> for bool {
    #[inline(always)]
    fn from(variant: PanCoordinatorA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PanCoordinator` reader - Specifies if the sender is a PAN coordinator."]
pub struct PanCoordinatorR(crate::FieldReader<bool, PanCoordinatorA>);
impl PanCoordinatorR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PanCoordinatorR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> PanCoordinatorA {
        match self.bits {
            false => PanCoordinatorA::NotPanCoordinator,
            true => PanCoordinatorA::PanCoordinator,
        }
    }
    #[doc = "Checks if the value of the `PanCoordinator` field is `NotPanCoordinator`"]
    #[inline(always)]
    pub fn is_not_pan_coordinator(&self) -> bool {
        **self == PanCoordinatorA::NotPanCoordinator
    }
    #[doc = "Checks if the value of the `PanCoordinator` field is `PanCoordinator`"]
    #[inline(always)]
    pub fn is_pan_coordinator(&self) -> bool {
        **self == PanCoordinatorA::PanCoordinator
    }
}
impl core::ops::Deref for PanCoordinatorR {
    type Target = crate::FieldReader<bool, PanCoordinatorA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct PanCoordinatorW<'a> {
    w: &'a mut W,
}
impl<'a> PanCoordinatorW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: PanCoordinatorA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `PanCoordinator` field to `NotPanCoordinator`"]
    #[inline(always)]
    pub fn not_pan_coordinator(self) -> &'a mut W {
        self.variant(PanCoordinatorA::NotPanCoordinator)
    }
    #[doc = "Set the value of the `PanCoordinator` field to `PanCoordinator`"]
    #[inline(always)]
    pub fn pan_coordinator(self) -> &'a mut W {
        self.variant(PanCoordinatorA::PanCoordinator)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Specifies if devices are permitted to join the PAN."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssociationPermitA {
    #[doc = "Devices are not permitted to associate with the PAN."]
    NotPermitted = 0,
    #[doc = "Devices are permitted to associate with the PAN."]
    Permitted = 1,
}
impl From<AssociationPermitA> for bool {
    #[inline(always)]
    fn from(variant: AssociationPermitA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AssociationPermit` reader - Specifies if devices are permitted to join the PAN."]
pub struct AssociationPermitR(crate::FieldReader<bool, AssociationPermitA>);
impl AssociationPermitR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AssociationPermitR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> AssociationPermitA {
        match self.bits {
            false => AssociationPermitA::NotPermitted,
            true => AssociationPermitA::Permitted,
        }
    }
    #[doc = "Checks if the value of the `AssociationPermit` field is `NotPermitted`"]
    #[inline(always)]
    pub fn is_not_permitted(&self) -> bool {
        **self == AssociationPermitA::NotPermitted
    }
    #[doc = "Checks if the value of the `AssociationPermit` field is `Permitted`"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        **self == AssociationPermitA::Permitted
    }
}
impl core::ops::Deref for AssociationPermitR {
    type Target = crate::FieldReader<bool, AssociationPermitA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct AssociationPermitW<'a> {
    w: &'a mut W,
}
impl<'a> AssociationPermitW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: AssociationPermitA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `AssociationPermit` field to `NotPermitted`"]
    #[inline(always)]
    pub fn not_permitted(self) -> &'a mut W {
        self.variant(AssociationPermitA::NotPermitted)
    }
    #[doc = "Set the value of the `AssociationPermit` field to `Permitted`"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut W {
        self.variant(AssociationPermitA::Permitted)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u16) -> Self {
        R { bits }
    }
    #[doc = "Read the `BeaconOrder` field."]
    #[inline(always)]
    pub fn beacon_order(&self) -> u8 {
        ((self.bits >> 0) & 0x0f) as u8
    }
    #[doc = "Read the `SuperframeOrder` field."]
    #[inline(always)]
    pub fn superframe_order(&self) -> u8 {
        ((self.bits >> 4) & 0x0f) as u8
    }
    #[doc = "Read the `FinalCapSlot` field."]
    #[inline(always)]
    pub fn final_cap_slot(&self) -> u8 {
        ((self.bits >> 8) & 0x0f) as u8
    }
    #[doc = "Read the `BattLifeExt` field."]
    #[inline(always)]
    pub fn batt_life_ext(&self) -> BattLifeExtR {
        BattLifeExtR::new((self.bits & 0x1000) != 0)
    }
    #[doc = "Read the `PanCoordinator` field."]
    #[inline(always)]
    pub fn pan_coordinator(&self) -> PanCoordinatorR {
        PanCoordinatorR::new((self.bits & 0x4000) != 0)
    }
    #[doc = "Read the `AssociationPermit` field."]
    #[inline(always)]
    pub fn association_permit(&self) -> AssociationPermitR {
        AssociationPermitR::new((self.bits & 0x8000) != 0)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u16) -> Self {
        W { bits }
    }
    #[doc = "Set the `BeaconOrder` field."]
    #[inline(always)]
    pub fn beacon_order(&mut self, value: u8) -> Self {
        let bits = (self.bits & !(0x0f << 0)) | ((value as u16 & 0x0f) << 0);
        Self { bits, ..*self }
    }
    #[doc = "Set the `SuperframeOrder` field."]
    #[inline(always)]
    pub fn superframe_order(&mut self, value: u8) -> Self {
        let bits = (self.bits & !(0x0f << 4)) | ((value as u16 & 0x0f) << 4);
        Self { bits, ..*self }
    }
    #[doc = "Set the `FinalCapSlot` field."]
    #[inline(always)]
    pub fn final_cap_slot(&mut self, value: u8) -> Self {
        let bits = (self.bits & !(0x0f << 8)) | ((value as u16 & 0x0f) << 8);
        Self { bits, ..*self }
    }
    #[doc = "Set the `BattLifeExt` field."]
    #[inline(always)]
    pub fn batt_life_ext(&mut self) -> BattLifeExtW {
        BattLifeExtW { w: self }
    }
    #[doc = "Set the `PanCoordinator` field."]
    #[inline(always)]
    pub fn pan_coordinator(&mut self) -> PanCoordinatorW {
        PanCoordinatorW { w: self }
    }
    #[doc = "Set the `AssociationPermit` field."]
    #[inline(always)]
    pub fn association_permit(&mut self) -> AssociationPermitW {
        AssociationPermitW { w: self }
    }
}
