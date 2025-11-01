use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: usize}
    let mut p = Vec::with_capacity(q);
    let mut n = 0;
    let mut i = 0;
    for _ in 0..q {
        I! {k: u8}
        match k {
            1 => p.push(n),
            2 => {
                I! {t: usize}
                n += t;
            },
            _ => {
                I! {h: usize}
                let mut c = 0;
                while i < p.len() {
                    if n - p[i] < h {
                        break;
                    }
                    c += 1;
                    i += 1;
                }
                println!("{}", c);
            },
        }
    }
}
