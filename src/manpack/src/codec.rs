
use crate::huffman::{compress, decompress};

pub fn compress_image(width: u32, height: u32, pixels: &[u32]) -> Vec<u8>
{
    let mut compressed_image: Vec<u8> = Vec::new();
    compressed_image.append(&mut width.to_le_bytes().to_vec());
    compressed_image.append(&mut height.to_le_bytes().to_vec());
    compressed_image.append(&mut compress(pixels));

    return compressed_image;
}

pub fn decompress_image(compressed_pixels: &[u8]) -> (u32, u32, Vec<u32>)
{
    let width = u32::from_le_bytes(compressed_pixels[0..=3].try_into().expect("input stream too small"));
    let height = u32::from_le_bytes(compressed_pixels[4..=7].try_into().expect("input stream too small"));
    let pixels = decompress(&compressed_pixels[8..]);

    return (width, height, pixels);
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_image_compression_decompression() {
        let imageInt = vec![1, 2, 3, 4,  1, 2, 3, 4,  5, 6, 7, 8,  5, 6, 7, 8,  1, 3, 5, 7];
        let image: Vec<u32> = imageInt.into();

        let compressed = compress_image(4, 5, &image);
        let decompressed = decompress_image(&compressed);

        assert_eq!(decompressed.0, 4);     // width
        assert_eq!(decompressed.1, 5);     // height
        assert_eq!(decompressed.2, image); // pixels
    }
}
