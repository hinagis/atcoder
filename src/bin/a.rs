fn main() {
    proconio::input! {a: [u32; 5]}
    let mut h = std::collections::HashMap::new();
    for &a in &a {
        *h.entry(a).or_insert(0) += 1;
    }
    for &v in h.values() {
        if v != 2 && v != 3 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
