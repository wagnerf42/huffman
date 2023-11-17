0) lancer `cargo run` pour tester le code


1) fn scan_char_frequencies(input: &str) -> HashMap<char, usize>

on cherche a compter le nombre d'apparition de chaque caractere dans l'entree.

indications: utiliser [chars](https://doc.rust-lang.org/stable/std/primitive.str.html#method.chars)
et eventuellement [entry](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html#method.entry)

2) fn compress_text(input: &str, codes: &HashMap<char, BitVec>) -> BitVec

etant donne un texte a compresser et une table de hachage contenant le code binaire de chaque caractere,
generer le code binaire de de l'entree compressee.

par exemple, si l'entree est aabaabbcab et que les codes sont '0' pour 'a', '10' pour 'b' et '11' pour c
on doit obtenir '001000101011010'.

la classe *BitVec* est definie dans une [crate](https://docs.rs/bitvec) a part.


3) fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Box<Node>

Etant donne le nombre d'apparition de chaque caractere dans le texte on peut construire
un arbre de huffman de la maniere suivante.

un node (classe *Node* ici) est soit une feuille contenant un caractere, soit un noeud interne
contenant exactement 2 enfants.

si on part de a:5, b:4, c:1 on va commencer par creer trois feuilles contenant a, b et c.
on groupe les feuilles avec leurs tailles dans des tuples et on insere tout ce beau monde
dans un tas (classe *BinaryHeap*).

a partir de la, l'algorithme est tres simple:

tant que la taille du tas est superieure a 1, on extrait les 2 noeuds de tailles minimales du tas
et on les fusionnent en creeant un nouveau noeud interne.

lorsque le tas atteint une taille de 1, alors, il contient la racine de l'arbre.

dans notre exemple on obtient l'arbre suivant :

```
            /\
           /  \
          a   /\
             /  \
            b    c
```


4) fn build_codes(root: &Node) -> HashMap<char, BitVec>

Pour trouver le code d'un caractere il suffit de regarder le chemin de la racine jusqu'a
sa feuille. lorsque l'on part a droite on compte '1' et '0' lorsque l'on part a gauche.

dans notre exemple on a donc 'a' qui est code par '0', 'b' par '10' et 'c' par '11'.


5) fn decompress(ctext: &BitVec, root: &Node) -> String

La fonction la plus dure. Etant donne le code du texte et l'arbre, reconstruire le texte
initial en parcourant l'arbre depuis la racine pour chaque caractere.
