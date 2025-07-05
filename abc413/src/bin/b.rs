fn main() {
    proconio::input! {
        n: usize,
        s: [String; n]
    }
    let mut h = std::collections::HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            h.insert(format!("{}{}", s[i], s[j]));
            h.insert(format!("{}{}", s[j], s[i]));
        }
    }
    println!("{}", h.len());
}
