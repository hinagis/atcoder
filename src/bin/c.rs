use proconio::{input as I, fastout};

#[fastout]
fn main() {
    I! {n: usize, q: usize}

    let mut h = std::collections::HashMap::new();
    for i in 1..=n {
        I! {a: u32}
        h.entry(a).or_insert(vec![]).push(i);
    }

    for _ in 0..q {
        I! {x: u32, k: usize}
        if let Some(v) = h.get(&x) {
            if v.len() >= k {
                println!("{}", v[k - 1])
            } else {
                println!("-1")
            }
        } else {
            println!("-1")
        }
    }
}
