use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: usize}
    let mut b = std::collections::HashMap::new();
    for _ in 0..q {
        I! {t: u8}
        if t == 3 {
            println!("{}", b.len());
            continue;
        }
        I! {x: usize}
        if t == 1 {
            *b.entry(x).or_insert(0) += 1;
        } else {
            let c = b.get(&x).unwrap();
            if *c > 1 {
                *b.get_mut(&x).unwrap() -= 1;
            } else {
                b.remove(&x);
            }
        }
    }
}
