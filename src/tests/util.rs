use core::{mem::size_of, slice::from_raw_parts};

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    from_raw_parts((p as *const T) as *const u8, size_of::<T>())
}
