use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        s: [String; n]
    }
    let mut h = std::collections::HashMap::new();
    for i in 0..n {
        let p = h.entry(s[i].clone()).or_insert(0);
        println!("{}{}", s[i], if *p == 0 {"".to_string()} else {"(".to_string() + &(*p).to_string() + ")"});
        *p += 1;
    }
}
