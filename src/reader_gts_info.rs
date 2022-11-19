use core2::io::Read;

use crate::{
    beacon::{
        self,
        gts_info::{GtsDirA, GtsInfoGeneric},
    },
    util::read_u8,
    Error,
};

impl GtsInfoGeneric {
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut res = Self::default();

        res.gts_specification = read_u8(reader)?;
        let gts_spec = beacon::gts_specification::R::new(res.gts_specification);
        if gts_spec.descriptor_count() > 0 {
            res.gts_directions = GtsDirA::read_gts_dir_present(reader)?;
        }

        Ok(res)
    }
}
