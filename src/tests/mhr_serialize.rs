use super::util::any_as_u8_slice;
use crate::mac_frame::{AddrNone, AddrShort, Mhr, PanNone, PanShort};

#[test]
pub fn test_write_beacon() {
    let mut mhr = Mhr::<PanShort, AddrShort, PanNone, AddrNone>::new();
    mhr.frame_control()
        .modify(|f| {
            f.frame_type()
                .mac_command()
                .source_addr_mode()
                .not_present()
                .dest_addr_mode()
                .address_16bit()
                .frame_pending()
                .no_frame_pending()
                .ack_request()
                .ack_not_requested()
                .security_enabled()
                .unencrypted()
        })
        .sequence_number()
        .set(1)
        .dest_address()
        .modify(|v| v.set(0xffff))
        .dest_pan()
        .modify(|v| v.set(0xffff));

    let dat = unsafe { any_as_u8_slice(&mhr) };

    assert_eq!(dat, &[0x03, 0x08, 0x01, 0xff, 0xff, 0xff, 0xff]);
}
