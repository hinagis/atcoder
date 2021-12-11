use proconio::{input};

fn main() {
    input! {n: usize}
    let mut d = std::collections::HashMap::new();

    let mut m = (0, String::new());
    for _ in 0..n {
        input! {s: String}
        let d = d.entry(s.clone()).or_insert(0);
        *d += 1;
        if *d > m.0 {
            m = (*d, s);
        }
    }

    println!("{}", m.1);
}
