
use std::collections::HashMap;
use std::iter::FromIterator;
use bit_vec::BitVec;
use huffman_compress::{CodeBuilder};

use crate::common::CompressedPixels;


pub fn compress(pixels: &[u32]) -> CompressedPixels
{
    let mut unique_pixels = HashMap::new();

    for pixel in pixels {
        let count = unique_pixels.entry(*pixel).or_insert(0);
        *count += 1;
    }

    let (book, _tree) = CodeBuilder::from_iter(unique_pixels).finish();

    let mut buffer = BitVec::new();
    for pixel in pixels {
        let _status = book.encode(&mut buffer, pixel);
    }

    CompressedPixels {
        bytes: buffer.to_bytes(),
        size: buffer.len(),
    }
}
