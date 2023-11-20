//! # Exercice: Codage de Huffman en Rust
//!
//! Cet exercice est disponible ici: <https://github.com/wagnerf42/huffman/>.
//!
//! Lancer `cargo test` pour tester les différentes fonctions ;
//! `cargo test <nom_de_fonction>` permet de ne tester que la fonction en question.
//!
//! `cargo run` permet de tester l'ensemble:
//! * `cargo run compress <source_file> <destination_file>` pour compresser un fichier
//! * `cargo run decompress <compressed_file> [destination_file]` pour décompresser un fichier
//!   (affiche le résultat sur la sortie standard en l'absence de fichier destination)
//!
//! ## Votre travail
//!
//! Implémenter, dans l'ordre où elle sont données,
//! toutes les fonctions spécifiées dans le [src/exercice.rs](exercice).
//!
//! ## Références
//!
//! [Codage de Huffman sur Wikipedia](https://fr.wikipedia.org/wiki/Codage_de_Huffman)

use bitvec::vec::BitVec;
use serde::{Deserialize, Serialize};
use std::{
    env::args,
    fs::File,
    io::{self, Write},
};

pub mod exercice;
mod solution;

/// Un nœud de l'arbre de Huffman.
///
/// Peut être une [feuille](Node::Leaf) ou un [nœud interne](Node::Internal).
#[derive(Serialize, Deserialize, Debug, PartialOrd, Eq, PartialEq, Ord)]
pub enum Node {
    /// Une feuille de l'arbre de Huffman
    Leaf(char),
    /// Un nœud interne de l'arbre de Huffman, possédant deux nœuds enfants.
    Internal([Box<Node>; 2]),
}

#[derive(Serialize, Deserialize)]
pub struct CompressedText {
    htree: Box<Node>,
    ctext: BitVec,
}

impl CompressedText {
    fn new(input: &str) -> Self {
        let frequencies = exercice::a__scan_char_frequencies(input);
        let htree = exercice::c__build_huffman_tree(&frequencies);
        let codes = exercice::d__build_codes(&htree);
        let ctext = exercice::b__compress_text(input, &codes);
        CompressedText { htree, ctext }
    }

    fn decompress(&self) -> String {
        exercice::e__decompress(&self.ctext, &self.htree)
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
