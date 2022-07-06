
use std::slice;

use crate::codec::compress;


#[no_mangle]
pub extern fn compressImage(pixels: *const u32, count: usize) -> u32
{
    let vec: Vec<u32>;

    unsafe {
        assert!(!pixels.is_null());

        vec = slice::from_raw_parts(pixels, count).to_vec();
    }

    return compress(&vec);
}
