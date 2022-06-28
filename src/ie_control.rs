#[doc = "Specifies the type of an IE header."]
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
#[doc = "Specifies the length of the IE header contents."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthA {}
impl From<LengthA> for u8 {
    #[inline(always)]
    fn from(variant: LengthA) -> Self {
        variant as _
    }
}
#[doc = "Field `Length` reader - Specifies the length of the IE header contents."]
pub struct LengthR(crate::FieldReader<u8, LengthA>);
impl LengthR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LengthR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> LengthA {
        match self.bits {
            _ => unreachable!(),
        }
    }
}
impl core::ops::Deref for LengthR {
    type Target = crate::FieldReader<u8, LengthA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct LengthW<'a> {
    w: &'a mut W,
}
impl<'a> LengthW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: LengthA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 0)) | ((value as u16 & 0x7f) << 0);
        self.w
    }
}
#[doc = "Specifies the type of the IE header."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElementIdA {
    #[doc = "Vendor Specific Header IE"]
    VendorSpecific = 0,
    #[doc = "CSL IE"]
    CslIe = 26,
    #[doc = "RIT IE"]
    RitIe = 27,
    #[doc = "DSME PAN descriptor IE"]
    DsmePan = 28,
    #[doc = "Rendezvous Time IE"]
    RendezvousTimeIe = 29,
    #[doc = "Time Correction IE"]
    TimeCorrectionIe = 30,
    #[doc = "Extended DSME PAN descriptor IE"]
    ExtDsmePan = 33,
    #[doc = "Fragment Sequence Context Description (FSCD) IE"]
    FragSeqContext = 34,
    #[doc = "Simplified Superframe Specification IE"]
    SimplSuperframe = 35,
    #[doc = "Simplified GTS Specification IE"]
    SimplGts = 36,
    #[doc = "LECIM Capabilities IE"]
    LecimCapabilities = 37,
    #[doc = "TRLE Descriptor IE"]
    TrleDescr = 38,
    #[doc = "RCC Capabilities IE"]
    Rcc = 39,
    #[doc = "RCCN Descriptor IE"]
    Rccn = 40,
    #[doc = "Global Time IE"]
    GlobalTime = 41,
    #[doc = "Assigned to external organization"]
    ExternalAna = 42,
    #[doc = "DA IE"]
    Da = 43,
    #[doc = "Header Termination 1 IE"]
    HeaderTermination1 = 126,
    #[doc = "Header Termination 2 IE"]
    HeaderTermination2 = 127,
}
impl From<ElementIdA> for u8 {
    #[inline(always)]
    fn from(variant: ElementIdA) -> Self {
        variant as _
    }
}
#[doc = "Field `ElementId` reader - Specifies the type of the IE header."]
pub struct ElementIdR(crate::FieldReader<u8, ElementIdA>);
impl ElementIdR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ElementIdR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> ElementIdA {
        match self.bits {
            0 => ElementIdA::VendorSpecific,
            26 => ElementIdA::CslIe,
            27 => ElementIdA::RitIe,
            28 => ElementIdA::DsmePan,
            29 => ElementIdA::RendezvousTimeIe,
            30 => ElementIdA::TimeCorrectionIe,
            33 => ElementIdA::ExtDsmePan,
            34 => ElementIdA::FragSeqContext,
            35 => ElementIdA::SimplSuperframe,
            36 => ElementIdA::SimplGts,
            37 => ElementIdA::LecimCapabilities,
            38 => ElementIdA::TrleDescr,
            39 => ElementIdA::Rcc,
            40 => ElementIdA::Rccn,
            41 => ElementIdA::GlobalTime,
            42 => ElementIdA::ExternalAna,
            43 => ElementIdA::Da,
            126 => ElementIdA::HeaderTermination1,
            127 => ElementIdA::HeaderTermination2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `ElementId` field is `VendorSpecific`"]
    #[inline(always)]
    pub fn is_vendor_specific(&self) -> bool {
        **self == ElementIdA::VendorSpecific
    }
    #[doc = "Checks if the value of the `ElementId` field is `CslIe`"]
    #[inline(always)]
    pub fn is_csl_ie(&self) -> bool {
        **self == ElementIdA::CslIe
    }
    #[doc = "Checks if the value of the `ElementId` field is `RitIe`"]
    #[inline(always)]
    pub fn is_rit_ie(&self) -> bool {
        **self == ElementIdA::RitIe
    }
    #[doc = "Checks if the value of the `ElementId` field is `DsmePan`"]
    #[inline(always)]
    pub fn is_dsme_pan(&self) -> bool {
        **self == ElementIdA::DsmePan
    }
    #[doc = "Checks if the value of the `ElementId` field is `RendezvousTimeIe`"]
    #[inline(always)]
    pub fn is_rendezvous_time_ie(&self) -> bool {
        **self == ElementIdA::RendezvousTimeIe
    }
    #[doc = "Checks if the value of the `ElementId` field is `TimeCorrectionIe`"]
    #[inline(always)]
    pub fn is_time_correction_ie(&self) -> bool {
        **self == ElementIdA::TimeCorrectionIe
    }
    #[doc = "Checks if the value of the `ElementId` field is `ExtDsmePan`"]
    #[inline(always)]
    pub fn is_ext_dsme_pan(&self) -> bool {
        **self == ElementIdA::ExtDsmePan
    }
    #[doc = "Checks if the value of the `ElementId` field is `FragSeqContext`"]
    #[inline(always)]
    pub fn is_frag_seq_context(&self) -> bool {
        **self == ElementIdA::FragSeqContext
    }
    #[doc = "Checks if the value of the `ElementId` field is `SimplSuperframe`"]
    #[inline(always)]
    pub fn is_simpl_superframe(&self) -> bool {
        **self == ElementIdA::SimplSuperframe
    }
    #[doc = "Checks if the value of the `ElementId` field is `SimplGts`"]
    #[inline(always)]
    pub fn is_simpl_gts(&self) -> bool {
        **self == ElementIdA::SimplGts
    }
    #[doc = "Checks if the value of the `ElementId` field is `LecimCapabilities`"]
    #[inline(always)]
    pub fn is_lecim_capabilities(&self) -> bool {
        **self == ElementIdA::LecimCapabilities
    }
    #[doc = "Checks if the value of the `ElementId` field is `TrleDescr`"]
    #[inline(always)]
    pub fn is_trle_descr(&self) -> bool {
        **self == ElementIdA::TrleDescr
    }
    #[doc = "Checks if the value of the `ElementId` field is `Rcc`"]
    #[inline(always)]
    pub fn is_rcc(&self) -> bool {
        **self == ElementIdA::Rcc
    }
    #[doc = "Checks if the value of the `ElementId` field is `Rccn`"]
    #[inline(always)]
    pub fn is_rccn(&self) -> bool {
        **self == ElementIdA::Rccn
    }
    #[doc = "Checks if the value of the `ElementId` field is `GlobalTime`"]
    #[inline(always)]
    pub fn is_global_time(&self) -> bool {
        **self == ElementIdA::GlobalTime
    }
    #[doc = "Checks if the value of the `ElementId` field is `ExternalAna`"]
    #[inline(always)]
    pub fn is_external_ana(&self) -> bool {
        **self == ElementIdA::ExternalAna
    }
    #[doc = "Checks if the value of the `ElementId` field is `Da`"]
    #[inline(always)]
    pub fn is_da(&self) -> bool {
        **self == ElementIdA::Da
    }
    #[doc = "Checks if the value of the `ElementId` field is `HeaderTermination1`"]
    #[inline(always)]
    pub fn is_header_termination_1(&self) -> bool {
        **self == ElementIdA::HeaderTermination1
    }
    #[doc = "Checks if the value of the `ElementId` field is `HeaderTermination2`"]
    #[inline(always)]
    pub fn is_header_termination_2(&self) -> bool {
        **self == ElementIdA::HeaderTermination2
    }
}
impl core::ops::Deref for ElementIdR {
    type Target = crate::FieldReader<u8, ElementIdA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct ElementIdW<'a> {
    w: &'a mut W,
}
impl<'a> ElementIdW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: ElementIdA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `ElementId` field to `VendorSpecific`"]
    #[inline(always)]
    pub fn vendor_specific(self) -> &'a mut W {
        self.variant(ElementIdA::VendorSpecific)
    }
    #[doc = "Set the value of the `ElementId` field to `CslIe`"]
    #[inline(always)]
    pub fn csl_ie(self) -> &'a mut W {
        self.variant(ElementIdA::CslIe)
    }
    #[doc = "Set the value of the `ElementId` field to `RitIe`"]
    #[inline(always)]
    pub fn rit_ie(self) -> &'a mut W {
        self.variant(ElementIdA::RitIe)
    }
    #[doc = "Set the value of the `ElementId` field to `DsmePan`"]
    #[inline(always)]
    pub fn dsme_pan(self) -> &'a mut W {
        self.variant(ElementIdA::DsmePan)
    }
    #[doc = "Set the value of the `ElementId` field to `RendezvousTimeIe`"]
    #[inline(always)]
    pub fn rendezvous_time_ie(self) -> &'a mut W {
        self.variant(ElementIdA::RendezvousTimeIe)
    }
    #[doc = "Set the value of the `ElementId` field to `TimeCorrectionIe`"]
    #[inline(always)]
    pub fn time_correction_ie(self) -> &'a mut W {
        self.variant(ElementIdA::TimeCorrectionIe)
    }
    #[doc = "Set the value of the `ElementId` field to `ExtDsmePan`"]
    #[inline(always)]
    pub fn ext_dsme_pan(self) -> &'a mut W {
        self.variant(ElementIdA::ExtDsmePan)
    }
    #[doc = "Set the value of the `ElementId` field to `FragSeqContext`"]
    #[inline(always)]
    pub fn frag_seq_context(self) -> &'a mut W {
        self.variant(ElementIdA::FragSeqContext)
    }
    #[doc = "Set the value of the `ElementId` field to `SimplSuperframe`"]
    #[inline(always)]
    pub fn simpl_superframe(self) -> &'a mut W {
        self.variant(ElementIdA::SimplSuperframe)
    }
    #[doc = "Set the value of the `ElementId` field to `SimplGts`"]
    #[inline(always)]
    pub fn simpl_gts(self) -> &'a mut W {
        self.variant(ElementIdA::SimplGts)
    }
    #[doc = "Set the value of the `ElementId` field to `LecimCapabilities`"]
    #[inline(always)]
    pub fn lecim_capabilities(self) -> &'a mut W {
        self.variant(ElementIdA::LecimCapabilities)
    }
    #[doc = "Set the value of the `ElementId` field to `TrleDescr`"]
    #[inline(always)]
    pub fn trle_descr(self) -> &'a mut W {
        self.variant(ElementIdA::TrleDescr)
    }
    #[doc = "Set the value of the `ElementId` field to `Rcc`"]
    #[inline(always)]
    pub fn rcc(self) -> &'a mut W {
        self.variant(ElementIdA::Rcc)
    }
    #[doc = "Set the value of the `ElementId` field to `Rccn`"]
    #[inline(always)]
    pub fn rccn(self) -> &'a mut W {
        self.variant(ElementIdA::Rccn)
    }
    #[doc = "Set the value of the `ElementId` field to `GlobalTime`"]
    #[inline(always)]
    pub fn global_time(self) -> &'a mut W {
        self.variant(ElementIdA::GlobalTime)
    }
    #[doc = "Set the value of the `ElementId` field to `ExternalAna`"]
    #[inline(always)]
    pub fn external_ana(self) -> &'a mut W {
        self.variant(ElementIdA::ExternalAna)
    }
    #[doc = "Set the value of the `ElementId` field to `Da`"]
    #[inline(always)]
    pub fn da(self) -> &'a mut W {
        self.variant(ElementIdA::Da)
    }
    #[doc = "Set the value of the `ElementId` field to `HeaderTermination1`"]
    #[inline(always)]
    pub fn header_termination_1(self) -> &'a mut W {
        self.variant(ElementIdA::HeaderTermination1)
    }
    #[doc = "Set the value of the `ElementId` field to `HeaderTermination2`"]
    #[inline(always)]
    pub fn header_termination_2(self) -> &'a mut W {
        self.variant(ElementIdA::HeaderTermination2)
    }
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 7)) | ((value as u16 & 0xff) << 7);
        self.w
    }
}
#[doc = "Specifies the type of the IE header."]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeA {
    #[doc = ""]
    Default = 0,
}
impl From<TypeA> for bool {
    #[inline(always)]
    fn from(variant: TypeA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Type` reader - Specifies the type of the IE header."]
pub struct TypeR(crate::FieldReader<bool, TypeA>);
impl TypeR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TypeR(crate::FieldReader::new(bits))
    }
    #[inline(always)]
    pub fn variant(&self) -> TypeA {
        match self.bits {
            false => TypeA::Default,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the `Type` field is `Default`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == TypeA::Default
    }
}
impl core::ops::Deref for TypeR {
    type Target = crate::FieldReader<bool, TypeA>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct TypeW<'a> {
    w: &'a mut W,
}
impl<'a> TypeW<'a> {
    #[inline(always)]
    pub fn variant(self, variant: TypeA) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set the value of the `Type` field to `Default`"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TypeA::Default)
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
    #[doc = "Read the `Length` field."]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(((self.bits >> 0) & 0x7f) as u8)
    }
    #[doc = "Read the `ElementId` field."]
    #[inline(always)]
    pub fn element_id(&self) -> ElementIdR {
        ElementIdR::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Read the `Type` field."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 0x8000) != 0)
    }
}
impl W {
    #[inline(always)]
    pub fn new(bits: u16) -> Self {
        W { bits }
    }
    #[doc = "Set the `Length` field."]
    #[inline(always)]
    pub fn length(&mut self) -> LengthW {
        LengthW { w: self }
    }
    #[doc = "Set the `ElementId` field."]
    #[inline(always)]
    pub fn element_id(&mut self) -> ElementIdW {
        ElementIdW { w: self }
    }
    #[doc = "Set the `Type` field."]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW {
        TypeW { w: self }
    }
}
