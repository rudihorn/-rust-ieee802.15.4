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
    #[doc = ""]
    Beacon = 0,
    #[doc = ""]
    Data = 1,
    #[doc = ""]
    Acknowledgement = 2,
    #[doc = ""]
    MacCommand = 3,
    #[doc = ""]
    Multipurpose = 5,
    #[doc = ""]
    Fragment = 6,
    #[doc = ""]
    Extended = 7,
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
            5 => FrameTypeA::Multipurpose,
            6 => FrameTypeA::Fragment,
            7 => FrameTypeA::Extended,
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
    #[doc = "Checks if the value of the `FrameType` field is `Multipurpose`"]
    #[inline(always)]
    pub fn is_multipurpose(&self) -> bool {
        **self == FrameTypeA::Multipurpose
    }
    #[doc = "Checks if the value of the `FrameType` field is `Fragment`"]
    #[inline(always)]
    pub fn is_fragment(&self) -> bool {
        **self == FrameTypeA::Fragment
    }
    #[doc = "Checks if the value of the `FrameType` field is `Extended`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        **self == FrameTypeA::Extended
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
    #[doc = "Set the value of the `FrameType` field to `Multipurpose`"]
    #[inline(always)]
    pub fn multipurpose(self) -> &'a mut W {
        self.variant(FrameTypeA::Multipurpose)
    }
    #[doc = "Set the value of the `FrameType` field to `Fragment`"]
    #[inline(always)]
    pub fn fragment(self) -> &'a mut W {
        self.variant(FrameTypeA::Fragment)
    }
    #[doc = "Set the value of the `FrameType` field to `Extended`"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(FrameTypeA::Extended)
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
    #[doc = ""]
    Unencrypted = 0,
    #[doc = ""]
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
    #[doc = ""]
    NoFramePending = 0,
    #[doc = ""]
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
    #[doc = ""]
    AckNotRequested = 0,
    #[doc = ""]
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
pub enum PanCompressionA {
    #[doc = ""]
    Compressed = 0,
    #[doc = ""]
    Uncompressed = 1,
}
impl From<PanCompressionA> for bool {
    #[inline(always)]
    fn from(variant: PanCompressionA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PanCompression` reader - Specifies whether the MAC frame is to be sent within the same PAN."]
pub struct PanCompressionR(crate::FieldReader<bool, PanCompressionA>);
impl PanCompressionR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PanCompressionR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> PanCompressionA {
        match self.bits {
            false => PanCompressionA::Compressed,
            true => PanCompressionA::Uncompressed,
        }
    }
    #[doc = "Checks if the value of the `PanCompression` field is `Compressed`"]
    #[inline(always)]
    pub fn is_compressed(&self) -> bool {
        **self == PanCompressionA::Compressed
    }
    #[doc = "Checks if the value of the `PanCompression` field is `Uncompressed`"]
    #[inline(always)]
    pub fn is_uncompressed(&self) -> bool {
        **self == PanCompressionA::Uncompressed
    }
}
impl core::ops::Deref for PanCompressionR {
    type Target = crate::FieldReader<bool, PanCompressionA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct PanCompressionW<'a> {
    w: &'a mut W,
}
impl<'a> PanCompressionW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: PanCompressionA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `PanCompression` field to `Compressed`"]
    #[inline(always)]
    pub fn compressed(self) -> &'a mut W {
        self.variant(PanCompressionA::Compressed)
    }
    #[doc = "Set the value of the `PanCompression` field to `Uncompressed`"]
    #[inline(always)]
    pub fn uncompressed(self) -> &'a mut W {
        self.variant(PanCompressionA::Uncompressed)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Specifies if the sequence number should be suppressed."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SeqNrSuppressionA {
    #[doc = ""]
    Included = 0,
    #[doc = ""]
    Suppressed = 1,
}
impl From<SeqNrSuppressionA> for bool {
    #[inline(always)]
    fn from(variant: SeqNrSuppressionA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SeqNrSuppression` reader - Specifies if the sequence number should be suppressed."]
pub struct SeqNrSuppressionR(crate::FieldReader<bool, SeqNrSuppressionA>);
impl SeqNrSuppressionR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SeqNrSuppressionR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> SeqNrSuppressionA {
        match self.bits {
            false => SeqNrSuppressionA::Included,
            true => SeqNrSuppressionA::Suppressed,
        }
    }
    #[doc = "Checks if the value of the `SeqNrSuppression` field is `Included`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == SeqNrSuppressionA::Included
    }
    #[doc = "Checks if the value of the `SeqNrSuppression` field is `Suppressed`"]
    #[inline(always)]
    pub fn is_suppressed(&self) -> bool {
        **self == SeqNrSuppressionA::Suppressed
    }
}
impl core::ops::Deref for SeqNrSuppressionR {
    type Target = crate::FieldReader<bool, SeqNrSuppressionA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct SeqNrSuppressionW<'a> {
    w: &'a mut W,
}
impl<'a> SeqNrSuppressionW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: SeqNrSuppressionA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `SeqNrSuppression` field to `Included`"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(SeqNrSuppressionA::Included)
    }
    #[doc = "Set the value of the `SeqNrSuppression` field to `Suppressed`"]
    #[inline(always)]
    pub fn suppressed(self) -> &'a mut W {
        self.variant(SeqNrSuppressionA::Suppressed)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Specified if Information Elements (IEs) are contained in the frame."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IePresentA {
    #[doc = ""]
    None = 0,
    #[doc = ""]
    Present = 1,
}
impl From<IePresentA> for bool {
    #[inline(always)]
    fn from(variant: IePresentA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IePresent` reader - Specified if Information Elements (IEs) are contained in the frame."]
pub struct IePresentR(crate::FieldReader<bool, IePresentA>);
impl IePresentR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IePresentR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> IePresentA {
        match self.bits {
            false => IePresentA::None,
            true => IePresentA::Present,
        }
    }
    #[doc = "Checks if the value of the `IePresent` field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == IePresentA::None
    }
    #[doc = "Checks if the value of the `IePresent` field is `Present`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == IePresentA::Present
    }
}
impl core::ops::Deref for IePresentR {
    type Target = crate::FieldReader<bool, IePresentA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct IePresentW<'a> {
    w: &'a mut W,
}
impl<'a> IePresentW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: IePresentA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `IePresent` field to `None`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(IePresentA::None)
    }
    #[doc = "Set the value of the `IePresent` field to `Present`"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(IePresentA::Present)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Specifies the type of the destination address."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DestAddrModeA {
    #[doc = ""]
    NotPresent = 0,
    #[doc = ""]
    Address16bit = 1,
    #[doc = ""]
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
#[doc = "Specifies the version of the frame"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameVersionA {
    #[doc = ""]
    Version2003 = 0,
    #[doc = ""]
    Version2006 = 1,
    #[doc = ""]
    Current = 2,
}
impl From<FrameVersionA> for u8 {
    #[inline(always)]
    fn from(variant: FrameVersionA) -> Self {
        variant as _
    }
}
#[doc = "Field `FrameVersion` reader - Specifies the version of the frame"]
pub struct FrameVersionR(crate::FieldReader<u8, FrameVersionA>);
impl FrameVersionR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FrameVersionR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> FrameVersionA {
        match self.bits {
            0 => FrameVersionA::Version2003,
            1 => FrameVersionA::Version2006,
            2 => FrameVersionA::Current,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `FrameVersion` field is `Version2003`"]
    #[inline(always)]
    pub fn is_version_2003(&self) -> bool {
        **self == FrameVersionA::Version2003
    }
    #[doc = "Checks if the value of the `FrameVersion` field is `Version2006`"]
    #[inline(always)]
    pub fn is_version_2006(&self) -> bool {
        **self == FrameVersionA::Version2006
    }
    #[doc = "Checks if the value of the `FrameVersion` field is `Current`"]
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        **self == FrameVersionA::Current
    }
}
impl core::ops::Deref for FrameVersionR {
    type Target = crate::FieldReader<u8, FrameVersionA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct FrameVersionW<'a> {
    w: &'a mut W,
}
impl<'a> FrameVersionW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: FrameVersionA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `FrameVersion` field to `Version2003`"]
    #[inline(always)]
    pub fn version_2003(self) -> &'a mut W {
        self.variant(FrameVersionA::Version2003)
    }
    #[doc = "Set the value of the `FrameVersion` field to `Version2006`"]
    #[inline(always)]
    pub fn version_2006(self) -> &'a mut W {
        self.variant(FrameVersionA::Version2006)
    }
    #[doc = "Set the value of the `FrameVersion` field to `Current`"]
    #[inline(always)]
    pub fn current(self) -> &'a mut W {
        self.variant(FrameVersionA::Current)
    }
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x02 << 2)) | ((value as u16 & 0x02) << 2);
        self.w
    }
}
#[doc = "Specifies the type of the source address."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SourceAddrModeA {
    #[doc = ""]
    NotPresent = 0,
    #[doc = ""]
    Address16bit = 1,
    #[doc = ""]
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
    #[doc = "Read the `PanCompression` field."]
    #[inline(always)]
    pub fn pan_compression(&self) -> PanCompressionR {
        PanCompressionR::new((self.bits & 0x0200) != 0)
    }
    #[doc = "Read the `SeqNrSuppression` field."]
    #[inline(always)]
    pub fn seq_nr_suppression(&self) -> SeqNrSuppressionR {
        SeqNrSuppressionR::new((self.bits & 0x80) != 0)
    }
    #[doc = "Read the `IePresent` field."]
    #[inline(always)]
    pub fn ie_present(&self) -> IePresentR {
        IePresentR::new((self.bits & 0x40) != 0)
    }
    #[doc = "Read the `DestAddrMode` field."]
    #[inline(always)]
    pub fn dest_addr_mode(&self) -> DestAddrModeR {
        DestAddrModeR::new(((self.bits >> 4) & 0x02) as u8)
    }
    #[doc = "Read the `FrameVersion` field."]
    #[inline(always)]
    pub fn frame_version(&self) -> FrameVersionR {
        FrameVersionR::new(((self.bits >> 2) & 0x02) as u8)
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
    #[doc = "Set the `PanCompression` field."]
    #[inline(always)]
    pub fn pan_compression(&mut self) -> PanCompressionW {
        PanCompressionW { w: self }
    }
    #[doc = "Set the `SeqNrSuppression` field."]
    #[inline(always)]
    pub fn seq_nr_suppression(&mut self) -> SeqNrSuppressionW {
        SeqNrSuppressionW { w: self }
    }
    #[doc = "Set the `IePresent` field."]
    #[inline(always)]
    pub fn ie_present(&mut self) -> IePresentW {
        IePresentW { w: self }
    }
    #[doc = "Set the `DestAddrMode` field."]
    #[inline(always)]
    pub fn dest_addr_mode(&mut self) -> DestAddrModeW {
        DestAddrModeW { w: self }
    }
    #[doc = "Set the `FrameVersion` field."]
    #[inline(always)]
    pub fn frame_version(&mut self) -> FrameVersionW {
        FrameVersionW { w: self }
    }
    #[doc = "Set the `SourceAddrMode` field."]
    #[inline(always)]
    pub fn source_addr_mode(&mut self) -> SourceAddrModeW {
        SourceAddrModeW { w: self }
    }
}
