use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        m: usize,
        s: [C; n]
    }
    let mut h = std::collections::HashSet::new();
    for i in 0..=n - m {
        for j in 0..=n - m {
            let mut t = String::with_capacity(m * m);
            for u in 0..m {
                for v in 0..m {
                    t.push(s[i + u][j + v]);
                }
            }
            h.insert(t);
        }
    }
    println!("{}", h.len());
}
