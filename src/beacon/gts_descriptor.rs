use core2::io::{Error, Read, Write};
use defmt::Format;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(packed)]
pub struct GtsDescriptor {
    short_address: u16,
    config: u8,
}
mod gts_descriptor_fields {
    pub struct ShortAddress<'a> {
        data: &'a mut super::GtsDescriptor,
    }
    impl<'a> ShortAddress<'a> {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::GtsDescriptor) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> u16 {
            self.data.short_address
        }
        #[inline(always)]
        pub fn set(&'a mut self, v: u16) -> &'a mut super::GtsDescriptor {
            self.data.short_address = v;
            self.data
        }
    }
    pub struct Config<'a> {
        data: &'a mut super::GtsDescriptor,
    }
    impl<'a> Config<'a> {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::GtsDescriptor) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> super::super::gts_descriptor_config::R {
            super::super::gts_descriptor_config::R::new(self.data.config)
        }
        #[inline(always)]
        pub fn modify<F>(&'a mut self, f: F) -> &'a mut super::GtsDescriptor
        where
            for<'w> F: FnOnce(
                &'w mut super::super::gts_descriptor_config::W,
            ) -> &'w mut super::super::gts_descriptor_config::W,
        {
            let bits = self.data.config;
            self.data.config = **f(&mut super::super::gts_descriptor_config::W::new(bits));
            self.data
        }
    }
}
impl GtsDescriptor {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            short_address: 0,
            config: 0,
        }
    }
    pub fn short_address(&mut self) -> gts_descriptor_fields::ShortAddress {
        gts_descriptor_fields::ShortAddress::new(self)
    }
    pub fn config(&mut self) -> gts_descriptor_fields::Config {
        gts_descriptor_fields::Config::new(self)
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.short_address.to_le_bytes())?;
        out.write(&self.config.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        reader.read_exact(&mut buffer)?;
        let short_address = u16::from_le_bytes(buffer);
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let config = u8::from_le_bytes(buffer);
        Ok(Self {
            short_address,
            config,
        })
    }
}
