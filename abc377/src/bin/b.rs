use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: [C; 8]}
    let (mut r, mut c) = (vec![true; 8], vec![true; 8]);
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                r[i] = false;
                c[j] = false;
            }
        }
    }
    println!("{}", r.iter().filter(|&&f| f).count() * c.iter().filter(|&&f| f).count());
}
