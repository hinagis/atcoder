use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        _: usize,
        m: usize
    }
    let mut r = std::collections::HashSet::new();
    for _ in 0..m {
        I! {u: U, v: U}
        let (u, v) = if u > v {(v, u)} else {(u, v)};
        if u != v {
            r.insert((u, v));
        }
    }
    println!("{}", m - r.len());
}
