#[doc = "The starting slot and length of a guaranteed time slot. Note that this does not include the device short address."]
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
    #[doc = "Read the `StartingSlot` field."]
    #[inline(always)]
    pub fn starting_slot(&self) -> u8 {
        ((self.bits >> 0) & 0x0f) as u8
    }
    #[doc = "Read the `Length` field."]
    #[inline(always)]
    pub fn length(&self) -> u8 {
        ((self.bits >> 4) & 0x0f) as u8
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `StartingSlot` field."]
    #[inline(always)]
    pub fn starting_slot(&mut self, value: u8) -> Self {
        let bits = (self.bits & !(0x0f << 0)) | ((value as u8 & 0x0f) << 0);
        Self { bits, ..*self }
    }
    #[doc = "Set the `Length` field."]
    #[inline(always)]
    pub fn length(&mut self, value: u8) -> Self {
        let bits = (self.bits & !(0x0f << 4)) | ((value as u8 & 0x0f) << 4);
        Self { bits, ..*self }
    }
}
