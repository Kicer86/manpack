
use cxx;

use manpack::codec::compress_image as compress_image_r;
use manpack::codec::decompress_image as decompress_image_r;


#[cxx::bridge]
mod ffi {

    struct Image {
        width: u32,
        height: u32,
        pixels: Vec<u32>,
    }

    #[namespace = "rust_part"]
    extern "Rust" {
        // Functions implemented in Rust.
        fn compress_image(width: u32, height: u32, buf: &[u32]) -> Vec<u8>;
        fn decompress_image(buf: &[u8]) -> Image;
    }
}


fn compress_image(width: u32, height: u32, buf: &[u32]) -> Vec<u8> {
    compress_image_r(width, height, buf)
}

fn decompress_image(buf: &[u8]) -> ffi::Image {
    let data = decompress_image_r(buf);

    ffi::Image {
        width: data.0,
        height: data.1,
        pixels: data.2,
    }
}
