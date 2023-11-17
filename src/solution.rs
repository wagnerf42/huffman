use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use bitvec::vec::BitVec;
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

pub use crate::{CompressedText, Node};

pub fn scan_char_frequencies(input: &str) -> HashMap<char, usize> {
    input.chars().fold(HashMap::new(), |mut h, c| {
        *h.entry(c).or_default() += 1;
        h
    })
}

/*

























































































*/

pub fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Box<Node> {
    let mut heap: BinaryHeap<(Reverse<usize>, Box<Node>)> = frequencies
        .iter()
        .map(|(c, count)| (Reverse(*count), Box::new(Node::Leaf(*c))))
        .collect();

    while let Some((s1, n1)) = heap.pop() {
        if let Some((s2, n2)) = heap.pop() {
            heap.push((Reverse(s1.0 + s2.0), Box::new(Node::Internal([n1, n2]))));
        } else {
            return n1;
        }
    }
    panic!("empty input");
}

/*












































































*/

pub fn build_codes(root: &Node) -> HashMap<char, BitVec> {
    let mut codes = HashMap::new();
    fn build_codes_recursively(
        root: &Node,
        codes: &mut HashMap<char, BitVec>,
        code_prefix: &mut BitVec,
    ) {
        match root {
            Node::Leaf(c) => {
                codes.insert(*c, code_prefix.clone());
            }
            Node::Internal(children) => {
                for (bit, child) in children.iter().enumerate() {
                    code_prefix.push(bit != 0);
                    build_codes_recursively(child, codes, code_prefix);
                    code_prefix.pop();
                }
            }
        }
    }
    build_codes_recursively(root, &mut codes, &mut BitVec::new());
    codes
}

/*





































































*/

pub fn compress_text(input: &str, codes: &HashMap<char, BitVec>) -> BitVec {
    input.chars().flat_map(|c| &codes[&c]).collect()
}

/*





























































































*/

pub fn decompress(ctext: &CompressedText) -> String {
    ctext
        .ctext
        .iter()
        .batching(|it| {
            match it
                .fold_while(&*ctext.htree, |node, bit| {
                    let next_node: &Node = match &node {
                        Node::Internal(children) => &children[*bit as usize],
                        _ => unreachable!(),
                    };
                    match &next_node {
                        Node::Leaf(_) => Done(next_node),
                        Node::Internal(_) => Continue(next_node),
                    }
                })
                .into_inner()
            {
                Node::Leaf(c) => Some(c),
                _ => None,
            }
        })
        .collect()
}
