use core2::io::{Error, Read, Write};
use defmt::Format;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameCounterPresent {
    frame_counter: u32,
}
impl FrameCounterPresent {
    pub fn new() -> Self {
        Self { frame_counter: 0 }
    }
    pub fn of_value(val: u32) -> Self {
        Self { frame_counter: val }
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
        out.write(&self.frame_counter.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        Ok(Self {
            frame_counter: u32::from_le_bytes(bytes),
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyIdOnly {
    key_id: u8,
}
impl KeyIdOnly {
    pub fn new() -> Self {
        Self { key_id: 0 }
    }
    pub fn of_value(val: u8) -> Self {
        Self { key_id: val }
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
        out.write(&self.key_id.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes = [0u8; 1];
        reader.read_exact(&mut bytes)?;
        Ok(Self {
            key_id: u8::from_le_bytes(bytes),
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(packed)]
pub struct KeyIdShort {
    key_source_1: u32,
    key_id_1: u8,
}
mod key_id_short_fields {
    pub struct KeySource1<'a> {
        data: &'a mut super::KeyIdShort,
    }
    impl<'a> KeySource1<'a> {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::KeyIdShort) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> u32 {
            self.data.key_source_1
        }
        #[inline(always)]
        pub fn set(&'a mut self, v: u32) -> &'a mut super::KeyIdShort {
            self.data.key_source_1 = v;
            self.data
        }
    }
    pub struct KeyId1<'a> {
        data: &'a mut super::KeyIdShort,
    }
    impl<'a> KeyId1<'a> {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::KeyIdShort) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> u8 {
            self.data.key_id_1
        }
        #[inline(always)]
        pub fn set(&'a mut self, v: u8) -> &'a mut super::KeyIdShort {
            self.data.key_id_1 = v;
            self.data
        }
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
    pub fn key_source_1(&mut self) -> key_id_short_fields::KeySource1 {
        key_id_short_fields::KeySource1::new(self)
    }
    pub fn key_id_1(&mut self) -> key_id_short_fields::KeyId1 {
        key_id_short_fields::KeyId1::new(self)
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.key_source_1.to_le_bytes())?;
        out.write(&self.key_id_1.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let key_source_1 = u32::from_le_bytes(buffer);
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let key_id_1 = u8::from_le_bytes(buffer);
        Ok(Self {
            key_source_1,
            key_id_1,
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(packed)]
pub struct KeyIdLong {
    key_source_2: u64,
    key_id_2: u8,
}
mod key_id_long_fields {
    pub struct KeySource2<'a> {
        data: &'a mut super::KeyIdLong,
    }
    impl<'a> KeySource2<'a> {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::KeyIdLong) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> u64 {
            self.data.key_source_2
        }
        #[inline(always)]
        pub fn set(&'a mut self, v: u64) -> &'a mut super::KeyIdLong {
            self.data.key_source_2 = v;
            self.data
        }
    }
    pub struct KeyId2<'a> {
        data: &'a mut super::KeyIdLong,
    }
    impl<'a> KeyId2<'a> {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::KeyIdLong) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> u8 {
            self.data.key_id_2
        }
        #[inline(always)]
        pub fn set(&'a mut self, v: u8) -> &'a mut super::KeyIdLong {
            self.data.key_id_2 = v;
            self.data
        }
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
    pub fn key_source_2(&mut self) -> key_id_long_fields::KeySource2 {
        key_id_long_fields::KeySource2::new(self)
    }
    pub fn key_id_2(&mut self) -> key_id_long_fields::KeyId2 {
        key_id_long_fields::KeyId2::new(self)
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.key_source_2.to_le_bytes())?;
        out.write(&self.key_id_2.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        let key_source_2 = u64::from_le_bytes(buffer);
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let key_id_2 = u8::from_le_bytes(buffer);
        Ok(Self {
            key_source_2,
            key_id_2,
        })
    }
}
pub trait FrameCounterType: Copy {
    fn default() -> Self;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub trait KeyIdentifier: Copy {
    fn default() -> Self;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyIdentifierA {
    KeyIdNone(KeyIdNone),
    KeyIdOnly(KeyIdOnly),
    KeyIdShort(KeyIdShort),
    KeyIdLong(KeyIdLong),
}
impl KeyIdentifierA {
    pub fn default() -> Self {
        Self::KeyIdNone(KeyIdNone::default())
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            KeyIdentifierA::KeyIdNone(v) => v.write(out),
            KeyIdentifierA::KeyIdOnly(v) => v.write(out),
            KeyIdentifierA::KeyIdShort(v) => v.write(out),
            KeyIdentifierA::KeyIdLong(v) => v.write(out),
        }
    }
    pub fn read_key_id_none<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(KeyIdentifierA::KeyIdNone(KeyIdNone::read(reader)?))
    }
    pub fn read_key_id_only<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(KeyIdentifierA::KeyIdOnly(KeyIdOnly::read(reader)?))
    }
    pub fn read_key_id_short<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(KeyIdentifierA::KeyIdShort(KeyIdShort::read(reader)?))
    }
    pub fn read_key_id_long<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(KeyIdentifierA::KeyIdLong(KeyIdLong::read(reader)?))
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
impl KeyIdentifier for KeyIdNone {
    fn default() -> Self {
        Self::new()
    }
}
impl KeyIdentifier for KeyIdOnly {
    fn default() -> Self {
        Self::new()
    }
}
impl KeyIdentifier for KeyIdShort {
    fn default() -> Self {
        Self::new()
    }
}
impl KeyIdentifier for KeyIdLong {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(packed)]
pub struct AuxiliarySecurityHeader<FrameCounterT, KeyIdT>
where
    FrameCounterT: FrameCounterType,
    KeyIdT: KeyIdentifier,
{
    security_control: u8,
    frame_counter: FrameCounterT,
    key_id: KeyIdT,
}
mod auxiliary_security_header_fields {
    pub struct SecurityControl<'a, FrameCounterT, KeyIdT>
    where
        FrameCounterT: super::FrameCounterType,
        KeyIdT: super::KeyIdentifier,
    {
        data: &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>,
    }
    impl<'a, FrameCounterT, KeyIdT> SecurityControl<'a, FrameCounterT, KeyIdT>
    where
        FrameCounterT: super::FrameCounterType,
        KeyIdT: super::KeyIdentifier,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> super::super::security_control::R {
            super::super::security_control::R::new(self.data.security_control)
        }
        #[inline(always)]
        pub fn modify<F>(
            &'a mut self,
            f: F,
        ) -> &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>
        where
            for<'w> F: FnOnce(
                &'w mut super::super::security_control::W,
            ) -> &'w mut super::super::security_control::W,
        {
            let bits = self.data.security_control;
            self.data.security_control = **f(&mut super::super::security_control::W::new(bits));
            self.data
        }
    }
    pub struct FrameCounter<'a, FrameCounterT, KeyIdT>
    where
        FrameCounterT: super::FrameCounterType,
        KeyIdT: super::KeyIdentifier,
    {
        data: &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>,
    }
    impl<'a, FrameCounterT, KeyIdT> FrameCounter<'a, FrameCounterT, KeyIdT>
    where
        FrameCounterT: super::FrameCounterType,
        KeyIdT: super::KeyIdentifier,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> FrameCounterT {
            self.data.frame_counter
        }
        #[inline(always)]
        pub fn modify<F>(
            &'a mut self,
            f: F,
        ) -> &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>
        where
            for<'w> F: FnOnce(&'w mut FrameCounterT) -> &'w mut FrameCounterT,
        {
            let mut cp = self.data.frame_counter;
            self.data.frame_counter = *f(&mut cp);
            self.data
        }
    }
    pub struct KeyId<'a, FrameCounterT, KeyIdT>
    where
        FrameCounterT: super::FrameCounterType,
        KeyIdT: super::KeyIdentifier,
    {
        data: &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>,
    }
    impl<'a, FrameCounterT, KeyIdT> KeyId<'a, FrameCounterT, KeyIdT>
    where
        FrameCounterT: super::FrameCounterType,
        KeyIdT: super::KeyIdentifier,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> KeyIdT {
            self.data.key_id
        }
        #[inline(always)]
        pub fn modify<F>(
            &'a mut self,
            f: F,
        ) -> &'a mut super::AuxiliarySecurityHeader<FrameCounterT, KeyIdT>
        where
            for<'w> F: FnOnce(&'w mut KeyIdT) -> &'w mut KeyIdT,
        {
            let mut cp = self.data.key_id;
            self.data.key_id = *f(&mut cp);
            self.data
        }
    }
}
impl<FrameCounterT, KeyIdT> AuxiliarySecurityHeader<FrameCounterT, KeyIdT>
where
    FrameCounterT: FrameCounterType,
    KeyIdT: KeyIdentifier,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            security_control: 0,
            frame_counter: FrameCounterT::default(),
            key_id: KeyIdT::default(),
        }
    }
    pub fn security_control(
        &mut self,
    ) -> auxiliary_security_header_fields::SecurityControl<FrameCounterT, KeyIdT> {
        auxiliary_security_header_fields::SecurityControl::new(self)
    }
    pub fn frame_counter(
        &mut self,
    ) -> auxiliary_security_header_fields::FrameCounter<FrameCounterT, KeyIdT> {
        auxiliary_security_header_fields::FrameCounter::new(self)
    }
    pub fn key_id(&mut self) -> auxiliary_security_header_fields::KeyId<FrameCounterT, KeyIdT> {
        auxiliary_security_header_fields::KeyId::new(self)
    }
}
pub struct AuxiliarySecurityHeaderGeneric {
    pub security_control: u8,
    pub frame_counter: FrameCounterTypeA,
    pub key_id: KeyIdentifierA,
}
impl AuxiliarySecurityHeaderGeneric {
    pub fn default() -> Self {
        Self {
            security_control: 0,
            frame_counter: FrameCounterTypeA::default(),
            key_id: KeyIdentifierA::default(),
        }
    }
    pub unsafe fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.security_control.to_le_bytes())?;
        self.frame_counter.write(out)?;
        self.key_id.write(out)?;
        Ok(())
    }
}
pub type AuxiliarySecurityHeaderDefault = AuxiliarySecurityHeader<FrameCounterNone, KeyIdNone>;
