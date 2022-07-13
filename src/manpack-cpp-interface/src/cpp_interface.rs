
use cxx;

use manpack::codec::compress_image as compress_image_r;


#[cxx::bridge]
mod ffi {

    #[namespace = "rust_part"]
    extern "Rust" {
        // Functions implemented in Rust.
        fn compress_image(buf: &[u32]) -> Vec<u8>;
    }
}


fn compress_image(buf: &[u32]) -> Vec<u8> {
    compress_image_r(buf)
}
