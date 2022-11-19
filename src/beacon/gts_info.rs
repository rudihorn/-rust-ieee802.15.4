use core2::io::{Error, Read, Write};
use defmt::Format;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GtsDirNone {}
impl GtsDirNone {
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
pub struct GtsDirPresent {
    gts_dir: u8,
}
mod gts_dir_present_fields {
    pub struct GtsDir<'a> {
        data: &'a mut super::GtsDirPresent,
    }
    impl<'a> GtsDir<'a> {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::GtsDirPresent) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> super::super::gts_directions::R {
            super::super::gts_directions::R::new(self.data.gts_dir)
        }
        #[inline(always)]
        pub fn modify<F>(&'a mut self, f: F) -> &'a mut super::GtsDirPresent
        where
            for<'w> F: FnOnce(
                &'w mut super::super::gts_directions::W,
            ) -> &'w mut super::super::gts_directions::W,
        {
            let bits = self.data.gts_dir;
            self.data.gts_dir = **f(&mut super::super::gts_directions::W::new(bits));
            self.data
        }
    }
}
impl GtsDirPresent {
    #[inline(always)]
    pub fn new() -> Self {
        Self { gts_dir: 0 }
    }
    pub fn gts_dir(&mut self) -> gts_dir_present_fields::GtsDir {
        gts_dir_present_fields::GtsDir::new(self)
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.gts_dir.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let gts_dir = u8::from_le_bytes(buffer);
        Ok(Self { gts_dir })
    }
}
pub trait GtsDir: Copy {
    fn default() -> Self;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GtsDirA {
    GtsDirNone(GtsDirNone),
    GtsDirPresent(GtsDirPresent),
}
impl GtsDirA {
    pub fn default() -> Self {
        Self::GtsDirNone(GtsDirNone::default())
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            GtsDirA::GtsDirNone(v) => v.write(out),
            GtsDirA::GtsDirPresent(v) => v.write(out),
        }
    }
    pub fn read_gts_dir_none<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(GtsDirA::GtsDirNone(GtsDirNone::read(reader)?))
    }
    pub fn read_gts_dir_present<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(GtsDirA::GtsDirPresent(GtsDirPresent::read(reader)?))
    }
}
impl GtsDir for GtsDirNone {
    fn default() -> Self {
        Self::new()
    }
}
impl GtsDir for GtsDirPresent {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(packed)]
pub struct GtsInfo<GtsDirectionsT>
where
    GtsDirectionsT: GtsDir,
{
    gts_specification: u8,
    gts_directions: GtsDirectionsT,
}
mod gts_info_fields {
    pub struct GtsSpecification<'a, GtsDirectionsT>
    where
        GtsDirectionsT: super::GtsDir,
    {
        data: &'a mut super::GtsInfo<GtsDirectionsT>,
    }
    impl<'a, GtsDirectionsT> GtsSpecification<'a, GtsDirectionsT>
    where
        GtsDirectionsT: super::GtsDir,
    {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::GtsInfo<GtsDirectionsT>) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> super::super::gts_specification::R {
            super::super::gts_specification::R::new(self.data.gts_specification)
        }
        #[inline(always)]
        pub fn modify<F>(&'a mut self, f: F) -> &'a mut super::GtsInfo<GtsDirectionsT>
        where
            for<'w> F: FnOnce(
                &'w mut super::super::gts_specification::W,
            ) -> &'w mut super::super::gts_specification::W,
        {
            let bits = self.data.gts_specification;
            self.data.gts_specification = **f(&mut super::super::gts_specification::W::new(bits));
            self.data
        }
    }
    pub struct GtsDirections<'a, GtsDirectionsT>
    where
        GtsDirectionsT: super::GtsDir,
    {
        data: &'a mut super::GtsInfo<GtsDirectionsT>,
    }
    impl<'a, GtsDirectionsT> GtsDirections<'a, GtsDirectionsT>
    where
        GtsDirectionsT: super::GtsDir,
    {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::GtsInfo<GtsDirectionsT>) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> GtsDirectionsT {
            self.data.gts_directions
        }
        #[inline(always)]
        pub fn modify<F>(&'a mut self, f: F) -> &'a mut super::GtsInfo<GtsDirectionsT>
        where
            for<'w> F: FnOnce(&'w mut GtsDirectionsT) -> &'w mut GtsDirectionsT,
        {
            let mut cp = self.data.gts_directions;
            self.data.gts_directions = *f(&mut cp);
            self.data
        }
    }
}
impl<GtsDirectionsT> GtsInfo<GtsDirectionsT>
where
    GtsDirectionsT: GtsDir,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            gts_specification: 0,
            gts_directions: GtsDirectionsT::default(),
        }
    }
    pub fn gts_specification(&mut self) -> gts_info_fields::GtsSpecification<GtsDirectionsT> {
        gts_info_fields::GtsSpecification::new(self)
    }
    pub fn gts_directions(&mut self) -> gts_info_fields::GtsDirections<GtsDirectionsT> {
        gts_info_fields::GtsDirections::new(self)
    }
}
pub struct GtsInfoGeneric {
    pub gts_specification: u8,
    pub gts_directions: GtsDirA,
}
impl GtsInfoGeneric {
    pub fn default() -> Self {
        Self {
            gts_specification: 0,
            gts_directions: GtsDirA::default(),
        }
    }
    pub unsafe fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.gts_specification.to_le_bytes())?;
        self.gts_directions.write(out)?;
        Ok(())
    }
}
pub type GtsInfoDefault = GtsInfo<GtsDirNone>;
