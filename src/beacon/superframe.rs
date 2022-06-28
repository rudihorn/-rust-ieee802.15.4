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
#[doc = "This field contains information about the transmission interval of the beacon."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeaconOrderA {}
impl From<BeaconOrderA> for u8 {
    #[inline(always)]
    fn from(variant: BeaconOrderA) -> Self {
        variant as _
    }
}
#[doc = "Field `BeaconOrder` reader - This field contains information about the transmission interval of the beacon."]
pub struct BeaconOrderR(crate::FieldReader<u8, BeaconOrderA>);
impl BeaconOrderR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BeaconOrderR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> BeaconOrderA {
        match self.bits {
            _ => unreachable!(),
        }
    }
}
impl core::ops::Deref for BeaconOrderR {
    type Target = crate::FieldReader<u8, BeaconOrderA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct BeaconOrderW<'a> {
    w: &'a mut W,
}
impl<'a> BeaconOrderW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: BeaconOrderA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 0)) | ((value as u16 & 0x0f) << 0);
        self.w
    }
}
#[doc = "This field contains information about the transmission duration of the beacon."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuperframeOrderA {}
impl From<SuperframeOrderA> for u8 {
    #[inline(always)]
    fn from(variant: SuperframeOrderA) -> Self {
        variant as _
    }
}
#[doc = "Field `SuperframeOrder` reader - This field contains information about the transmission duration of the beacon."]
pub struct SuperframeOrderR(crate::FieldReader<u8, SuperframeOrderA>);
impl SuperframeOrderR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SuperframeOrderR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> SuperframeOrderA {
        match self.bits {
            _ => unreachable!(),
        }
    }
}
impl core::ops::Deref for SuperframeOrderR {
    type Target = crate::FieldReader<u8, SuperframeOrderA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct SuperframeOrderW<'a> {
    w: &'a mut W,
}
impl<'a> SuperframeOrderW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: SuperframeOrderA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u16 & 0x0f) << 4);
        self.w
    }
}
#[doc = "This fied specifies the final superframe slot utilized by the CAP."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FinalCapSlotA {}
impl From<FinalCapSlotA> for u8 {
    #[inline(always)]
    fn from(variant: FinalCapSlotA) -> Self {
        variant as _
    }
}
#[doc = "Field `FinalCapSlot` reader - This fied specifies the final superframe slot utilized by the CAP."]
pub struct FinalCapSlotR(crate::FieldReader<u8, FinalCapSlotA>);
impl FinalCapSlotR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FinalCapSlotR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> FinalCapSlotA {
        match self.bits {
            _ => unreachable!(),
        }
    }
}
impl core::ops::Deref for FinalCapSlotR {
    type Target = crate::FieldReader<u8, FinalCapSlotA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct FinalCapSlotW<'a> {
    w: &'a mut W,
}
impl<'a> FinalCapSlotW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: FinalCapSlotA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u16 & 0x0f) << 8);
        self.w
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
#[doc = "Specifices if devices are permitted to join the PAN."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssocationPermitA {
    #[doc = "Devices are not permitted to associate with the PAN."]
    NotPermitted = 0,
    #[doc = "Devices are permitted to associate with the PAN."]
    Permitted = 1,
}
impl From<AssocationPermitA> for bool {
    #[inline(always)]
    fn from(variant: AssocationPermitA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AssocationPermit` reader - Specifices if devices are permitted to join the PAN."]
pub struct AssocationPermitR(crate::FieldReader<bool, AssocationPermitA>);
impl AssocationPermitR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AssocationPermitR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> AssocationPermitA {
        match self.bits {
            false => AssocationPermitA::NotPermitted,
            true => AssocationPermitA::Permitted,
        }
    }
    #[doc = "Checks if the value of the `AssocationPermit` field is `NotPermitted`"]
    #[inline(always)]
    pub fn is_not_permitted(&self) -> bool {
        **self == AssocationPermitA::NotPermitted
    }
    #[doc = "Checks if the value of the `AssocationPermit` field is `Permitted`"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        **self == AssocationPermitA::Permitted
    }
}
impl core::ops::Deref for AssocationPermitR {
    type Target = crate::FieldReader<bool, AssocationPermitA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct AssocationPermitW<'a> {
    w: &'a mut W,
}
impl<'a> AssocationPermitW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: AssocationPermitA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `AssocationPermit` field to `NotPermitted`"]
    #[inline(always)]
    pub fn not_permitted(self) -> &'a mut W {
        self.variant(AssocationPermitA::NotPermitted)
    }
    #[doc = "Set the value of the `AssocationPermit` field to `Permitted`"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut W {
        self.variant(AssocationPermitA::Permitted)
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
    pub fn beacon_order(&self) -> BeaconOrderR {
        BeaconOrderR::new(((self.bits >> 0) & 0x0f) as u8)
    }
    #[doc = "Read the `SuperframeOrder` field."]
    #[inline(always)]
    pub fn superframe_order(&self) -> SuperframeOrderR {
        SuperframeOrderR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Read the `FinalCapSlot` field."]
    #[inline(always)]
    pub fn final_cap_slot(&self) -> FinalCapSlotR {
        FinalCapSlotR::new(((self.bits >> 8) & 0x0f) as u8)
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
    #[doc = "Read the `AssocationPermit` field."]
    #[inline(always)]
    pub fn assocation_permit(&self) -> AssocationPermitR {
        AssocationPermitR::new((self.bits & 0x8000) != 0)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u16) -> Self {
        W { bits }
    }
    #[doc = "Set the `BeaconOrder` field."]
    #[inline(always)]
    pub fn beacon_order(&mut self) -> BeaconOrderW {
        BeaconOrderW { w: self }
    }
    #[doc = "Set the `SuperframeOrder` field."]
    #[inline(always)]
    pub fn superframe_order(&mut self) -> SuperframeOrderW {
        SuperframeOrderW { w: self }
    }
    #[doc = "Set the `FinalCapSlot` field."]
    #[inline(always)]
    pub fn final_cap_slot(&mut self) -> FinalCapSlotW {
        FinalCapSlotW { w: self }
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
    #[doc = "Set the `AssocationPermit` field."]
    #[inline(always)]
    pub fn assocation_permit(&mut self) -> AssocationPermitW {
        AssocationPermitW { w: self }
    }
}
