//! Contient toutes les fonctions que vous devez écrire.
#![allow(non_snake_case, unused_imports)]

use super::Node;
use bitvec::vec::BitVec;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

/// Compte le nombre d'apparition de chaque caractère dans l'entrée.
///
/// # Retourne
///
/// Une [table de hachage](HashMap) possédant un couple (clé, valeur)
/// pour chaque caractère présent dans `input`,
/// où la clé est le caractère, et où la valeur est son nombre d'apparitions.
///
/// # Indications
///
/// Utilisez la méthode [chars](str::chars) des [chaînes de caractères](str),
/// et éventuellement la méthode [entry](HashMap::entry) des [tables de hachage](HashMap).

pub fn a__scan_char_frequencies(input: &str) -> HashMap<char, usize> {
    // Remplacez la ligne ci-dessous par votre code
    solution::a__scan_char_frequencies(input)
}

//

/// Étant donnés un texte à compresser,
/// et une table de hachage contenant le code binaire de chaque caractere,
/// génère le code binaire de de l'entrée compressée.
///
/// # Exemple
///
/// Si l'entrée est `"aabaabbcab"` et que les codes sont `0` pour `'a'`,
/// `10` pour `'b'` et `11` pour `'c'`,
/// on doit obtenir `001000101011010`.
///
/// # Note: `BitVec`
///
/// la classe [`BitVec`] est definie dans une [crate](bitvec) à part,
/// et possède une interace similaire à celle d'un [vecteurs](Vec)
/// de la bibliothèque standard dont les éléments seraient des booléens.

pub fn b__compress_text(input: &str, codes: &HashMap<char, BitVec>) -> BitVec {
    // Remplacez la ligne ci-dessous par votre code
    solution::b__compress_text(input, codes)
}

//

/// Consruit l'arbre de Huffman correspondant aux nombre d'apparitions de caractères donnés en entrée.
///
/// # Méthode
///
/// Un nœud de l'arbre (classe [Node])
/// est soit une feuille contenant un caractère,
/// soit un nœud interne contenant exactement 2 enfants.
///
/// Pour chaque caractère de l'entrée,
/// on crée un tuple composé du nombre d'apparition et d'un nœud
/// [feuille](Node::Leaf) contenant ce caractère.
/// On insère tout ce beau monde dan un tas binaire
/// (classe [BinaryHeap] de la bibliothèque standard).
///
/// Ensuite, tant que la taille du tas est supérieure à 1,
/// on extrait du tas les 2 noeuds dont les nombres d'apparitions sont les plus faibles,
/// on les fusionne en créant un nouveau nœud interne
/// (dont le nombre d'apparitions est la somme de ceux des deux nœuds extraits).
/// On insère ce nouveau nœud dans le tas.
///
/// Lorsque le tas atteint une taille de 1,
/// il contient la racine de l'arbre.
///
/// # Exemple
///
/// Si on part de a:5, b:3, c:1,
/// le tas contient initialement 3 nœuds.
/// On extrait c:1 et b:3, que l'on fusionne dans un nouveau nœud de poids 4.
/// Le tas contient désormais a:5 et (c^b):4.
/// On extrait ces deux noeuds, que l'on fusionne (en un nœud de poids 9).
/// Ce nouveau nœud est maintenant seul dans le tas, c'est la racine de l'arbre,
/// dont la structure est la suivante :
///
/// ```text
///          /\
///         /  \
///        /\   a
///       /  \
///      c    b
/// ```
///
/// # Note
///
/// Le paramètre `frequencies` peut être produit par la fonction
/// [`a__scan_char_frequencies`] précédemment écrite.

pub fn c__build_huffman_tree(frequencies: &HashMap<char, usize>) -> Box<Node> {
    // Remplacez la ligne ci-dessous par votre code
    solution::c__build_huffman_tree(frequencies)
}

//

/// Retourne, pour un arbre de Huffman,
/// une table de hachage associant à chaque caractère de l'arbre son code.
///
/// # Méthode
///
/// Pour trouver le code d'un caractère,
/// il suffit de regarder le chemin de la racine jusqu'à sa feuille.
/// Lorsqu'on part à droite on compte 1, et 0 lorsqu'on part à gauche.
///
/// # Exemple
///
/// Avec l'arbre ci-dessous, le code de `'a'` est donc `1`,
/// le code de `'b'` est `01`, et le code de `'c'` est `00`.
///
/// ```text
///          /\
///         /  \
///        /\   a
///       /  \
///      c    b
/// ```
///
/// # Note
///
/// Le paramètre d'entrée  `root` peut être produit par la fonction
/// [`c__build_huffman_tree`] précédemment écrite.
///
/// La valeur de retour de cette fonction est utilisable en entrée
/// (paramètre `codes`) de la fonction [`b__compress_text`].

pub fn d__build_codes(root: &Node) -> HashMap<char, BitVec> {
    // Remplacez la ligne ci-dessous par votre code
    solution::d__build_codes(root)
}

//

/// À partir d'un texte compressé et de l'arbre de Huffman lui correspondant,
/// reconstruit le texte original.
///
/// # Méthode
///
/// Cette fonction doit parcourir l'arbre `htree` depuis la racine;
/// en fonction des bits contenus dans `ctext`.
/// Chaque fois qu'elle atteint une feuille de l'arbre,
/// elle doit produire le caractère correspondant,
/// et repartir de la racine de l'arbre,
/// jusqu'à arriver à la fin de `ctext`.

pub fn e__decompress(ctext: &BitVec, htree: &Node) -> String {
    // Remplacez la ligne ci-dessous par votre code
    solution::e__decompress(ctext, htree)
}

//

use super::solution;
