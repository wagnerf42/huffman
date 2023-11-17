use serde::{Deserialize, Serialize};

use std::{
    collections::{HashMap},
    io::Write,
};

use bitvec::vec::BitVec;

mod solution;

fn scan_char_frequencies(input: &str) -> HashMap<char, usize> {
    solution::scan_char_frequencies(input)
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, Eq, PartialEq, Ord)]
pub enum Node {
    Leaf(char),
    Internal([Box<Node>; 2]),
}

fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Box<Node> {
    solution::build_huffman_tree(frequencies)
}

fn build_codes(root: &Node) -> HashMap<char, BitVec> {
    solution::build_codes(root)
}

#[derive(Serialize, Deserialize)]
pub struct CompressedText {
    htree: Box<Node>,
    ctext: BitVec,
}

impl CompressedText {
    fn new(input: &str) -> Self {
        let frequencies = scan_char_frequencies(input);
        let htree = build_huffman_tree(&frequencies);
        let codes = build_codes(&htree);
        let ctext = compress_text(input, &codes);
        CompressedText { htree, ctext }
    }

    fn decompress(&self) -> String {
        solution::decompress(self)
    }

    fn load(file_name: &str) -> std::io::Result<Self> {
        let binary_soup = std::fs::read(file_name)?;
        Ok(bincode::deserialize(&binary_soup[..]).unwrap())
    }

    fn save_as(&self, file_name: &str) -> std::io::Result<()> {
        let mut writer = std::io::BufWriter::new(std::fs::File::create(file_name)?);
        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        writer.write_all(&encoded)?;
        Ok(())
    }
}

fn compress_text(input: &str, codes: &HashMap<char, BitVec>) -> BitVec {
    solution::compress_text(input, codes)
}

fn main() -> std::io::Result<()> {
    let input = "aabaabbcab".to_string();
    // let input = std::fs::read_to_string("pg22048.txt")?;
    let ctext = CompressedText::new(&input);
    ctext.save_as("test.ctxt")?;
    let reloaded = CompressedText::load("test.ctxt")?;
    let decompressed_input = reloaded.decompress();
    assert_eq!(&input, &decompressed_input);
    Ok(())
}
