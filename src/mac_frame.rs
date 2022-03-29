pub trait Panid {
    fn default() -> Self;
}
enum PanidA {
    PanNone,
    PanShort,
}
pub trait Address {
    fn default() -> Self;
}
enum AddressA {
    AddrNone,
    AddrShort,
    AddrExtended,
}
#[repr(packed)]
pub struct AddrNone {}
impl AddrNone {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}
#[repr(packed)]
pub struct AddrShort {
    pub address: u16,
}
impl AddrShort {
    #[inline(always)]
    pub fn new() -> Self {
        Self { address: 0 }
    }
}
#[repr(packed)]
pub struct AddrExtended {
    pub address: u32,
}
impl AddrExtended {
    #[inline(always)]
    pub fn new() -> Self {
        Self { address: 0 }
    }
}
#[repr(packed)]
pub struct PanNone {}
impl PanNone {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
}
#[repr(packed)]
pub struct PanShort {
    pub pan: u16,
}
impl PanShort {
    #[inline(always)]
    pub fn new() -> Self {
        Self { pan: 0 }
    }
}
pub struct FrameControl {
    bits: u16,
}
impl FrameControl {
    pub fn new() -> Self {
        Self { bits: 0 }
    }
    pub fn read(&self) -> crate::frame_control::R {
        crate::frame_control::R::new(self.bits)
    }
    pub fn modify<'a, F>(&'a mut self, f: F)
    where
        for<'w> F: FnOnce(&'w mut crate::frame_control::W) -> &'w mut crate::frame_control::W,
    {
        let bits = self.bits;
        self.bits = **f(&mut crate::frame_control::W::new(bits))
    }
}
#[repr(packed)]
pub struct Mhr<DestPan, DestAddress, SourcePan, SourceAddress>
where
    DestPan: Panid,
    DestAddress: Address,
    SourcePan: Panid,
    SourceAddress: Address,
{
    pub frame_control: FrameControl,
    pub sequence_number: u8,
    pub dest_pan: DestPan,
    pub dest_address: DestAddress,
    pub source_pan: SourcePan,
    pub source_address: SourceAddress,
}
impl<DestPan, DestAddress, SourcePan, SourceAddress>
    Mhr<DestPan, DestAddress, SourcePan, SourceAddress>
where
    DestPan: Panid,
    DestAddress: Address,
    SourcePan: Panid,
    SourceAddress: Address,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            frame_control: FrameControl::new(),
            sequence_number: 0,
            dest_pan: DestPan::default(),
            dest_address: DestAddress::default(),
            source_pan: SourcePan::default(),
            source_address: SourceAddress::default(),
        }
    }
}
pub type MhrDefault = Mhr<PanNone, AddrNone, PanNone, AddrNone>;
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
