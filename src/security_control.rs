#[doc = "This field provides information about what protection is applied to the frame."]
pub struct R {
    bits: u8,
}
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
#[doc = "Indicates the actual frame protection that is provided"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SecurityLevelA {
    #[doc = "Security level 0, no encryption."]
    None = 0,
    #[doc = "Security level 1, uses a 4 byte MIC for data authenticity."]
    Mic32 = 1,
    #[doc = "Security level 2, uses an 8 byte MIC for data authenticity."]
    Mic64 = 2,
    #[doc = "Security level 3, uses a 16 byte MIC for data authenticity."]
    Mic128 = 3,
    #[doc = "Security level 5, uses a 4 byte MIC for data encryption."]
    EncMic32 = 5,
    #[doc = "Security level 6, uses an 8 byte MIC for data encryption."]
    EncMic64 = 6,
    #[doc = "Security level 7, uses a 16 byte MIC for data encryption."]
    EncMic128 = 7,
}
impl From<SecurityLevelA> for u8 {
    #[inline(always)]
    fn from(variant: SecurityLevelA) -> Self {
        variant as _
    }
}
#[doc = "Field `SecurityLevel` reader - Indicates the actual frame protection that is provided"]
pub struct SecurityLevelR(crate::FieldReader<u8, SecurityLevelA>);
impl SecurityLevelR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SecurityLevelR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> SecurityLevelA {
        match self.bits {
            0 => SecurityLevelA::None,
            1 => SecurityLevelA::Mic32,
            2 => SecurityLevelA::Mic64,
            3 => SecurityLevelA::Mic128,
            5 => SecurityLevelA::EncMic32,
            6 => SecurityLevelA::EncMic64,
            7 => SecurityLevelA::EncMic128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `SecurityLevel` field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SecurityLevelA::None
    }
    #[doc = "Checks if the value of the `SecurityLevel` field is `Mic32`"]
    #[inline(always)]
    pub fn is_mic_32(&self) -> bool {
        **self == SecurityLevelA::Mic32
    }
    #[doc = "Checks if the value of the `SecurityLevel` field is `Mic64`"]
    #[inline(always)]
    pub fn is_mic_64(&self) -> bool {
        **self == SecurityLevelA::Mic64
    }
    #[doc = "Checks if the value of the `SecurityLevel` field is `Mic128`"]
    #[inline(always)]
    pub fn is_mic_128(&self) -> bool {
        **self == SecurityLevelA::Mic128
    }
    #[doc = "Checks if the value of the `SecurityLevel` field is `EncMic32`"]
    #[inline(always)]
    pub fn is_enc_mic_32(&self) -> bool {
        **self == SecurityLevelA::EncMic32
    }
    #[doc = "Checks if the value of the `SecurityLevel` field is `EncMic64`"]
    #[inline(always)]
    pub fn is_enc_mic_64(&self) -> bool {
        **self == SecurityLevelA::EncMic64
    }
    #[doc = "Checks if the value of the `SecurityLevel` field is `EncMic128`"]
    #[inline(always)]
    pub fn is_enc_mic_128(&self) -> bool {
        **self == SecurityLevelA::EncMic128
    }
}
impl core::ops::Deref for SecurityLevelR {
    type Target = crate::FieldReader<u8, SecurityLevelA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct SecurityLevelW<'a> {
    w: &'a mut W,
}
impl<'a> SecurityLevelW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: SecurityLevelA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `SecurityLevel` field to `None`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SecurityLevelA::None)
    }
    #[doc = "Set the value of the `SecurityLevel` field to `Mic32`"]
    #[inline(always)]
    pub fn mic_32(self) -> &'a mut W {
        self.variant(SecurityLevelA::Mic32)
    }
    #[doc = "Set the value of the `SecurityLevel` field to `Mic64`"]
    #[inline(always)]
    pub fn mic_64(self) -> &'a mut W {
        self.variant(SecurityLevelA::Mic64)
    }
    #[doc = "Set the value of the `SecurityLevel` field to `Mic128`"]
    #[inline(always)]
    pub fn mic_128(self) -> &'a mut W {
        self.variant(SecurityLevelA::Mic128)
    }
    #[doc = "Set the value of the `SecurityLevel` field to `EncMic32`"]
    #[inline(always)]
    pub fn enc_mic_32(self) -> &'a mut W {
        self.variant(SecurityLevelA::EncMic32)
    }
    #[doc = "Set the value of the `SecurityLevel` field to `EncMic64`"]
    #[inline(always)]
    pub fn enc_mic_64(self) -> &'a mut W {
        self.variant(SecurityLevelA::EncMic64)
    }
    #[doc = "Set the value of the `SecurityLevel` field to `EncMic128`"]
    #[inline(always)]
    pub fn enc_mic_128(self) -> &'a mut W {
        self.variant(SecurityLevelA::EncMic128)
    }
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x04 << 5)) | ((value as u8 & 0x04) << 5);
        self.w
    }
}
#[doc = "Specifies whether the key that is used to protect the frame can be derived implicitly or explicitly."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyIdentifierModeA {
    #[doc = "Key is determined implicitly."]
    Implicit = 0,
    #[doc = "Key is determined from the key index field."]
    KeyIndex = 1,
    #[doc = "Key is determined explicitly from the 4-octet key source and key index fields."]
    KeySource4 = 2,
    #[doc = "Key is determined explicitly from the 8-octet key source and key index fields."]
    KeySource8 = 3,
}
impl From<KeyIdentifierModeA> for u8 {
    #[inline(always)]
    fn from(variant: KeyIdentifierModeA) -> Self {
        variant as _
    }
}
#[doc = "Field `KeyIdentifierMode` reader - Specifies whether the key that is used to protect the frame can be derived implicitly or explicitly."]
pub struct KeyIdentifierModeR(crate::FieldReader<u8, KeyIdentifierModeA>);
impl KeyIdentifierModeR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KeyIdentifierModeR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> KeyIdentifierModeA {
        match self.bits {
            0 => KeyIdentifierModeA::Implicit,
            1 => KeyIdentifierModeA::KeyIndex,
            2 => KeyIdentifierModeA::KeySource4,
            3 => KeyIdentifierModeA::KeySource8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `KeyIdentifierMode` field is `Implicit`"]
    #[inline(always)]
    pub fn is_implicit(&self) -> bool {
        **self == KeyIdentifierModeA::Implicit
    }
    #[doc = "Checks if the value of the `KeyIdentifierMode` field is `KeyIndex`"]
    #[inline(always)]
    pub fn is_key_index(&self) -> bool {
        **self == KeyIdentifierModeA::KeyIndex
    }
    #[doc = "Checks if the value of the `KeyIdentifierMode` field is `KeySource4`"]
    #[inline(always)]
    pub fn is_key_source_4(&self) -> bool {
        **self == KeyIdentifierModeA::KeySource4
    }
    #[doc = "Checks if the value of the `KeyIdentifierMode` field is `KeySource8`"]
    #[inline(always)]
    pub fn is_key_source_8(&self) -> bool {
        **self == KeyIdentifierModeA::KeySource8
    }
}
impl core::ops::Deref for KeyIdentifierModeR {
    type Target = crate::FieldReader<u8, KeyIdentifierModeA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct KeyIdentifierModeW<'a> {
    w: &'a mut W,
}
impl<'a> KeyIdentifierModeW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: KeyIdentifierModeA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `KeyIdentifierMode` field to `Implicit`"]
    #[inline(always)]
    pub fn implicit(self) -> &'a mut W {
        self.variant(KeyIdentifierModeA::Implicit)
    }
    #[doc = "Set the value of the `KeyIdentifierMode` field to `KeyIndex`"]
    #[inline(always)]
    pub fn key_index(self) -> &'a mut W {
        self.variant(KeyIdentifierModeA::KeyIndex)
    }
    #[doc = "Set the value of the `KeyIdentifierMode` field to `KeySource4`"]
    #[inline(always)]
    pub fn key_source_4(self) -> &'a mut W {
        self.variant(KeyIdentifierModeA::KeySource4)
    }
    #[doc = "Set the value of the `KeyIdentifierMode` field to `KeySource8`"]
    #[inline(always)]
    pub fn key_source_8(self) -> &'a mut W {
        self.variant(KeyIdentifierModeA::KeySource8)
    }
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x02 << 3)) | ((value as u8 & 0x02) << 3);
        self.w
    }
}
#[doc = "Specifies if the frame counter should be suppressed from the frame."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameCounterSuppresionA {
    #[doc = "The frame counter is included in the frame."]
    Present = 0,
    #[doc = "The frame counter is suppressed from the frame."]
    Suppressed = 1,
}
impl From<FrameCounterSuppresionA> for bool {
    #[inline(always)]
    fn from(variant: FrameCounterSuppresionA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FrameCounterSuppresion` reader - Specifies if the frame counter should be suppressed from the frame."]
pub struct FrameCounterSuppresionR(crate::FieldReader<bool, FrameCounterSuppresionA>);
impl FrameCounterSuppresionR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FrameCounterSuppresionR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> FrameCounterSuppresionA {
        match self.bits {
            false => FrameCounterSuppresionA::Present,
            true => FrameCounterSuppresionA::Suppressed,
        }
    }
    #[doc = "Checks if the value of the `FrameCounterSuppresion` field is `Present`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == FrameCounterSuppresionA::Present
    }
    #[doc = "Checks if the value of the `FrameCounterSuppresion` field is `Suppressed`"]
    #[inline(always)]
    pub fn is_suppressed(&self) -> bool {
        **self == FrameCounterSuppresionA::Suppressed
    }
}
impl core::ops::Deref for FrameCounterSuppresionR {
    type Target = crate::FieldReader<bool, FrameCounterSuppresionA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct FrameCounterSuppresionW<'a> {
    w: &'a mut W,
}
impl<'a> FrameCounterSuppresionW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: FrameCounterSuppresionA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `FrameCounterSuppresion` field to `Present`"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(FrameCounterSuppresionA::Present)
    }
    #[doc = "Set the value of the `FrameCounterSuppresion` field to `Suppressed`"]
    #[inline(always)]
    pub fn suppressed(self) -> &'a mut W {
        self.variant(FrameCounterSuppresionA::Suppressed)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Specifies if the absolute number slot (ASN) is used to generate the Nonce."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AsnInNonceA {
    #[doc = "The frame counter is used to generate the Nonce."]
    FrameCounterNonce = 0,
    #[doc = "The ASN is used to generate the Nonce."]
    AsnNonce = 1,
}
impl From<AsnInNonceA> for bool {
    #[inline(always)]
    fn from(variant: AsnInNonceA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AsnInNonce` reader - Specifies if the absolute number slot (ASN) is used to generate the Nonce."]
pub struct AsnInNonceR(crate::FieldReader<bool, AsnInNonceA>);
impl AsnInNonceR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AsnInNonceR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> AsnInNonceA {
        match self.bits {
            false => AsnInNonceA::FrameCounterNonce,
            true => AsnInNonceA::AsnNonce,
        }
    }
    #[doc = "Checks if the value of the `AsnInNonce` field is `FrameCounterNonce`"]
    #[inline(always)]
    pub fn is_frame_counter_nonce(&self) -> bool {
        **self == AsnInNonceA::FrameCounterNonce
    }
    #[doc = "Checks if the value of the `AsnInNonce` field is `AsnNonce`"]
    #[inline(always)]
    pub fn is_asn_nonce(&self) -> bool {
        **self == AsnInNonceA::AsnNonce
    }
}
impl core::ops::Deref for AsnInNonceR {
    type Target = crate::FieldReader<bool, AsnInNonceA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct AsnInNonceW<'a> {
    w: &'a mut W,
}
impl<'a> AsnInNonceW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: AsnInNonceA) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set the value of the `AsnInNonce` field to `FrameCounterNonce`"]
    #[inline(always)]
    pub fn frame_counter_nonce(self) -> &'a mut W {
        self.variant(AsnInNonceA::FrameCounterNonce)
    }
    #[doc = "Set the value of the `AsnInNonce` field to `AsnNonce`"]
    #[inline(always)]
    pub fn asn_nonce(self) -> &'a mut W {
        self.variant(AsnInNonceA::AsnNonce)
    }
    #[inline(always)]
    pub fn bits(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        R { bits }
    }
    #[doc = "Read the `SecurityLevel` field."]
    #[inline(always)]
    pub fn security_level(&self) -> SecurityLevelR {
        SecurityLevelR::new(((self.bits >> 5) & 0x04) as u8)
    }
    #[doc = "Read the `KeyIdentifierMode` field."]
    #[inline(always)]
    pub fn key_identifier_mode(&self) -> KeyIdentifierModeR {
        KeyIdentifierModeR::new(((self.bits >> 3) & 0x02) as u8)
    }
    #[doc = "Read the `FrameCounterSuppresion` field."]
    #[inline(always)]
    pub fn frame_counter_suppresion(&self) -> FrameCounterSuppresionR {
        FrameCounterSuppresionR::new((self.bits & 0x04) != 0)
    }
    #[doc = "Read the `AsnInNonce` field."]
    #[inline(always)]
    pub fn asn_in_nonce(&self) -> AsnInNonceR {
        AsnInNonceR::new((self.bits & 0x02) != 0)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `SecurityLevel` field."]
    #[inline(always)]
    pub fn security_level(&mut self) -> SecurityLevelW {
        SecurityLevelW { w: self }
    }
    #[doc = "Set the `KeyIdentifierMode` field."]
    #[inline(always)]
    pub fn key_identifier_mode(&mut self) -> KeyIdentifierModeW {
        KeyIdentifierModeW { w: self }
    }
    #[doc = "Set the `FrameCounterSuppresion` field."]
    #[inline(always)]
    pub fn frame_counter_suppresion(&mut self) -> FrameCounterSuppresionW {
        FrameCounterSuppresionW { w: self }
    }
    #[doc = "Set the `AsnInNonce` field."]
    #[inline(always)]
    pub fn asn_in_nonce(&mut self) -> AsnInNonceW {
        AsnInNonceW { w: self }
    }
}
