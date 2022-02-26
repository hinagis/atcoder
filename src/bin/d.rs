use proconio::{input as I, fastout};

#[fastout]
fn main() {
    I! {q: usize}

    let mut v = std::collections::BTreeSet::new();
    for i in 0..q {
        I! {
            s: u8,
            x: i64
        }
        if s == 1 {
            v.insert((x, i));
        } else {
            I! {k: usize}
            println!("{}", if s == 2 {
                v.range(..(x, q)).rev().nth(k - 1).map_or(-1, |p| p.0)
            } else {
                v.range((x, 0)..).nth(k - 1).map_or(-1, |p| p.0)
            })
        }
    }
}
