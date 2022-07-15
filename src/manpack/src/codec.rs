
use crate::huffman::compress;

pub fn compress_image(pixels: &[u32]) -> Vec<u8>
{
    compress(pixels)
}

pub fn decompress_image(pixels: &[u8]) -> Vec<u32>
{
    Vec::new()
}
