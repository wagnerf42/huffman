fn creation_vecteur_range(depart: u32, taille: usize) -> Vec<u32> {
    (depart..).take(taille).collect()
}

fn map<F: Fn(u32) -> u32>(vec: &mut Vec<u32>, fonction: F) {
    for x in vec {
        *x = fonction(*x)
    }
}

fn produit_scalaire(s1: &[u32], s2: &[u32]) -> u32 {
    s1.iter().zip(s2).map(|(e1, e2)| *e1 * *e2).sum()
}

fn main() {
    let mut v1 = creation_vecteur_range(0, 10);
    let mut v2 = creation_vecteur_range(1, 10);
    map(&mut v1, |x| x % 2);
    map(&mut v2, |x| x % 2);
    println!("v1: {v1:?}");
    println!("v2: {v2:?}");
    let p = produit_scalaire(&v1, &v2);
    assert_eq!(p, 0);
}
