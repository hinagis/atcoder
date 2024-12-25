use proconio::input as I;

fn main() {
    I! {n: usize}
    let mut h = std::collections::HashSet::new();
    for _ in 0..n {
        I! {l: usize, a: [u32; l]}
        h.insert(a);
    }
    println!("{}", h.len());
}
