use core2::io::{Error, Read, Write};
#[derive(Copy, Clone)]
#[repr(packed)]
pub struct FrameCounterNone {}
impl FrameCounterNone {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
    pub fn write<W>(&self, _out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        Ok(())
    }
    pub fn read<R>(_reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {})
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
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.frame_counter.to_be_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        Ok(Self {
            frame_counter: u32::from_be_bytes(bytes),
        })
    }
}
#[derive(Copy, Clone)]
#[repr(packed)]
pub struct KeyIdNone {}
impl KeyIdNone {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
    pub fn write<W>(&self, _out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        Ok(())
    }
    pub fn read<R>(_reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {})
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
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.key_id.to_be_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes = [0u8; 1];
        reader.read_exact(&mut bytes)?;
        Ok(Self {
            key_id: u8::from_be_bytes(bytes),
        })
    }
}
#[derive(Copy, Clone)]
#[repr(packed)]
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
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.key_source_1.to_be_bytes())?;
        out.write(&self.key_id_1.to_be_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let key_source_1 = u32::from_be_bytes(buffer);
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let key_id_1 = u8::from_be_bytes(buffer);
        Ok(Self {
            key_source_1,
            key_id_1,
        })
    }
}
#[derive(Copy, Clone)]
#[repr(packed)]
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
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.key_source_2.to_be_bytes())?;
        out.write(&self.key_id_2.to_be_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        let key_source_2 = u64::from_be_bytes(buffer);
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let key_id_2 = u8::from_be_bytes(buffer);
        Ok(Self {
            key_source_2,
            key_id_2,
        })
    }
}
pub trait KeyId: Copy {
    fn default() -> Self;
}
pub enum KeyIdA {
    KeyIdNone(KeyIdNone),
    KeyIdOnly(KeyIdOnly),
    KeyIdShort(KeyIdShort),
    KeyIdLong(KeyIdLong),
}
impl KeyIdA {
    pub fn default() -> Self {
        Self::KeyIdNone(KeyIdNone::default())
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            KeyIdA::KeyIdNone(v) => v.write(out),
            KeyIdA::KeyIdOnly(v) => v.write(out),
            KeyIdA::KeyIdShort(v) => v.write(out),
            KeyIdA::KeyIdLong(v) => v.write(out),
        }
    }
    pub fn read_key_id_none<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(KeyIdA::KeyIdNone(KeyIdNone::read(reader)?))
    }
    pub fn read_key_id_only<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(KeyIdA::KeyIdOnly(KeyIdOnly::read(reader)?))
    }
    pub fn read_key_id_short<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(KeyIdA::KeyIdShort(KeyIdShort::read(reader)?))
    }
    pub fn read_key_id_long<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(KeyIdA::KeyIdLong(KeyIdLong::read(reader)?))
    }
}
pub trait FrameCounterType: Copy {
    fn default() -> Self;
}
pub enum FrameCounterTypeA {
    FrameCounterNone(FrameCounterNone),
    FrameCounterPresent(FrameCounterPresent),
}
impl FrameCounterTypeA {
    pub fn default() -> Self {
        Self::FrameCounterNone(FrameCounterNone::default())
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            FrameCounterTypeA::FrameCounterNone(v) => v.write(out),
            FrameCounterTypeA::FrameCounterPresent(v) => v.write(out),
        }
    }
    pub fn read_frame_counter_none<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(FrameCounterTypeA::FrameCounterNone(FrameCounterNone::read(
            reader,
        )?))
    }
    pub fn read_frame_counter_present<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(FrameCounterTypeA::FrameCounterPresent(
            FrameCounterPresent::read(reader)?,
        ))
    }
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
    pub fn read(&self) -> super::security_control::R {
        super::security_control::R::new(self.data.security_control)
    }
    #[inline(always)]
    pub fn modify<F>(&'a mut self, f: F) -> &'a mut AuxiliarySecurityHeader<FrameCounterT>
    where
        for<'w> F: FnOnce(&'w mut super::security_control::W) -> &'w mut super::security_control::W,
    {
        let bits = self.data.security_control;
        self.data.security_control = **f(&mut super::security_control::W::new(bits));
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
pub struct AuxiliarySecurityHeaderGeneric {
    pub security_control: u8,
    pub frame_counter: FrameCounterTypeA,
}
impl AuxiliarySecurityHeaderGeneric {
    pub fn default() -> Self {
        Self {
            security_control: 0,
            frame_counter: FrameCounterTypeA::default(),
        }
    }
    pub unsafe fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.security_control.to_be_bytes())?;
        self.frame_counter.write(out)?;
        Ok(())
    }
}
pub type AuxiliarySecurityHeaderDefault = AuxiliarySecurityHeader<FrameCounterNone>;
