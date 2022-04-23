#[repr(packed)]
#[derive(Clone, Copy)]
pub struct FrameCounterNone {}
impl FrameCounterNone {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameCounterPresent {
    frame_counter: u32,
}
impl FrameCounterPresent {
    pub fn new() -> Self {
        Self { frame_counter: 0 }
    }
    pub fn get(&self) -> u32 {
        self.frame_counter
    }
    pub fn set(&mut self, v: u32) -> &mut Self {
        self.frame_counter = v;
        self
    }
}
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct KeyIdNone {}
impl KeyIdNone {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyIdOnly {
    key_id: u8,
}
impl KeyIdOnly {
    pub fn new() -> Self {
        Self { key_id: 0 }
    }
    pub fn get(&self) -> u8 {
        self.key_id
    }
    pub fn set(&mut self, v: u8) -> &mut Self {
        self.key_id = v;
        self
    }
}
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct KeyIdShort {
    key_source_1: u32,
    key_id_1: u8,
}
pub struct KeySource1<'a> {
    data: &'a mut KeyIdShort,
}
impl<'a> KeySource1<'a> {
    #[inline(always)]
    pub(crate) fn new(data: &'a mut KeyIdShort) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.data.key_source_1
    }
    #[inline(always)]
    pub fn set(&'a mut self, v: u32) -> &'a mut KeyIdShort {
        self.data.key_source_1 = v;
        self.data
    }
}
pub struct KeyId1<'a> {
    data: &'a mut KeyIdShort,
}
impl<'a> KeyId1<'a> {
    #[inline(always)]
    pub(crate) fn new(data: &'a mut KeyIdShort) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> u8 {
        self.data.key_id_1
    }
    #[inline(always)]
    pub fn set(&'a mut self, v: u8) -> &'a mut KeyIdShort {
        self.data.key_id_1 = v;
        self.data
    }
}
impl KeyIdShort {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            key_source_1: 0,
            key_id_1: 0,
        }
    }
    pub fn key_source_1(&mut self) -> KeySource1 {
        KeySource1::new(self)
    }
    pub fn key_id_1(&mut self) -> KeyId1 {
        KeyId1::new(self)
    }
}
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct KeyIdLong {
    key_source_2: u64,
    key_id_2: u8,
}
pub struct KeySource2<'a> {
    data: &'a mut KeyIdLong,
}
impl<'a> KeySource2<'a> {
    #[inline(always)]
    pub(crate) fn new(data: &'a mut KeyIdLong) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.data.key_source_2
    }
    #[inline(always)]
    pub fn set(&'a mut self, v: u64) -> &'a mut KeyIdLong {
        self.data.key_source_2 = v;
        self.data
    }
}
pub struct KeyId2<'a> {
    data: &'a mut KeyIdLong,
}
impl<'a> KeyId2<'a> {
    #[inline(always)]
    pub(crate) fn new(data: &'a mut KeyIdLong) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> u8 {
        self.data.key_id_2
    }
    #[inline(always)]
    pub fn set(&'a mut self, v: u8) -> &'a mut KeyIdLong {
        self.data.key_id_2 = v;
        self.data
    }
}
impl KeyIdLong {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            key_source_2: 0,
            key_id_2: 0,
        }
    }
    pub fn key_source_2(&mut self) -> KeySource2 {
        KeySource2::new(self)
    }
    pub fn key_id_2(&mut self) -> KeyId2 {
        KeyId2::new(self)
    }
}
pub trait KeyId: Copy {
    fn default() -> Self;
}
enum KeyIdA {
    KeyIdNone,
    KeyIdOnly,
    KeyIdShort,
    KeyIdLong,
}
pub trait FrameCounterType: Copy {
    fn default() -> Self;
}
enum FrameCounterTypeA {
    FrameCounterNone,
    FrameCounterPresent,
}
impl KeyId for KeyIdNone {
    fn default() -> Self {
        Self::new()
    }
}
impl KeyId for KeyIdOnly {
    fn default() -> Self {
        Self::new()
    }
}
impl KeyId for KeyIdShort {
    fn default() -> Self {
        Self::new()
    }
}
impl KeyId for KeyIdLong {
    fn default() -> Self {
        Self::new()
    }
}
impl FrameCounterType for FrameCounterNone {
    fn default() -> Self {
        Self::new()
    }
}
impl FrameCounterType for FrameCounterPresent {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct AuxiliarySecurityHeader<FrameCounterT>
where
    FrameCounterT: FrameCounterType,
{
    security_control: u8,
    frame_counter: FrameCounterT,
}
pub struct SecurityControl<'a, FrameCounterT>
where
    FrameCounterT: FrameCounterType,
{
    data: &'a mut AuxiliarySecurityHeader<FrameCounterT>,
}
impl<'a, FrameCounterT> SecurityControl<'a, FrameCounterT>
where
    FrameCounterT: FrameCounterType,
{
    #[inline(always)]
    pub(crate) fn new(data: &'a mut AuxiliarySecurityHeader<FrameCounterT>) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> crate::security_control::R {
        crate::security_control::R::new(self.data.security_control)
    }
    #[inline(always)]
    pub fn modify<F>(&'a mut self, f: F) -> &'a mut AuxiliarySecurityHeader<FrameCounterT>
    where
        for<'w> F: FnOnce(&'w mut crate::security_control::W) -> &'w mut crate::security_control::W,
    {
        let bits = self.data.security_control;
        self.data.security_control = **f(&mut crate::security_control::W::new(bits));
        self.data
    }
}
pub struct FrameCounter<'a, FrameCounterT>
where
    FrameCounterT: FrameCounterType,
{
    data: &'a mut AuxiliarySecurityHeader<FrameCounterT>,
}
impl<'a, FrameCounterT> FrameCounter<'a, FrameCounterT>
where
    FrameCounterT: FrameCounterType,
{
    #[inline(always)]
    pub(crate) fn new(data: &'a mut AuxiliarySecurityHeader<FrameCounterT>) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn read(&self) -> FrameCounterT {
        self.data.frame_counter
    }
    #[inline(always)]
    pub fn modify<F>(&'a mut self, f: F) -> &'a mut AuxiliarySecurityHeader<FrameCounterT>
    where
        for<'w> F: FnOnce(&'w mut FrameCounterT) -> &'w mut FrameCounterT,
    {
        let mut cp = self.data.frame_counter;
        self.data.frame_counter = *f(&mut cp);
        self.data
    }
}
impl<FrameCounterT> AuxiliarySecurityHeader<FrameCounterT>
where
    FrameCounterT: FrameCounterType,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            security_control: 0,
            frame_counter: FrameCounterT::default(),
        }
    }
    pub fn security_control(&mut self) -> SecurityControl<FrameCounterT> {
        SecurityControl::new(self)
    }
    pub fn frame_counter(&mut self) -> FrameCounter<FrameCounterT> {
        FrameCounter::new(self)
    }
}
pub type AuxiliarySecurityHeaderDefault = AuxiliarySecurityHeader<FrameCounterNone>;
