use bitvec::vec::BitVec;
use std::collections::HashMap;
use test_case::test_case;

use crate::*;

#[test_case("foo", &[('f', 1), ('o', 2)]; "foo")]
#[test_case("hello world", &[('h', 1), ('e', 1), ('l', 3), ('o', 2), (' ', 1), ('w', 1), ('r', 1), ('d', 1), ]; "hello world")]
#[test_case("aabaabbcab", &[('a', 5), ('b', 4), ('c', 1), ]; "aabaabbcab")]
fn test_scan_char_frequencies(txt: &str, exp: &[(char, usize)]) {
    let got = scan_char_frequencies(txt);
    let exp: HashMap<char, usize> = exp.iter().copied().collect();
    assert_eq!(&got, &exp);
}

#[test_case("foo", &["f 0", "o 1"], "011"; "foo_1")]
#[test_case("foo", &["f 10", "o 0"], "1000"; "foo_2")]
#[test_case("aabaabbcab", &["a 0", "b 10", "c 11"], "001000101011010"; "aabaabbcab")]
fn test_compress_text(txt: &str, codes: &[&str], exp: &str) {
    let codes = codes.iter().map(str2cb).collect();
    let got = compress_text(txt, &codes);
    let exp = str2bitvec(exp);
    assert_eq!(got, exp);
}

#[test_case(&[('f', 1), ('o', 2)], "f o ."; "foo")]
#[test_case(&[('a', 5), ('b', 4), ('c', 1)], "c b . a ."; "aabaabbcab")]
fn test_build_huffman_tree(freqs: &[(char, usize)], exp: &str) {
    let freqs = freqs.iter().copied().collect();
    let got = *build_huffman_tree(&freqs);
    let exp = str2tree(exp);
    assert_eq!(got, exp);
}

#[test_case("f o .", &["o 1", "f 0"]; "foo")]
#[test_case("a b c . .", &["a 0", "b 10", "c 11"]; "aabaabbcab_1")]
#[test_case("c b . a .", &["c 00", "b 01", "a 1"]; "aabaabbcab_2")]
fn test_build_codes(htree: &str, exp: &[&str]) {
    let htree = &str2tree(htree);
    let got = build_codes(htree);
    let exp = exp.iter().map(str2cb).collect();
    assert_eq!(got, exp);
}

#[test_case("011", "f o .", "foo"; "foo_1")]
#[test_case("100", "o f .", "foo"; "foo_2")]
#[test_case("110111010100101", "c b . a .", "aabaabbcab"; "aabaabbcab_1")]
#[test_case("001000101011010", "a b c . .", "aabaabbcab"; "aabaabbcab_2")]
fn test_decompress(ctext: &str, htree: &str, exp: &str) {
    let ctext = str2bitvec(ctext);
    let htree = str2tree(htree);
    let ct = CompressedText{ ctext, htree: Box::new(htree) };
    let got = ct.decompress();
    assert_eq!(got, exp);
}

#[test]
fn test_compressed_text_new() {
    let ctext = crate::CompressedText::new("aabaabbcab");
    let htree = str2tree("c b . a .");
    assert_eq!(&*ctext.htree, &htree);
    let bits = str2bitvec("110111010100101");
    assert_eq!(ctext.ctext, bits);
}

//

// Extract a pair (char, BitVec) from a string
//
// where the char is the first char of txt,
// and BitVec is constructed by only considering 0s and 1s in txt
fn str2cb(txt: &&str) -> (char, BitVec) {
    (txt.chars().next().unwrap(), str2bitvec(txt))
}

/// Convert txt into a BitVec, ignoring all characters different from 0 and 1
fn str2bitvec(txt: &str) -> BitVec {
    txt.chars()
        .filter(|c| *c == '0' || *c == '1')
        .map(|c| c == '1')
        .collect()
}
/// Convert a RPN string into a tree
///
/// * spaces are ignored
/// * '.' means "merge the two top nodes of the stack into a single node"
/// * any other character means "push a this new leaf to the stack"
fn str2tree(txt: &str) -> Node {
    let mut stack = vec![];
    txt.chars().filter(|c| *c != ' ').for_each(|c| match c {
        '.' => {
            assert!(stack.len() >= 2);
            let n2 = stack.pop().unwrap();
            let n1 = stack.pop().unwrap();
            stack.push(Node::Internal([Box::new(n1), Box::new(n2)]));
        }
        c => stack.push(Node::Leaf(c)),
    });
    assert_eq!(stack.len(), 1);
    stack.pop().unwrap()
}
