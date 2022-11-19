use core2::io::{Error, Read};

use crate::{
    auxiliary_security_header::{
        AuxiliarySecurityHeaderGeneric, FrameCounterTypeA, KeyIdentifierA,
    },
    security_control,
    util::read_u8,
};

impl AuxiliarySecurityHeaderGeneric {
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut sec_header = AuxiliarySecurityHeaderGeneric::default();
        sec_header.security_control = read_u8(reader)?;
        let control_r = security_control::R::new(sec_header.security_control);
        if control_r.frame_counter_suppresion().bit_is_clear() {
            sec_header.frame_counter = FrameCounterTypeA::read_frame_counter_present(reader)?;
        }
        match control_r.key_identifier_mode().variant() {
            security_control::KeyIdentifierModeA::Implicit => {}
            security_control::KeyIdentifierModeA::KeyIndex => {
                sec_header.key_id = KeyIdentifierA::read_key_id_only(reader)?;
            }
            security_control::KeyIdentifierModeA::KeySource4 => {
                sec_header.key_id = KeyIdentifierA::read_key_id_short(reader)?;
            }
            security_control::KeyIdentifierModeA::KeySource8 => {
                sec_header.key_id = KeyIdentifierA::read_key_id_long(reader)?;
            }
        }
        Ok(sec_header)
    }
}
