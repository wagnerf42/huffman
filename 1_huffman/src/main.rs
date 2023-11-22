use bitvec::vec::BitVec;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{self, Write},
};

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
        solution::decompress(&self.ctext, &self.htree)
    }

    fn load(file_name: &str) -> io::Result<Self> {
        let binary_soup = std::fs::read(file_name)?;
        Ok(bincode::deserialize(&binary_soup[..]).unwrap())
    }

    fn save_as(&self, file_name: &str) -> io::Result<()> {
        let mut writer = io::BufWriter::new(File::create(file_name)?);
        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        writer.write_all(&encoded)?;
        Ok(())
    }
}

fn compress_text(input: &str, codes: &HashMap<char, BitVec>) -> BitVec {
    solution::compress_text(input, codes)
}

fn main() -> io::Result<()> {
    let args: Vec<_> = args().collect();
    match &args.get(1).map(String::as_str) {
        Some("compress") => compress(&args[2..]),
        Some("decompress") => decompress(&args[2..]),
        _ => {
            panic!("expect 1st argument 'compress' or 'decompress'");
        }
    }
}

fn compress(args: &[String]) -> io::Result<()> {
    let [src, dest] = args else {
        panic!("usage: compress <source_file> <destination_file>");
    };
    let input = std::fs::read_to_string(src)?;
    let ctext = CompressedText::new(&input);
    ctext.save_as(dest)?;
    let reloaded = CompressedText::load(dest)?;
    let decompressed_input = reloaded.decompress();
    assert_eq!(&input, &decompressed_input);
    Ok(())
}

fn decompress(args: &[String]) -> io::Result<()> {
    match args {
        [src] => {
            let ctext = CompressedText::load(src)?;
            println!("{}", ctext.decompress());
        }
        [src, dest] => {
            let ctext = CompressedText::load(src)?;
            let output = ctext.decompress();
            let mut writer = io::BufWriter::new(File::create(dest)?);
            writer.write_all(output.as_bytes())?;
        }
        _ => panic!("usage: decompress <compressed_file> [destination_file]"),
    }
    Ok(())
}

#[cfg(test)]
mod test;
