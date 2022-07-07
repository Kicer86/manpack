
use std::collections::HashMap;
use std::iter::FromIterator;
use bit_vec::BitVec;
use huffman_compress::{CodeBuilder};


pub fn compress(pixels: &[u32]) -> usize
{
    let mut unique_pixels = HashMap::new();

    for pixel in pixels {
        let count = unique_pixels.entry(*pixel).or_insert(0);
        *count += 1;
    }

    let (book, _tree) = CodeBuilder::from_iter(unique_pixels).finish();

    let mut buffer = BitVec::new();
    for pixel in pixels {
        book.encode(&mut buffer, pixel);
    }

    return buffer.len();
}
