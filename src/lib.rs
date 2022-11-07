#![no_std]

pub mod auxiliary_security_header;
pub mod frame_control;
pub mod generic;
pub mod ie_control;
pub mod mac_frame;
pub mod security_control;

pub mod beacon;
pub mod reader;
pub mod util;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
use generic::*;
use mac_frame::{AddrNone, AddrShort, PanNone};

pub fn test() -> frame_control::W {
    let mut v = frame_control::W::new(0);
    v.frame_type()
        .data()
        .ack_request()
        .ack_requested()
        .dest_addr_mode()
        .address_16bit();
    v
}

pub fn test2() {
    let _ = mac_frame::MhrDefault::new();
    let mut v = mac_frame::Mhr::<PanNone, AddrShort, PanNone, AddrNone>::new();
    v.frame_control()
        .modify(|v| {
            v.ack_request()
                .ack_not_requested()
                .dest_addr_mode()
                .address_16bit()
                .pan_compression()
                .compressed()
                .source_addr_mode()
                .not_present()
        })
        .frame_control()
        .modify(|v| v.frame_pending().no_frame_pending())
        .dest_address()
        .modify(|v| v.set(1234));
}
