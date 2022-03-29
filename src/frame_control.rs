#[doc = "This field contains information about the frame type, addressing and control flags."]
pub struct R {
    bits: u16,
}
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
#[doc = "This field contains information about the frame type, addressing and control flags."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameTypeA {
    Beacon = 0,
    Data = 1,
    Acknowledgement = 2,
    MacCommand = 3,
}
impl From<FrameTypeA> for u8 {
    #[inline(always)]
    fn from(variant: FrameTypeA) -> Self {
        variant as _
    }
}
#[doc = "Field `FrameType` reader - This field contains information about the frame type, addressing and control flags."]
pub struct FrameTypeR(crate::FieldReader<u8, FrameTypeA>);
impl FrameTypeR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FrameTypeR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> FrameTypeA {
        match self.bits {
            0 => FrameTypeA::Beacon,
            1 => FrameTypeA::Data,
            2 => FrameTypeA::Acknowledgement,
            3 => FrameTypeA::MacCommand,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `FrameType` field is `Beacon`"]
    #[inline(always)]
    pub fn is_beacon(&self) -> bool {
        **self == FrameTypeA::Beacon
    }
    #[doc = "Checks if the value of the `FrameType` field is `Data`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        **self == FrameTypeA::Data
    }
    #[doc = "Checks if the value of the `FrameType` field is `Acknowledgement`"]
    #[inline(always)]
    pub fn is_acknowledgement(&self) -> bool {
        **self == FrameTypeA::Acknowledgement
    }
    #[doc = "Checks if the value of the `FrameType` field is `MacCommand`"]
    #[inline(always)]
    pub fn is_mac_command(&self) -> bool {
        **self == FrameTypeA::MacCommand
    }
}
impl core::ops::Deref for FrameTypeR {
    type Target = crate::FieldReader<u8, FrameTypeA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct FrameTypeW<'a> {
    w: &'a mut W,
}
impl<'a> FrameTypeW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: FrameTypeA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `FrameType` field to `Beacon`"]
    #[inline(always)]
    pub fn beacon(self) -> &'a mut W {
        self.variant(FrameTypeA::Beacon)
    }
    #[doc = "Set the value of the `FrameType` field to `Data`"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(FrameTypeA::Data)
    }
    #[doc = "Set the value of the `FrameType` field to `Acknowledgement`"]
    #[inline(always)]
    pub fn acknowledgement(self) -> &'a mut W {
        self.variant(FrameTypeA::Acknowledgement)
    }
    #[doc = "Set the value of the `FrameType` field to `MacCommand`"]
    #[inline(always)]
    pub fn mac_command(self) -> &'a mut W {
        self.variant(FrameTypeA::MacCommand)
    }
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x04 << 13)) | ((value as u16 & 0x04) << 13);
        self.w
    }
}
#[doc = "Specifies if the frame is encrypted using the key stored in the PIB."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SecurityEnabledA {
    Unencrypted = 0,
    Encrypted = 1,
}
impl From<SecurityEnabledA> for bool {
    #[inline(always)]
    fn from(variant: SecurityEnabledA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecurityEnabled` reader - Specifies if the frame is encrypted using the key stored in the PIB."]
pub struct SecurityEnabledR(crate::FieldReader<bool, SecurityEnabledA>);
impl SecurityEnabledR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SecurityEnabledR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> SecurityEnabledA {
        match self.bits {
            false => SecurityEnabledA::Unencrypted,
            true => SecurityEnabledA::Encrypted,
        }
    }
    #[doc = "Checks if the value of the `SecurityEnabled` field is `Unencrypted`"]
    #[inline(always)]
    pub fn is_unencrypted(&self) -> bool {
        **self == SecurityEnabledA::Unencrypted
    }
    #[doc = "Checks if the value of the `SecurityEnabled` field is `Encrypted`"]
    #[inline(always)]
    pub fn is_encrypted(&self) -> bool {
        **self == SecurityEnabledA::Encrypted
    }
}
impl core::ops::Deref for SecurityEnabledR {
    type Target = crate::FieldReader<bool, SecurityEnabledA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct SecurityEnabledW<'a> {
    w: &'a mut W,
}
impl<'a> SecurityEnabledW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: SecurityEnabledA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `SecurityEnabled` field to `Unencrypted`"]
    #[inline(always)]
    pub fn unencrypted(self) -> &'a mut W {
        self.variant(SecurityEnabledA::Unencrypted)
    }
    #[doc = "Set the value of the `SecurityEnabled` field to `Encrypted`"]
    #[inline(always)]
    pub fn encrypted(self) -> &'a mut W {
        self.variant(SecurityEnabledA::Encrypted)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Specifies if the sender has additional data to send to the recipient."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FramePendingA {
    NoFramePending = 0,
    FramePending = 1,
}
impl From<FramePendingA> for bool {
    #[inline(always)]
    fn from(variant: FramePendingA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FramePending` reader - Specifies if the sender has additional data to send to the recipient."]
pub struct FramePendingR(crate::FieldReader<bool, FramePendingA>);
impl FramePendingR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FramePendingR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> FramePendingA {
        match self.bits {
            false => FramePendingA::NoFramePending,
            true => FramePendingA::FramePending,
        }
    }
    #[doc = "Checks if the value of the `FramePending` field is `NoFramePending`"]
    #[inline(always)]
    pub fn is_no_frame_pending(&self) -> bool {
        **self == FramePendingA::NoFramePending
    }
    #[doc = "Checks if the value of the `FramePending` field is `FramePending`"]
    #[inline(always)]
    pub fn is_frame_pending(&self) -> bool {
        **self == FramePendingA::FramePending
    }
}
impl core::ops::Deref for FramePendingR {
    type Target = crate::FieldReader<bool, FramePendingA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct FramePendingW<'a> {
    w: &'a mut W,
}
impl<'a> FramePendingW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: FramePendingA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `FramePending` field to `NoFramePending`"]
    #[inline(always)]
    pub fn no_frame_pending(self) -> &'a mut W {
        self.variant(FramePendingA::NoFramePending)
    }
    #[doc = "Set the value of the `FramePending` field to `FramePending`"]
    #[inline(always)]
    pub fn frame_pending(self) -> &'a mut W {
        self.variant(FramePendingA::FramePending)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Specifies whether an acknowledgement is required from the recipient device."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AckRequestA {
    AckNotRequested = 0,
    AckRequested = 1,
}
impl From<AckRequestA> for bool {
    #[inline(always)]
    fn from(variant: AckRequestA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AckRequest` reader - Specifies whether an acknowledgement is required from the recipient device."]
pub struct AckRequestR(crate::FieldReader<bool, AckRequestA>);
impl AckRequestR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AckRequestR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> AckRequestA {
        match self.bits {
            false => AckRequestA::AckNotRequested,
            true => AckRequestA::AckRequested,
        }
    }
    #[doc = "Checks if the value of the `AckRequest` field is `AckNotRequested`"]
    #[inline(always)]
    pub fn is_ack_not_requested(&self) -> bool {
        **self == AckRequestA::AckNotRequested
    }
    #[doc = "Checks if the value of the `AckRequest` field is `AckRequested`"]
    #[inline(always)]
    pub fn is_ack_requested(&self) -> bool {
        **self == AckRequestA::AckRequested
    }
}
impl core::ops::Deref for AckRequestR {
    type Target = crate::FieldReader<bool, AckRequestA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct AckRequestW<'a> {
    w: &'a mut W,
}
impl<'a> AckRequestW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: AckRequestA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `AckRequest` field to `AckNotRequested`"]
    #[inline(always)]
    pub fn ack_not_requested(self) -> &'a mut W {
        self.variant(AckRequestA::AckNotRequested)
    }
    #[doc = "Set the value of the `AckRequest` field to `AckRequested`"]
    #[inline(always)]
    pub fn ack_requested(self) -> &'a mut W {
        self.variant(AckRequestA::AckRequested)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Specifies whether the MAC frame is to be sent within the same PAN."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntraPanA {
    PanPresent = 0,
    InterPan = 1,
}
impl From<IntraPanA> for bool {
    #[inline(always)]
    fn from(variant: IntraPanA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IntraPan` reader - Specifies whether the MAC frame is to be sent within the same PAN."]
pub struct IntraPanR(crate::FieldReader<bool, IntraPanA>);
impl IntraPanR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IntraPanR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> IntraPanA {
        match self.bits {
            false => IntraPanA::PanPresent,
            true => IntraPanA::InterPan,
        }
    }
    #[doc = "Checks if the value of the `IntraPan` field is `PanPresent`"]
    #[inline(always)]
    pub fn is_pan_present(&self) -> bool {
        **self == IntraPanA::PanPresent
    }
    #[doc = "Checks if the value of the `IntraPan` field is `InterPan`"]
    #[inline(always)]
    pub fn is_inter_pan(&self) -> bool {
        **self == IntraPanA::InterPan
    }
}
impl core::ops::Deref for IntraPanR {
    type Target = crate::FieldReader<bool, IntraPanA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct IntraPanW<'a> {
    w: &'a mut W,
}
impl<'a> IntraPanW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: IntraPanA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `IntraPan` field to `PanPresent`"]
    #[inline(always)]
    pub fn pan_present(self) -> &'a mut W {
        self.variant(IntraPanA::PanPresent)
    }
    #[doc = "Set the value of the `IntraPan` field to `InterPan`"]
    #[inline(always)]
    pub fn inter_pan(self) -> &'a mut W {
        self.variant(IntraPanA::InterPan)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Specifies the type of the destination address."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DestAddrModeA {
    NotPresent = 0,
    Address16bit = 1,
    Address64bitExtended = 3,
}
impl From<DestAddrModeA> for u8 {
    #[inline(always)]
    fn from(variant: DestAddrModeA) -> Self {
        variant as _
    }
}
#[doc = "Field `DestAddrMode` reader - Specifies the type of the destination address."]
pub struct DestAddrModeR(crate::FieldReader<u8, DestAddrModeA>);
impl DestAddrModeR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DestAddrModeR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> DestAddrModeA {
        match self.bits {
            0 => DestAddrModeA::NotPresent,
            1 => DestAddrModeA::Address16bit,
            3 => DestAddrModeA::Address64bitExtended,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `DestAddrMode` field is `NotPresent`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == DestAddrModeA::NotPresent
    }
    #[doc = "Checks if the value of the `DestAddrMode` field is `Address16bit`"]
    #[inline(always)]
    pub fn is_address_16bit(&self) -> bool {
        **self == DestAddrModeA::Address16bit
    }
    #[doc = "Checks if the value of the `DestAddrMode` field is `Address64bitExtended`"]
    #[inline(always)]
    pub fn is_address_64bit_extended(&self) -> bool {
        **self == DestAddrModeA::Address64bitExtended
    }
}
impl core::ops::Deref for DestAddrModeR {
    type Target = crate::FieldReader<u8, DestAddrModeA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct DestAddrModeW<'a> {
    w: &'a mut W,
}
impl<'a> DestAddrModeW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: DestAddrModeA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `DestAddrMode` field to `NotPresent`"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(DestAddrModeA::NotPresent)
    }
    #[doc = "Set the value of the `DestAddrMode` field to `Address16bit`"]
    #[inline(always)]
    pub fn address_16bit(self) -> &'a mut W {
        self.variant(DestAddrModeA::Address16bit)
    }
    #[doc = "Set the value of the `DestAddrMode` field to `Address64bitExtended`"]
    #[inline(always)]
    pub fn address_64bit_extended(self) -> &'a mut W {
        self.variant(DestAddrModeA::Address64bitExtended)
    }
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x02 << 4)) | ((value as u16 & 0x02) << 4);
        self.w
    }
}
#[doc = "Specifies the type of the source address."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SourceAddrModeA {
    NotPresent = 0,
    Address16bit = 1,
    Address64bitExtended = 3,
}
impl From<SourceAddrModeA> for u8 {
    #[inline(always)]
    fn from(variant: SourceAddrModeA) -> Self {
        variant as _
    }
}
#[doc = "Field `SourceAddrMode` reader - Specifies the type of the source address."]
pub struct SourceAddrModeR(crate::FieldReader<u8, SourceAddrModeA>);
impl SourceAddrModeR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SourceAddrModeR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> SourceAddrModeA {
        match self.bits {
            0 => SourceAddrModeA::NotPresent,
            1 => SourceAddrModeA::Address16bit,
            3 => SourceAddrModeA::Address64bitExtended,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `SourceAddrMode` field is `NotPresent`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == SourceAddrModeA::NotPresent
    }
    #[doc = "Checks if the value of the `SourceAddrMode` field is `Address16bit`"]
    #[inline(always)]
    pub fn is_address_16bit(&self) -> bool {
        **self == SourceAddrModeA::Address16bit
    }
    #[doc = "Checks if the value of the `SourceAddrMode` field is `Address64bitExtended`"]
    #[inline(always)]
    pub fn is_address_64bit_extended(&self) -> bool {
        **self == SourceAddrModeA::Address64bitExtended
    }
}
impl core::ops::Deref for SourceAddrModeR {
    type Target = crate::FieldReader<u8, SourceAddrModeA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct SourceAddrModeW<'a> {
    w: &'a mut W,
}
impl<'a> SourceAddrModeW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: SourceAddrModeA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `SourceAddrMode` field to `NotPresent`"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(SourceAddrModeA::NotPresent)
    }
    #[doc = "Set the value of the `SourceAddrMode` field to `Address16bit`"]
    #[inline(always)]
    pub fn address_16bit(self) -> &'a mut W {
        self.variant(SourceAddrModeA::Address16bit)
    }
    #[doc = "Set the value of the `SourceAddrMode` field to `Address64bitExtended`"]
    #[inline(always)]
    pub fn address_64bit_extended(self) -> &'a mut W {
        self.variant(SourceAddrModeA::Address64bitExtended)
    }
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x02 << 0)) | ((value as u16 & 0x02) << 0);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u16) -> Self {
        R { bits }
    }
    #[doc = "Read the `FrameType` field."]
    #[inline(always)]
    pub fn frame_type(&self) -> FrameTypeR {
        FrameTypeR::new(((self.bits >> 13) & 0x04) as u8)
    }
    #[doc = "Read the `SecurityEnabled` field."]
    #[inline(always)]
    pub fn security_enabled(&self) -> SecurityEnabledR {
        SecurityEnabledR::new((self.bits & 0x1000) != 0)
    }
    #[doc = "Read the `FramePending` field."]
    #[inline(always)]
    pub fn frame_pending(&self) -> FramePendingR {
        FramePendingR::new((self.bits & 0x0800) != 0)
    }
    #[doc = "Read the `AckRequest` field."]
    #[inline(always)]
    pub fn ack_request(&self) -> AckRequestR {
        AckRequestR::new((self.bits & 0x0400) != 0)
    }
    #[doc = "Read the `IntraPan` field."]
    #[inline(always)]
    pub fn intra_pan(&self) -> IntraPanR {
        IntraPanR::new((self.bits & 0x0200) != 0)
    }
    #[doc = "Read the `DestAddrMode` field."]
    #[inline(always)]
    pub fn dest_addr_mode(&self) -> DestAddrModeR {
        DestAddrModeR::new(((self.bits >> 4) & 0x02) as u8)
    }
    #[doc = "Read the `SourceAddrMode` field."]
    #[inline(always)]
    pub fn source_addr_mode(&self) -> SourceAddrModeR {
        SourceAddrModeR::new(((self.bits >> 0) & 0x02) as u8)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u16) -> Self {
        W { bits }
    }
    #[doc = "Set the `FrameType` field."]
    #[inline(always)]
    pub fn frame_type(&mut self) -> FrameTypeW {
        FrameTypeW { w: self }
    }
    #[doc = "Set the `SecurityEnabled` field."]
    #[inline(always)]
    pub fn security_enabled(&mut self) -> SecurityEnabledW {
        SecurityEnabledW { w: self }
    }
    #[doc = "Set the `FramePending` field."]
    #[inline(always)]
    pub fn frame_pending(&mut self) -> FramePendingW {
        FramePendingW { w: self }
    }
    #[doc = "Set the `AckRequest` field."]
    #[inline(always)]
    pub fn ack_request(&mut self) -> AckRequestW {
        AckRequestW { w: self }
    }
    #[doc = "Set the `IntraPan` field."]
    #[inline(always)]
    pub fn intra_pan(&mut self) -> IntraPanW {
        IntraPanW { w: self }
    }
    #[doc = "Set the `DestAddrMode` field."]
    #[inline(always)]
    pub fn dest_addr_mode(&mut self) -> DestAddrModeW {
        DestAddrModeW { w: self }
    }
    #[doc = "Set the `SourceAddrMode` field."]
    #[inline(always)]
    pub fn source_addr_mode(&mut self) -> SourceAddrModeW {
        SourceAddrModeW { w: self }
    }
}
