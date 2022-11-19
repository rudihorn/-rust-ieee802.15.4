use core2::io::{Error, Read, Write};
use defmt::Format;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssocRequest {
    capability: u8,
}
mod assoc_request_fields {
    pub struct Capability<'a> {
        data: &'a mut super::AssocRequest,
    }
    impl<'a> Capability<'a> {
        #[inline(always)]
        pub(crate) fn new(data: &'a mut super::AssocRequest) -> Self {
            Self { data }
        }
        #[inline(always)]
        pub fn read(&self) -> super::super::assoc_request_capability::R {
            super::super::assoc_request_capability::R::new(self.data.capability)
        }
        #[inline(always)]
        pub fn modify<F>(&'a mut self, f: F) -> &'a mut super::AssocRequest
        where
            for<'w> F: FnOnce(
                &'w mut super::super::assoc_request_capability::W,
            ) -> &'w mut super::super::assoc_request_capability::W,
        {
            let bits = self.data.capability;
            self.data.capability = **f(&mut super::super::assoc_request_capability::W::new(bits));
            self.data
        }
    }
}
impl AssocRequest {
    #[inline(always)]
    pub fn new() -> Self {
        Self { capability: 0 }
    }
    pub fn capability(&mut self) -> assoc_request_fields::Capability {
        assoc_request_fields::Capability::new(self)
    }
    pub fn write<W>(&self, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        out.write(&self.capability.to_le_bytes())?;
        Ok(())
    }
    pub fn read<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let capability = u8::from_le_bytes(buffer);
        Ok(Self { capability })
    }
}
