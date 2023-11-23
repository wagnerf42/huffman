0) Lancer `cargo test` pour tester les differentes fonctions ;
   `cargo test <nom_de_fonction>` permet de ne tester que la fonction en question.

   Une fois toutes les fonctions implementees, `cargo run` permet de tester l'ensemble:
   * `cargo run compress <source_file> <destination_file>` pour compresser un fichier
   * `cargo run decompress <compressed_file> [destination_file]` pour decompresser un fichier (affiche le resultat sur la sortie standard en l'absence de fichier destination)


1) ```rust
   fn scan_char_frequencies(input: &str) -> HashMap<char, usize>
   ````

   On cherche a compter le nombre d'apparition de chaque caractere dans l'entree.

   Indications: utiliser la methode
   [chars](https://doc.rust-lang.org/stable/std/primitive.str.html#method.chars)
   des chaines de caracteres, et eventuellement la methode
   [entry](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html#method.entry)
   des tables de hachage.


2) ```rust
   fn compress_text(input: &str, codes: &HashMap<char, BitVec>) -> BitVec
   ```

   Etant donne un texte a compresser,
   et une table de hachage contenant le code binaire de chaque caractere,
   generer le code binaire de de l'entree compressee.

   Par exemple, si l'entree est `aabaabbcab`
   et que les codes sont `0` pour `'a'`, `10` pour `'b'` et `11` pour `'c'`,
   on doit obtenir `001000101011010`.

   La classe *BitVec* est definie dans une [crate](https://docs.rs/bitvec) a part.
   Elle possede une interface similiaire a celle d'un
   [vecteur](https://doc.rust-lang.org/std/vec/struct.Vec.html)
   de booleens.


3) ```rust
   fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Box<Node>
   ````

   Etant donne le nombre d'apparitions de chaque caractere dans le texte,
   on peut construire un arbre de huffman de la maniere suivante.

   Un node (classe `Node` fournie) est soit une feuille contenant un caractere,
   soit un noeud interne contenant exactement 2 enfants.

   Pour chaque caractere de l'entree,
   on cree un tuple compose du nombre d'apparitions et d'un noeud feuille contenance ce caractere.
   On insere tout ce beau monde dans un tas binaire
   (classes [BinaryHeap](https://doc.rust-lang.org/std/collections/binary_heap/struct.BinaryHeap.html)).

   Ensuite, tant que la taille du tas est superieure a 1,
   on en extrait les 2 noeuds avec le plus petit nombre d'apparitions,
   on les fusionne en creant un nouveau noeud interne.
   On insere ce nouveau noeud dans le tas,
   avec pour nombre d'apparition la somme de ceux de ses enfants.
   

   Exemple : s on part de a:5, b:3 et c:1,
   on va commencer par creer les trois feuilles correspondantes.
   + On extrait du tas c:1 et b:3, que l'on fusionne dans un nouveau noeud de poids 4.
   + Le tas contient maintenant a:5 et (c^b):4.
   + On extrait ces deux noeuds, que l'on fusionne (en un noeud de poids 9).
   + Ce nouveau noeud est maintenant le seul dans le tas,
     c'est la racine de l'arbre, dont la struture est la suivante :
   ```
         /\
        /  \
       /\   a
      /  \
     c    b
   ```


4) ```rust
   fn build_codes(root: &Node) -> HashMap<char, BitVec>
   ````

   Cette fonction produit une table de hachage associant chaque caractere present dans l'arbre a son code.

   Pour trouver le code d'un caractere il suffit de regarder le chemin de la racine jusqu'a
sa feuille. lorsque l'on part a droite on compte `1` et `0` lorsque l'on part a gauche.

   Exemple: avec l'abre ci-dessus, on a donc `'a'`` qui est code par `1`,
   `'b'``par `01` et `'c'` par `00`.


5) ```rust
   fn decompress(ctext: &BitVec, root: &Node) -> String
   ````

   Cette fonction reconstruit le texte original a partir de son code et de l'arbre de Huffman.

   Elle doit parcourir l'arbe depuis la raine,
   en fonction des bits contenus dans `ctext`.
   Chaque fois qu'elle atteint une feuille de l'arbre,
   elle doit ajouter le catactere correspondant dans sa sortie,
   et repartir de la racine de l'arbre,
   jusqu'a arriver a la fin de `ctext`.
