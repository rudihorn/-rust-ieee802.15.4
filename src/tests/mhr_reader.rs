use crate::mac_frame::{AddrShort, AddressA, MhrGeneric, PanShort, PanidA};
use crate::util::SliceReader;

#[test]
pub fn test() {
    let dat: &[u8] = &[0x03, 0x08, 0x01, 0xff, 0xff, 0xff, 0xff];

    let mut reader = &mut SliceReader::new(dat);
    let mhr_generic = MhrGeneric::read(&mut reader).unwrap();

    assert_eq!(mhr_generic.frame_control, 0x0803);
    assert_eq!(mhr_generic.sequence_number, 1);
    assert_eq!(
        mhr_generic.dest_address,
        AddressA::AddrShort(AddrShort::of_value(0xffff))
    );
    assert_eq!(
        mhr_generic.dest_pan,
        PanidA::PanShort(PanShort::of_value(0xffff))
    );

    assert_eq!(reader.len(), 0);
}
