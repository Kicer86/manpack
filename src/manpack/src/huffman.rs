
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use bit_vec::BitVec;


pub fn compress<T>(data: &[T]) -> Vec<u8>
where
    T: Eq + Hash + Copy
{
    let words = calculate_weights(data);
    let dictionary = build_dictionary(&words);
    let compressed_data = compress_data(&dictionary, data);

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

    if words.is_empty() {
        return Dictionary::<T>::new();
    }

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


fn compress_data<T>(dict: &Dictionary<T>, data: &[T]) -> BitVec
    where
        T: Eq + Hash
{
    let mut output = BitVec::new();

    for v in data {
        let code = dict.get(v).unwrap();

        for b in code {
            output.push(b);
        }
    }

    return output;
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
    fn test_build_empty_dictionary() {

        let words: HashMap<u8, usize> = HashMap::new();
        let dictionary = build_dictionary(&words);

        assert_eq!(dictionary.len(), 0);
    }
}
