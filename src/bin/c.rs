use proconio::{input as I};

fn main() {
    I! {n: usize}
    let mut m = (0, 0);
    let mut h = std::collections::HashMap::new();
    for i in 1..=n {
        I!{s: String, t: u32}
        if ! h.contains_key(&s) {
            h.insert(s, t);
            if t > m.1 {
                m = (i, t);
            }
        }
    }

    println!("{}", m.0);
}
