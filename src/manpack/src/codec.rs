
use crate::huffman::compress;

pub fn compress_image(pixels: &[u32]) -> Vec<u8>
{
    compress(pixels)
}
