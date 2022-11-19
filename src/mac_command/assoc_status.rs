#[doc = "Association status"]
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
#[doc = "The association status after a request."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssociationStatusA {
    #[doc = "Association successful."]
    AssocSuccess = 0,
    #[doc = "The PAN is at capacity."]
    PanAtCapacity = 1,
    #[doc = "PAN access denied."]
    PanAccessDenied = 2,
    #[doc = "Hopping sequence offset duplication."]
    HoppingDuplication = 3,
    #[doc = "Fast association successful."]
    FastAssocSuccess = 128,
}
impl From<AssociationStatusA> for u8 {
    #[inline(always)]
    fn from(variant: AssociationStatusA) -> Self {
        variant as _
    }
}
#[doc = "Field `AssociationStatus` reader - The association status after a request."]
pub struct AssociationStatusR(crate::FieldReader<u8, AssociationStatusA>);
impl AssociationStatusR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AssociationStatusR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> AssociationStatusA {
        match self.bits {
            0 => AssociationStatusA::AssocSuccess,
            1 => AssociationStatusA::PanAtCapacity,
            2 => AssociationStatusA::PanAccessDenied,
            3 => AssociationStatusA::HoppingDuplication,
            128 => AssociationStatusA::FastAssocSuccess,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `AssociationStatus` field is `AssocSuccess`"]
    #[inline(always)]
    pub fn is_assoc_success(&self) -> bool {
        **self == AssociationStatusA::AssocSuccess
    }
    #[doc = "Checks if the value of the `AssociationStatus` field is `PanAtCapacity`"]
    #[inline(always)]
    pub fn is_pan_at_capacity(&self) -> bool {
        **self == AssociationStatusA::PanAtCapacity
    }
    #[doc = "Checks if the value of the `AssociationStatus` field is `PanAccessDenied`"]
    #[inline(always)]
    pub fn is_pan_access_denied(&self) -> bool {
        **self == AssociationStatusA::PanAccessDenied
    }
    #[doc = "Checks if the value of the `AssociationStatus` field is `HoppingDuplication`"]
    #[inline(always)]
    pub fn is_hopping_duplication(&self) -> bool {
        **self == AssociationStatusA::HoppingDuplication
    }
    #[doc = "Checks if the value of the `AssociationStatus` field is `FastAssocSuccess`"]
    #[inline(always)]
    pub fn is_fast_assoc_success(&self) -> bool {
        **self == AssociationStatusA::FastAssocSuccess
    }
}
impl core::ops::Deref for AssociationStatusR {
    type Target = crate::FieldReader<u8, AssociationStatusA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct AssociationStatusW<'a> {
    w: &'a mut W,
}
impl<'a> AssociationStatusW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: AssociationStatusA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `AssociationStatus` field to `AssocSuccess`"]
    #[inline(always)]
    pub fn assoc_success(self) -> &'a mut W {
        self.variant(AssociationStatusA::AssocSuccess)
    }
    #[doc = "Set the value of the `AssociationStatus` field to `PanAtCapacity`"]
    #[inline(always)]
    pub fn pan_at_capacity(self) -> &'a mut W {
        self.variant(AssociationStatusA::PanAtCapacity)
    }
    #[doc = "Set the value of the `AssociationStatus` field to `PanAccessDenied`"]
    #[inline(always)]
    pub fn pan_access_denied(self) -> &'a mut W {
        self.variant(AssociationStatusA::PanAccessDenied)
    }
    #[doc = "Set the value of the `AssociationStatus` field to `HoppingDuplication`"]
    #[inline(always)]
    pub fn hopping_duplication(self) -> &'a mut W {
        self.variant(AssociationStatusA::HoppingDuplication)
    }
    #[doc = "Set the value of the `AssociationStatus` field to `FastAssocSuccess`"]
    #[inline(always)]
    pub fn fast_assoc_success(self) -> &'a mut W {
        self.variant(AssociationStatusA::FastAssocSuccess)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 0)) | ((value as u8 & 0xff) << 0);
        self.w
    }
}
impl R {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        R { bits }
    }
    #[doc = "Read the `AssociationStatus` field."]
    #[inline(always)]
    pub fn association_status(&self) -> AssociationStatusR {
        AssociationStatusR::new(((self.bits >> 0) & 0xff) as u8)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u8) -> Self {
        W { bits }
    }
    #[doc = "Set the `AssociationStatus` field."]
    #[inline(always)]
    pub fn association_status(&mut self) -> AssociationStatusW {
        AssociationStatusW { w: self }
    }
}
