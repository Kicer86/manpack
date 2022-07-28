
use bit_vec::BitVec;
use std::collections::{HashMap, BinaryHeap};
use std::fmt::Debug;
use std::hash::Hash;
use std::mem::size_of;


pub trait Serialize {
    fn to_bytes(self) -> BitVec;
    fn from_bytes(bytes: &BitVec) -> Self;
}

impl Serialize for usize {
    fn to_bytes(self) -> BitVec {
        let bytes = self.to_le_bytes();
        return BitVec::from_bytes(&bytes);
    }

    fn from_bytes(bits: &BitVec) -> Self {
        Self::from_le_bytes(bits.to_bytes().try_into().expect("slice with incorrect length"))
    }
}

impl Serialize for u32 {
    fn to_bytes(self) -> BitVec {
        let bytes = self.to_le_bytes();
        return BitVec::from_bytes(&bytes);
    }

    fn from_bytes(bits: &BitVec) -> Self {
        Self::from_le_bytes(bits.to_bytes().try_into().expect("slice with incorrect length"))
    }
}

impl Serialize for i32 {
    fn to_bytes(self) -> BitVec {
        let bytes = self.to_le_bytes();
        return BitVec::from_bytes(&bytes);
    }

    fn from_bytes(bits: &BitVec) -> Self {
        Self::from_le_bytes(bits.to_bytes().try_into().expect("slice with incorrect length"))
    }
}

impl Serialize for u16 {
    fn to_bytes(self) -> BitVec {
        let bytes = self.to_le_bytes();
        return BitVec::from_bytes(&bytes);
    }

    fn from_bytes(bits: &BitVec) -> Self {
        Self::from_le_bytes(bits.to_bytes().try_into().expect("slice with incorrect length"))
    }
}

impl Serialize for u8 {
    fn to_bytes(self) -> BitVec {
        let bytes = self.to_le_bytes();
        return BitVec::from_bytes(&bytes);
    }

    fn from_bytes(bits: &BitVec) -> Self {
        Self::from_le_bytes(bits.to_bytes().try_into().expect("slice with incorrect length"))
    }
}


pub fn compress<T>(data: &[T]) -> Vec<u8>
where
    T: Hash + Copy + Serialize + Debug + Ord
{
    let words = calculate_weights(data);
    let dictionary = build_dictionary(&words);
    let mut compressed_dict = compress_dictionary(&dictionary);
    let mut compressed_data = compress_data(&dictionary, data);

    let mut compressed = BitVec::new();
    compressed.append(&mut compressed_dict.len().to_bytes());
    compressed.append(&mut compressed_dict);
    compressed.append(&mut compressed_data.len().to_bytes());
    compressed.append(&mut compressed_data);

    log::trace!("Compression finished. Total size: {} bits", compressed.len());

    return compressed.to_bytes();
}


pub fn decompress<T>(data: &[u8]) -> Vec<T>
where
    T: Eq + Hash + Copy + Serialize
{
    let buf = BitVec::from_bytes(data);
    let mut buf_iter = buf.iter();

    let compressed_dict_size = extract_as::<usize>(&mut buf_iter);
    let compressed_dict = extract(&mut buf_iter, compressed_dict_size);
    let dictionary = decompress_dictionary::<T>(&mut compressed_dict.iter());

    let compressed_data_size = extract_as::<usize>(&mut buf_iter);
    let compressed_data = extract(&mut buf_iter, compressed_data_size);
    assert!(buf_iter.len() < 8);        // during compression bit stream was filled with zeroes up to multiple of 8. So there should be no more than 7 false bits left
    let data = decompress_data::<T>(&dictionary, &mut compressed_data.iter());

    return data;
}


fn extract(iter: &mut bit_vec::Iter, bits: usize) -> BitVec {

    let mut result = BitVec::new();

    assert!(iter.len() >= bits);

    for _ in 0..bits {
        let bit = iter.next().unwrap();
        result.push(bit);
    }

    result
}


fn extract_as<T>(iter: &mut bit_vec::Iter) -> T
where
    T: Serialize
{
    let value_bits = extract(iter, std::mem::size_of::<T>() * 8);

    T::from_bytes(&value_bits)
}


fn calculate_weights<T>(data: &[T]) -> HashMap<T, usize>
where
    T: Eq + Hash + Copy
{
    log::trace!("Calculating weights for {} pixels", data.len());

    let mut unique_words: HashMap<T, usize> = HashMap::new();

    for word in data {
        let count = unique_words.entry(*word).or_insert(0);
        *count += 1;
    }

    log::trace!("Found {} unique pixels", unique_words.len());

    return unique_words;
}


type Dictionary<T> = HashMap<T, BitVec>;

struct Tree<T> {
    weight: usize,
    data: Node<T>,
}

enum Node<T> {
    Leaf { value: T },
    Branch { left: Box<Node<T>>, right: Box<Node<T>> },
}

impl<T> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<T> Eq for Tree<T> { }

impl<T> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

struct SearchTree<T> {
    node: Box<SearchNode<T>>,
}

enum SearchNode<T> {
    Empty,
    Leaf { value: T, },
    Node { left: Box<SearchNode::<T>>, right: Box<SearchNode::<T>>, },
}

impl<T> SearchTree<T>
where
    T: Copy
{

    fn new() -> SearchTree<T> {
        SearchTree { node: Box::new(SearchNode::<T>::Empty), }
    }

    fn insert(&mut self, bits: &BitVec, value: T) {
        let mut current_node: *mut Box<SearchNode<T>> = &mut self.node;

        let mut i = 0;
        for bit in bits {

            i += 1;
            let is_last = i == bits.len();
            let mut current_node_type = unsafe { &mut **current_node };

            match current_node_type {
                SearchNode::Empty => {
                    let mut left = Box::new(SearchNode::<T>::Empty);
                    let mut right = Box::new(SearchNode::<T>::Empty);

                    if is_last {
                        let leaf = Box::new(SearchNode::<T>::Leaf{value: value});
                        if bit {
                            right = leaf;
                        } else {
                            left = leaf;
                        }
                    }

                    unsafe {
                        *current_node = Box::new(SearchNode::<T>::Node { left: left, right: right, });
                    }

                    current_node_type = unsafe { &mut **current_node };
                    current_node = match current_node_type {
                        SearchNode::Node{left, right} => {
                            if bit {
                                &mut *right
                            } else {
                                &mut *left
                            }
                        },
                        _ => { panic!("Node was expected") }
                    }
                },
                SearchNode::Leaf{value: _} => {
                    panic!("Cannot jump over Leaf");
                },
                SearchNode::Node{left, right} => {
                    current_node = if bit { &mut *right } else { &mut *left }
                },
            }
        }
    }

}

fn build_dictionary<T>(words: &HashMap<T, usize>) -> Dictionary<T>
where
    T: Hash + Copy + Ord
{
    log::trace!("Building dictionary for {} words", words.len());

    if words.is_empty() {
        return Dictionary::<T>::new();
    }

    // BinaryHeap is 'heap max' - root is the the maximum item.
    // here we need the oposite so std::cmp::Reverse is being used
    let mut trees: BinaryHeap::<std::cmp::Reverse::<Tree::<T>>> = BinaryHeap::with_capacity(words.len());

    // load all words as trees
    for (word, weight) in words {
        let node = Tree {
            weight: *weight,
            data: Node::<T>::Leaf { value: *word },
        };

        trees.push(std::cmp::Reverse(node));
    }

    log::trace!("Initial, flat, Huffman tree prepared");

    // build one tree from trees
    while trees.len() > 1
    {
        // merge two lightest items into one
        let l = trees.pop().unwrap();               // we are safe here - trees.len is at least 2
        let r = trees.pop().unwrap();

        let node = Node::<T>::Branch { left: Box::new(l.0.data), right: Box::new(r.0.data) };
        let tree = Tree::<T> { weight: l.0.weight + r.0.weight, data: node };

        trees.push(std::cmp::Reverse(tree));
    }

    log::trace!("Huffman tree built");

    // build dictionary from tree
    let mut dictionary = Dictionary::new();
    let tree = trees.pop().unwrap();

    parse_node(&tree.0.data, &mut dictionary, BitVec::new());

    log::trace!("Huffman tree converted into dictionary");
    let longest_code = dictionary.iter().reduce(|acc, item| { if acc.1.len() > item.1.len() { acc } else { item } });
    log::trace!("Longest code: {} bits", longest_code.expect("No entries in nonempty dictionary.").1.len());

    return dictionary;
}


fn parse_node<T>(node: &Node<T>, dict: &mut Dictionary<T>, code: BitVec)
where
    T: Eq + Hash + Copy
{
    match node {
        Node::Leaf { value } => {
            dict.insert(*value, code);
        }
        Node::Branch { left, right } => {
            let mut lc = code.clone();
            let mut rc = code.clone();

            lc.push(false);
            rc.push(true);

            parse_node(&left, dict, lc);
            parse_node(&right, dict, rc);
        }
    }
}


fn compress_data<T>(dict: &Dictionary<T>, data: &[T]) -> BitVec
    where
        T: Eq + Hash
{
    log::trace!("Compressing data");

    let mut output = BitVec::new();

    for v in data {
        let mut code = dict.get(v).unwrap().clone();

        output.append(&mut code);
    }

    log::trace!("Data compressed. Size: {} bits", output.len());

    return output;
}


fn decompress_data<T>(dict: &Dictionary<T>, iter: &mut bit_vec::Iter) -> Vec<T>
where
    T: Copy
{
    log::trace!("Decompressing data");

    // build AntiDictionary
    let mut anti_dictionary = HashMap::new();
    let mut search_tree = SearchTree::<T>::new();

    for (word, code) in dict {
        anti_dictionary.insert(code, word);
        search_tree.insert(code, *word);
    }

    log::trace!("AntiDictionary built");

    // extract bit by bit from buf
    let mut result: Vec<T> = Vec::new();
    let mut current_code = BitVec::new();

    for bit in iter {
        current_code.push(bit);

        let word = anti_dictionary.get(&current_code);

        if word.is_some() {
            result.push(**word.unwrap());
            current_code = BitVec::new();
        }
    }

    assert!(current_code.is_empty());

    log::trace!("Data decompressed");

    return result;
}


fn compress_dictionary<T>(dict: &Dictionary<T>) -> BitVec
where
    T: Copy + Serialize
{
    log::trace!("Compressing directory");

    let mut compressed_dict = BitVec::new();
    let words_count: u32 = dict.len() as u32;

    // save count of words as 32 bits
    compressed_dict.append(&mut words_count.to_bytes());

    // save word size as 8 bits
    let word_size: u8 = size_of::<T>() as u8;
    compressed_dict.append(&mut word_size.to_bytes());

    // save words
    for (word, _) in dict {
        compressed_dict.append(&mut word.to_bytes());
    }

    //save codes
    for (_, code) in dict {
        let code_len: u8 = code.len() as u8;
        compressed_dict.append(&mut code_len.to_bytes());
        compressed_dict.append(&mut code.clone());
    }

    log::trace!("Directory compressed. Size: {} bits", compressed_dict.len());

    return compressed_dict;
}


fn decompress_dictionary<T>(compressed_dict_iter: &mut bit_vec::Iter) -> Dictionary<T>
where
    T: Eq + Hash + Copy + Serialize
{
    log::trace!("Decompressing directory");

    let mut dictionary = Dictionary::new();

    let words_count = extract_as::<u32>(compressed_dict_iter);
    let word_size = extract_as::<u8>(compressed_dict_iter);
    assert!(word_size as usize == std::mem::size_of::<T>());

    let mut words = Vec::with_capacity(words_count as usize);

    log::trace!("Expecting {} words", words_count);

    // read words
    for _ in 0..words_count {
        let word = extract_as::<T>(compressed_dict_iter);
        words.push(word);
    }

    log::trace!("{} words read", words_count);

    // read words' codes
    for word in words {
        let code_len = extract_as::<u8>(compressed_dict_iter);
        let code = extract(compressed_dict_iter, code_len.into());
        dictionary.insert(word, code.clone());
    }

    // all data should be consumed
    assert!(compressed_dict_iter.len() == 0);

    log::trace!("Directory decompressed");

    return dictionary;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_dictionary() {

        let words: HashMap<u8, usize> =
            [
             (0, 30),
             (1, 100),
             (2, 20),
             (3, 10),
             (4, 70),
             (5, 90),
             (6, 60),
             (7, 80),
             (8, 50),
             (9, 40),
            ].iter().cloned().collect();

        let dictionary = build_dictionary(&words);
        let expected_order = vec![1, 5, 7, 4, 6, 8, 9, 0, 2, 3];

        for i in 0..9 {
            let l = expected_order[i];
            let r = expected_order[i + 1];

            assert!(dictionary.get(&l).unwrap().len() <= dictionary.get(&r).unwrap().len());
        }
    }

     #[test]
    fn test_dictionary_build_is_stable() {

        let words: HashMap<u8, usize> =
            [
             (0, 30),
             (1, 100),
             (2, 20),
             (3, 10),
             (4, 10),
             (5, 50),
             (6, 25),
             (7, 50),
             (8, 25),
             (9, 10),
            ].iter().cloned().collect();

        let dictionary = build_dictionary(&words);

        for _ in 0..5 {
            let dictionary2 = build_dictionary(&words);

            assert_eq!(dictionary, dictionary2);
        }
    }

    #[test]
    fn test_build_empty_dictionary() {

        let words: HashMap<u8, usize> = HashMap::new();
        let dictionary = build_dictionary(&words);

        assert_eq!(dictionary.len(), 0);
    }

    #[test]
    fn test_dictionary_compression_decompression() {

        let dictionary = Dictionary::from([
            ( 1, BitVec::from_bytes(&[0b01011111, 0b10100000]) ),
            ( 2, BitVec::from_fn(11, |i| { i % 2 == 0 }) ),
            ( 3, BitVec::from_bytes(&[0b01011111, 0b10100000]) ),
            ( 4, BitVec::from_fn(13, |i| { i % 3 == 0 }) ),
            ( 5, BitVec::from_bytes(&[0b11001100, 0b11110000]) ),
            ( 6, BitVec::from_fn(17, |i| { i % 5 == 0 }) ),
        ]);

        let compressed_dict = compress_dictionary(&dictionary);
        let decompressed_dict = decompress_dictionary(&mut compressed_dict.iter());

        assert_eq!(dictionary, decompressed_dict);
    }

    #[test]
    fn test_data_compression_decompression() {

        let dictionary = Dictionary::from([
            ( 1, BitVec::from_bytes(&[0b10000000]) ),
            ( 2, BitVec::from_bytes(&[0b01000000]) ),
            ( 3, BitVec::from_bytes(&[0b00100000]) ),
            ( 4, BitVec::from_bytes(&[0b00010000]) ),
            ( 5, BitVec::from_bytes(&[0b00001000]) ),
            ( 6, BitVec::from_bytes(&[0b00000100]) ),
            ( 7, BitVec::from_bytes(&[0b00000010]) ),
            ( 8, BitVec::from_bytes(&[0b00000001]) ),
        ]);

        let data = vec![1, 1, 6, 5, 4, 3, 2, 6, 5, 4, 3, 2, 1, 1, 5];

        let compressed_data = compress_data(&dictionary, &data[..]);
        let decompressed_data = decompress_data(&dictionary, &mut compressed_data.iter());

        assert_eq!(data, decompressed_data);
    }


    #[test]
    fn test_search_tree_() {

        let words: HashMap<u8, usize> =
            [
             (0, 30),
             (1, 100),
             (2, 20),
             (3, 10),
             (4, 10),
             (5, 50),
             (6, 25),
             (7, 50),
             (8, 25),
             (9, 10),
            ].iter().cloned().collect();

        let dictionary = build_dictionary(&words);
        let mut search_tree = SearchTree::new();

        for (word, code) in dictionary {
            search_tree.insert(&code, word);
        }

        //assert_eq!(data, decompressed_data);
    }
}
