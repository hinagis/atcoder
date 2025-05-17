use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        h: usize,
        w: usize,
        n: usize,
    }
    let mut u = vec![0; h];
    let mut v = vec![0; w];
    let mut t = std::collections::BTreeMap::new();
    for _ in 0..n {
        I! {
            x: U,
            y: U,
        }
        u[x] += 1;
        v[y] += 1;
        t.entry(x).or_insert(vec![]).push(y);
        t.entry(y).or_insert(vec![]).push(x);
    }
    for _ in 0..q {
        I! {
            i: u8,
            a: U,
        }
        if i == 1 {
            println!("{}", u[a]);
            for j in t[&a] {
                v[j] -= 1;
            }
            t.remove(&a);
        } else {
            println!("{}", v[a]);
            for j in t[&a] {
                v[j] -= 1;
            }
            t.remove(&a);
        }
    }
}
