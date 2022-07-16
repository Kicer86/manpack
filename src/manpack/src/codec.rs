
use crate::huffman::{compress, decompress};

pub fn compress_image(pixels: &[u32]) -> Vec<u8>
{
    compress(pixels)
}

pub fn decompress_image(compressed_pixels: &[u8]) -> Vec<u32>
{
    decompress(compressed_pixels)
}
