use proconio::{input as I, fastout};

#[fastout]
fn main() {
    I! {n: usize}
    let mut s: Vec<(usize, usize)> = Vec::with_capacity(n);
    for _ in 0..n {
        I! {a: usize}
        let l = s.len();
        if l > 0 && s[l - 1].0 == a {
            let c = s[l - 1].1;
            if c + 1 == a {
                for _ in 0..c {
                    s.pop();
                }
            } else {
                s.push((a, c + 1));
            }
        } else {
            s.push((a, 1));
        }
        println!("{}", s.len());
    }
}
