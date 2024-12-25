use proconio::input as I;

fn main() {
    I! {n: usize}

    let mut h = std::collections::HashSet::new();
    for _ in 0..n {
        I! {a: u32}
        h.insert(a);
    }

    println!("{}", h.len());
}
