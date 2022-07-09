
use cxx;

use manpack::codec::compress;


#[cxx::bridge]
mod ffi {
    struct CompressedPixels {
        bytes: Vec<u8>,
        size: usize,
    }

    #[namespace = "rust_part"]
    extern "Rust" {
        // Functions implemented in Rust.
        fn compress_image(buf: &[u32]) -> CompressedPixels;
    }
}


fn compress_image(buf: &[u32]) -> ffi::CompressedPixels {
    let compressed = compress(buf);

    ffi::CompressedPixels {
        bytes: compressed.bytes,
        size: compressed.size,
    }
}
