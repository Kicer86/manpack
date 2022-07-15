
use cxx;

use manpack::codec::compress_image as compress_image_r;
use manpack::codec::decompress_image as decompress_image_r;


#[cxx::bridge]
mod ffi {

    #[namespace = "rust_part"]
    extern "Rust" {
        // Functions implemented in Rust.
        fn compress_image(buf: &[u32]) -> Vec<u8>;
        fn decompress_image(buf: &[u8]) -> Vec<u32>;
    }
}


fn compress_image(buf: &[u32]) -> Vec<u8> {
    compress_image_r(buf)
}

fn decompress_image(buf: &[u8]) -> Vec<u32> {
    decompress_image_r(buf)
}
