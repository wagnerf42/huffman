
Un petit exercice pour s'echauffer.

Pas de code fourni pour l'echauffement, le but est de vous laisser
ecrire les prototypes de fonctions.

0) utiliser `cargo new` pour creer un nouveau projet

1) utiliser `cargo run` pour lancer le code du hello world qui a ete genere

2) ecrire une fonction `creation_vecteur_range` prenant deux arguments
  - un entier *e* de depart, non signe, de 32bits
  - une taille *t* (sous forme d'un usize)
  et renvoyant un vecteur *v* de *t* entiers de 32 bits non signes :
  la suite de *t* entiers a partir de *e*.
  
  par exemple creation_vecteur_range(3, 5) devra renvoyer
  le vecteur contenant [3,4,5,6,7].

3) testez votre fonction en creant un vecteur v1 de 10 elements, demarrant a 0
puis en l'affichant a l'aide d'un `println`.

4) ecrire une fonction `map` prenant en argument un vecteur d'entiers, et une fonction
*f* sur les entiers. votre fonction devra remplacer chaque entier du vecteur par son image
a travers *f*.

  par exemple si *v* contient [0,1,2,3,4,5,6,8,9] apres l'appel a map sur v avec la fonction `|x| x+1`
  *v* contiendra alors [1,2,3,4,5,6,7,8,9,10].

  la fonction passee en argument etant generique, on utilisera le trait Fn(u32) -> u32 pour contraindre
  son type.

5) creez un second vecteur *v2* contenant les entiers de 1 a 10.
  a l'aide de map, remplacez chaque entier *e* de *v* et de *v2*  par *e % 2*.
  
6) ecrire une fonction calculant le produit scalaire de 2 vecteurs
  et verifiez que le produit scalaire de *v* par *v2* vaut bien 0.
