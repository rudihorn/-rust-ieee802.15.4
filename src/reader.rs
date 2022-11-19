use core2::io::{Error, Read};

use crate::{
    frame_control::{DestAddrModeA, SourceAddrModeA},
    mac_frame::{AddressA, MhrGeneric, PanidA},
    util::{read_u16, read_u8},
};

impl MhrGeneric {
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut mhr = MhrGeneric::default();

        let frame_control = read_u16(reader)?;
        mhr.frame_control = frame_control;
        let frame_control_r = crate::frame_control::R::new(frame_control);
        if frame_control_r.seq_nr_suppression().bit_is_clear() {
            mhr.sequence_number = read_u8(reader)?;
        }

        if !frame_control_r.frame_version().is_current() {
            match frame_control_r.dest_addr_mode().variant() {
                DestAddrModeA::Address16bit => {
                    mhr.dest_pan = PanidA::read_pan_short(reader)?;
                    mhr.dest_address = AddressA::read_addr_short(reader)?;
                }
                DestAddrModeA::Address64bitExtended => {
                    mhr.dest_pan = PanidA::read_pan_short(reader)?;
                    mhr.dest_address = AddressA::read_addr_extended(reader)?;
                }
                DestAddrModeA::NotPresent => {}
            }
            if frame_control_r.pan_compression().bit_is_clear()
                && !frame_control_r.source_addr_mode().is_not_present()
            {
                mhr.source_pan = PanidA::read_pan_short(reader)?;
            }
            match frame_control_r.source_addr_mode().variant() {
                SourceAddrModeA::Address16bit => {
                    mhr.source_address = AddressA::read_addr_short(reader)?;
                }
                SourceAddrModeA::Address64bitExtended => {
                    mhr.dest_address = AddressA::read_addr_extended(reader)?;
                }
                SourceAddrModeA::NotPresent => {}
            }
        } else {
            match frame_control_r.dest_addr_mode().variant() {
                DestAddrModeA::Address16bit => {
                    if !(frame_control_r.source_addr_mode().is_not_present()
                        && frame_control_r.pan_compression().bit_is_set())
                    {
                        mhr.dest_pan = PanidA::read_pan_short(reader)?;
                    }
                    mhr.dest_address = AddressA::read_addr_short(reader)?;
                }
                DestAddrModeA::Address64bitExtended => {
                    if !(!frame_control_r.source_addr_mode().is_address_16bit()
                        && frame_control_r.pan_compression().bit_is_set())
                    {
                        mhr.dest_pan = PanidA::read_pan_short(reader)?;
                    }
                    mhr.dest_address = AddressA::read_addr_extended(reader)?;
                }
                DestAddrModeA::NotPresent => {
                    if !frame_control_r.source_addr_mode().is_not_present()
                        && frame_control_r.pan_compression().bit_is_set()
                    {
                        mhr.dest_pan = PanidA::read_pan_short(reader)?;
                    }
                }
            }

            match frame_control_r.source_addr_mode().variant() {
                SourceAddrModeA::NotPresent => {}
                SourceAddrModeA::Address16bit => {
                    if frame_control_r.pan_compression().bit_is_clear() {
                        mhr.source_pan = PanidA::read_pan_short(reader)?;
                    }
                    mhr.source_address = AddressA::read_addr_short(reader)?;
                }
                SourceAddrModeA::Address64bitExtended => {
                    if frame_control_r.pan_compression().bit_is_clear()
                        && !frame_control_r.dest_addr_mode().is_address_64bit_extended()
                    {
                        mhr.source_pan = PanidA::read_pan_short(reader)?;
                    }
                    mhr.source_address = AddressA::read_addr_extended(reader)?;
                }
            }
        }

        Ok(mhr)
    }
}
