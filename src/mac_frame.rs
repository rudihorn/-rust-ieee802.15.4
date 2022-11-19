use core2::io::{Error, Read, Write};
use defmt::Format;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddrNone {}
impl AddrNone {
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
pub struct AddrShort {
    address: u16,
}
impl AddrShort {
    pub fn new() -> Self {
        Self { address: 0 }
    }
    pub fn of_value(val: u16) -> Self {
        Self { address: val }
    }
    pub fn get(&self) -> u16 {
        self.address
    }
    pub fn set(&mut self, v: u16) -> &mut Self {
        self.address = v;
        self
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.address.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes = [0u8; 2];
        reader.read_exact(&mut bytes)?;
        Ok(Self {
            address: u16::from_le_bytes(bytes),
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddrExtended {
    address: u64,
}
impl AddrExtended {
    pub fn new() -> Self {
        Self { address: 0 }
    }
    pub fn of_value(val: u64) -> Self {
        Self { address: val }
    }
    pub fn get(&self) -> u64 {
        self.address
    }
    pub fn set(&mut self, v: u64) -> &mut Self {
        self.address = v;
        self
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.address.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        Ok(Self {
            address: u64::from_le_bytes(bytes),
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PanNone {}
impl PanNone {
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
pub struct PanShort {
    pan: u16,
}
impl PanShort {
    pub fn new() -> Self {
        Self { pan: 0 }
    }
    pub fn of_value(val: u16) -> Self {
        Self { pan: val }
    }
    pub fn get(&self) -> u16 {
        self.pan
    }
    pub fn set(&mut self, v: u16) -> &mut Self {
        self.pan = v;
        self
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.pan.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes = [0u8; 2];
        reader.read_exact(&mut bytes)?;
        Ok(Self {
            pan: u16::from_le_bytes(bytes),
        })
    }
}
pub trait Address: Copy {
    fn default() -> Self;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressA {
    AddrNone(AddrNone),
    AddrShort(AddrShort),
    AddrExtended(AddrExtended),
}
impl AddressA {
    pub fn default() -> Self {
        Self::AddrNone(AddrNone::default())
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            AddressA::AddrNone(v) => v.write(out),
            AddressA::AddrShort(v) => v.write(out),
            AddressA::AddrExtended(v) => v.write(out),
        }
    }
    pub fn read_addr_none<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(AddressA::AddrNone(AddrNone::read(reader)?))
    }
    pub fn read_addr_short<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(AddressA::AddrShort(AddrShort::read(reader)?))
    }
    pub fn read_addr_extended<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(AddressA::AddrExtended(AddrExtended::read(reader)?))
    }
}
pub trait Panid: Copy {
    fn default() -> Self;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanidA {
    PanNone(PanNone),
    PanShort(PanShort),
}
impl PanidA {
    pub fn default() -> Self {
        Self::PanNone(PanNone::default())
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            PanidA::PanNone(v) => v.write(out),
            PanidA::PanShort(v) => v.write(out),
        }
    }
    pub fn read_pan_none<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(PanidA::PanNone(PanNone::read(reader)?))
    }
    pub fn read_pan_short<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(PanidA::PanShort(PanShort::read(reader)?))
    }
}
impl Address for AddrNone {
    fn default() -> Self {
        Self::new()
    }
}
impl Address for AddrShort {
    fn default() -> Self {
        Self::new()
    }
}
impl Address for AddrExtended {
    fn default() -> Self {
        Self::new()
    }
}
impl Panid for PanNone {
    fn default() -> Self {
        Self::new()
    }
}
impl Panid for PanShort {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(packed)]
pub struct Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    frame_control: u16,
    sequence_number: u8,
    dest_pan: DestPanT,
    dest_address: DestAddressT,
    source_pan: SourcePanT,
    source_address: SourceAddressT,
}
mod mhr_fields {
    pub struct FrameControl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    }
    impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        FrameControl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> super::super::frame_control::R {
            super::super::frame_control::R::new(self.data.frame_control)
        }
        #[inline(always)]
        pub fn modify<F>(
            &'a mut self,
            f: F,
        ) -> &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        where
            for<'w> F: FnOnce(
                &'w mut super::super::frame_control::W,
            ) -> &'w mut super::super::frame_control::W,
        {
            let bits = self.data.frame_control;
            self.data.frame_control = **f(&mut super::super::frame_control::W::new(bits));
            self.data
        }
    }
    pub struct SequenceNumber<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    }
    impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        SequenceNumber<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> u8 {
            self.data.sequence_number
        }
        #[inline(always)]
        pub fn set(
            &'a mut self,
            v: u8,
        ) -> &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
            self.data.sequence_number = v;
            self.data
        }
    }
    pub struct DestPan<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    }
    impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        DestPan<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> DestPanT {
            self.data.dest_pan
        }
        #[inline(always)]
        pub fn modify<F>(
            &'a mut self,
            f: F,
        ) -> &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        where
            for<'w> F: FnOnce(&'w mut DestPanT) -> &'w mut DestPanT,
        {
            let mut cp = self.data.dest_pan;
            self.data.dest_pan = *f(&mut cp);
            self.data
        }
    }
    pub struct DestAddress<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    }
    impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        DestAddress<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> DestAddressT {
            self.data.dest_address
        }
        #[inline(always)]
        pub fn modify<F>(
            &'a mut self,
            f: F,
        ) -> &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        where
            for<'w> F: FnOnce(&'w mut DestAddressT) -> &'w mut DestAddressT,
        {
            let mut cp = self.data.dest_address;
            self.data.dest_address = *f(&mut cp);
            self.data
        }
    }
    pub struct SourcePan<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    }
    impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        SourcePan<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> SourcePanT {
            self.data.source_pan
        }
        #[inline(always)]
        pub fn modify<F>(
            &'a mut self,
            f: F,
        ) -> &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        where
            for<'w> F: FnOnce(&'w mut SourcePanT) -> &'w mut SourcePanT,
        {
            let mut cp = self.data.source_pan;
            self.data.source_pan = *f(&mut cp);
            self.data
        }
    }
    pub struct SourceAddress<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
    }
    impl<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        SourceAddress<'a, DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    where
        DestPanT: super::Panid,
        DestAddressT: super::Address,
        SourcePanT: super::Panid,
        SourceAddressT: super::Address,
    {
        #[inline(always)]
        pub(crate) fn new(
            data: &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>,
        ) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> SourceAddressT {
            self.data.source_address
        }
        #[inline(always)]
        pub fn modify<F>(
            &'a mut self,
            f: F,
        ) -> &'a mut super::Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
        where
            for<'w> F: FnOnce(&'w mut SourceAddressT) -> &'w mut SourceAddressT,
        {
            let mut cp = self.data.source_address;
            self.data.source_address = *f(&mut cp);
            self.data
        }
    }
}
impl<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
    Mhr<DestPanT, DestAddressT, SourcePanT, SourceAddressT>
where
    DestPanT: Panid,
    DestAddressT: Address,
    SourcePanT: Panid,
    SourceAddressT: Address,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            frame_control: 0,
            sequence_number: 0,
            dest_pan: DestPanT::default(),
            dest_address: DestAddressT::default(),
            source_pan: SourcePanT::default(),
            source_address: SourceAddressT::default(),
        }
    }
    pub fn frame_control(
        &mut self,
    ) -> mhr_fields::FrameControl<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        mhr_fields::FrameControl::new(self)
    }
    pub fn sequence_number(
        &mut self,
    ) -> mhr_fields::SequenceNumber<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        mhr_fields::SequenceNumber::new(self)
    }
    pub fn dest_pan(
        &mut self,
    ) -> mhr_fields::DestPan<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        mhr_fields::DestPan::new(self)
    }
    pub fn dest_address(
        &mut self,
    ) -> mhr_fields::DestAddress<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        mhr_fields::DestAddress::new(self)
    }
    pub fn source_pan(
        &mut self,
    ) -> mhr_fields::SourcePan<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        mhr_fields::SourcePan::new(self)
    }
    pub fn source_address(
        &mut self,
    ) -> mhr_fields::SourceAddress<DestPanT, DestAddressT, SourcePanT, SourceAddressT> {
        mhr_fields::SourceAddress::new(self)
    }
}
pub struct MhrGeneric {
    pub frame_control: u16,
    pub sequence_number: u8,
    pub dest_pan: PanidA,
    pub dest_address: AddressA,
    pub source_pan: PanidA,
    pub source_address: AddressA,
}
impl MhrGeneric {
    pub fn default() -> Self {
        Self {
            frame_control: 0,
            sequence_number: 0,
            dest_pan: PanidA::default(),
            dest_address: AddressA::default(),
            source_pan: PanidA::default(),
            source_address: AddressA::default(),
        }
    }
    pub unsafe fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.frame_control.to_le_bytes())?;
        out.write(&self.sequence_number.to_le_bytes())?;
        self.dest_pan.write(out)?;
        self.dest_address.write(out)?;
        self.source_pan.write(out)?;
        self.source_address.write(out)?;
        Ok(())
    }
}
pub type MhrDefault = Mhr<PanNone, AddrNone, PanNone, AddrNone>;
