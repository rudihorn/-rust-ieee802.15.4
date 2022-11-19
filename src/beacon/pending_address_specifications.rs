#[doc = ""]
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
impl R {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        R { bits }
    }
    #[doc = "Read the `NumberShortAddresses` field."]
    #[inline(always)]
    pub fn number_short_addresses(&self) -> u8 {
        ((self.bits >> 0) & 0x07) as u8
    }
    #[doc = "Read the `NumberExtendedAddresses` field."]
    #[inline(always)]
    pub fn number_extended_addresses(&self) -> u8 {
        ((self.bits >> 4) & 0x07) as u8
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `NumberShortAddresses` field."]
    #[inline(always)]
    pub fn number_short_addresses(&mut self, value: u8) -> Self {
        let bits = (self.bits & !(0x07 << 0)) | ((value as u8 & 0x07) << 0);
        Self { bits, ..*self }
    }
    #[doc = "Set the `NumberExtendedAddresses` field."]
    #[inline(always)]
    pub fn number_extended_addresses(&mut self, value: u8) -> Self {
        let bits = (self.bits & !(0x07 << 4)) | ((value as u8 & 0x07) << 4);
        Self { bits, ..*self }
    }
}
