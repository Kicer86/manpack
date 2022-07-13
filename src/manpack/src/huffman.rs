
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use bit_vec::BitVec;
use core::fmt::Display;
use itertools::Itertools;


pub fn compress<T>(data: &[T]) -> Vec<u8>
where
    T: Eq + Hash + Copy + Display
{

    let words = calculate_weights(data);
    let dictionary = build_dictionary(&words);

    for (key, value) in &dictionary {
        println!("{}: {}", key, value.iter().format(""));
    }

    Vec::new()
}


fn calculate_weights<T>(data: &[T]) -> HashMap<T, usize>
where
    T: Eq + Hash + Copy
{

    let mut unique_words: HashMap<T, usize> = HashMap::new();

    for word in data {
        let count = unique_words.entry(*word).or_insert(0);
        *count += 1;
    }

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


fn build_dictionary<T>(words: &HashMap<T, usize>) -> Dictionary<T>
where
    T: Eq + Hash + Copy
{
    let mut trees: VecDeque::<Tree<T> > = VecDeque::with_capacity(words.len());

    // load all words as trees
    for (word, weight) in words {
        let node = Tree {
            weight: *weight,
            data: Node::<T>::Leaf { value: *word },
        };

        trees.push_back(node);
    }

    // build one tree from trees
    while trees.len() > 1
    {
        // sort by weight
        trees.make_contiguous().sort_by(|l, r| l.weight.cmp(&r.weight));

        // merge two heaviest items into one
        let l = trees.pop_front().unwrap();
        let r = trees.pop_front().unwrap();

        let node = Node::<T>::Branch { left: Box::new(l.data), right: Box::new(r.data) };
        let tree = Tree::<T> { weight: l.weight + r.weight, data: node };

        trees.push_back(tree);
    }

    // build dictionary from tree
    let mut dictionary = Dictionary::new();
    let tree = trees.pop_front().unwrap();

    parse_node(&tree.data, &mut dictionary, BitVec::new());

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
