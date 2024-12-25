fn main() {
    proconio::input! {a: [u32; 5]}
    let mut h = std::collections::HashSet::new();
    for &a in &a {
        h.insert(a);
    }
    println!("{}", h.len());
}
