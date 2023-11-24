//
//
//

#[derive(Debug)]
enum MinMaxRes<T> {
    None,
    Single(T),
    MinMax(T, T),
}

fn minmax<T: PartialOrd, I: Iterator<Item = T>>(mut iterateur: I) -> MinMaxRes<T> {
    if let Some(premier) = iterateur.next() {
        if let Some(second) = iterateur.next() {
            let (mut min, mut max) = if premier < second {
                (premier, second)
            } else {
                (second, premier)
            };

            for e in iterateur {
                if e < min {
                    min = e
                } else if e > max {
                    max = e
                }
            }
            MinMaxRes::MinMax(min, max)
        } else {
            MinMaxRes::Single(premier)
        }
    } else {
        MinMaxRes::None
    }
}

fn main() {
    let v = vec![1, 4, 0, 2, 8, 3];
    let r = minmax(v.iter().copied());
    println!("{:?}", r);

    let v: Vec<u32> = Vec::new();
    let r = minmax(v.iter().copied());
    println!("{:?}", r);

    let r = minmax(0..10);
    println!("{:?}", r);
    let v2 = vec![1.2, 2.4, 1.8, 0.4, 4.2, 1.1];
    let r = minmax(v2.iter());
    println!("{:?}", r);

    let mut v = vec![1, 1, 1];
    let r = minmax(v.iter_mut());
    match r {
        MinMaxRes::None => (),
        MinMaxRes::MinMax(min, max) => {
            *min += 2;
            *max -= 1;
        }
        MinMaxRes::Single(s) => *s += 1,
    }
    println!("v: {:?}", v);
}
