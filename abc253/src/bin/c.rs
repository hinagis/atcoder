use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: usize}
    let mut t = std::collections::BTreeMap::new();
    for _ in 0..q {
        I! {k: u8}
        if k == 3 {
            println!("{}", t.keys().last().unwrap() - t.keys().next().unwrap());
        } else {
            I! {x: u32}
            if k == 2 {
                I! {c: usize}
                let p = {
                    let p = t.entry(x).or_insert(0);
                    *p -= c.min(*p);
                    *p
                };
                if p == 0 {
                    t.remove(&x);
                }
            } else {
                *t.entry(x).or_insert(0) += 1;
            }
        }
    }
}
