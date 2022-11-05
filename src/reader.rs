use core::mem::size_of;

use crate::{
    frame_control::{self, DestAddrModeA, SourceAddrModeA},
    mac_frame::{AddrExtended, AddrNone, AddrShort, AddressA, PanNone, PanShort, PanidA},
};

pub struct Initial {}
pub struct FrameControl {
    fc: frame_control::R,
}
pub struct SequenceNumber {
    fc: frame_control::R,
    seqno: Option<u8>,
}
pub struct DestPan {
    fc: frame_control::R,
    pan: PanidA,
}
pub struct DestAddress {
    fc: frame_control::R,
    address: AddressA,
}
pub struct SourcePan {
    fc: frame_control::R,
    pan: PanidA,
}
pub struct SourceAddress {
    fc: frame_control::R,
    address: AddressA,
}

pub struct Reader<'a, STATE> {
    pub dat: &'a [u8],
    pub state: STATE,
}

impl<'a, STATE> Reader<'a, STATE> {
    unsafe fn read<T>(&mut self) -> &'a T {
        let (_, v, _) = self.dat.align_to::<T>();
        &v[0]
    }

    unsafe fn to_state<ST>(self, state: ST) -> Reader<'a, ST> {
        Reader {
            dat: self.dat,
            state,
        }
    }

    unsafe fn to_next_state<T: 'a, F, ST>(mut self, f: F) -> Reader<'a, ST>
    where
        F: FnOnce(&'a T) -> ST,
    {
        let t = self.read::<T>();
        Reader {
            dat: &self.dat[size_of::<T>()..],
            state: f(t),
        }
    }
}

impl<'a> Reader<'a, FrameControl> {
    pub fn new(dat: &'a [u8]) -> Self {
        let rd = Reader {
            dat,
            state: Initial {},
        };
        unsafe { rd.to_next_state(|fc: &'a frame_control::R| FrameControl { fc: fc.clone() }) }
    }

    pub fn get(&self) -> &frame_control::R {
        &self.state.fc
    }

    pub fn next(self) -> Reader<'a, SequenceNumber> {
        let fc = self.state.fc.clone();
        unsafe {
            if fc.seq_nr_suppression().bit_is_set() {
                self.to_next_state(|seq: &'a u8| SequenceNumber {
                    fc,
                    seqno: Some(seq.clone()),
                })
            } else {
                self.to_state(SequenceNumber { fc, seqno: None })
            }
        }
    }
}

impl<'a> Reader<'a, SequenceNumber> {
    pub fn get(&self) -> Option<u8> {
        self.state.seqno
    }

    pub fn next(self) -> Reader<'a, DestPan> {
        let fc = self.state.fc.clone();
        unsafe {
            if fc.dest_addr_mode().is_not_present() {
                self.to_state(DestPan {
                    fc,
                    pan: PanidA::PanNone(PanNone::new()),
                })
            } else {
                self.to_next_state(|pan: &'a PanShort| DestPan {
                    fc,
                    pan: PanidA::PanShort(pan.clone()),
                })
            }
        }
    }
}

impl<'a> Reader<'a, DestPan> {
    pub fn to_dest_addr(self) -> Reader<'a, DestAddress> {
        let fc = self.state.fc.clone();
        unsafe {
            match fc.dest_addr_mode().variant() {
                DestAddrModeA::NotPresent => self.to_next_state(|addr: &'a AddrNone| DestAddress {
                    fc,
                    address: AddressA::AddrNone(addr.clone()),
                }),
                DestAddrModeA::Address16bit => {
                    self.to_next_state(|addr: &'a AddrShort| DestAddress {
                        fc,
                        address: AddressA::AddrShort(addr.clone()),
                    })
                }
                DestAddrModeA::Address64bitExtended => {
                    self.to_next_state(|addr: &'a AddrExtended| DestAddress {
                        fc,
                        address: AddressA::AddrExtended(addr.clone()),
                    })
                }
            }
        }
    }
}

impl<'a> Reader<'a, DestPan> {
    pub fn to_source_pan(self) -> Reader<'a, SourcePan> {
        let fc = self.state.fc;

        let src_pan_present = if fc.frame_version().is_current() {
            true
        } else {
            false
        };

        unsafe {
            if src_pan_present {
                self.to_state(SourcePan {
                    fc,
                    pan: PanidA::PanNone(PanNone::new()),
                })
            } else {
                self.to_next_state(|pan: &'a PanShort| SourcePan {
                    fc,
                    pan: PanidA::PanShort(pan.clone()),
                })
            }
        }
    }
}

impl<'a> Reader<'a, SourcePan> {
    pub fn to_source_addr(self) -> Reader<'a, SourceAddress> {
        let fc = self.state.fc.clone();
        unsafe {
            match fc.source_addr_mode().variant() {
                SourceAddrModeA::NotPresent => {
                    self.to_next_state(|addr: &'a AddrNone| SourceAddress {
                        fc,
                        address: AddressA::AddrNone(addr.clone()),
                    })
                }
                SourceAddrModeA::Address16bit => {
                    self.to_next_state(|addr: &'a AddrShort| SourceAddress {
                        fc,
                        address: AddressA::AddrShort(addr.clone()),
                    })
                }
                SourceAddrModeA::Address64bitExtended => {
                    self.to_next_state(|addr: &'a AddrExtended| SourceAddress {
                        fc,
                        address: AddressA::AddrExtended(addr.clone()),
                    })
                }
            }
        }
    }
}

impl<'a> Reader<'a, SourceAddress> {}
