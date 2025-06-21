use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
    }
    let mut c = 0;
    let mut f = vec![false; n + 2];
    for _ in 0..q {
        I! {a: usize}
        if f[a - 1] == f[a + 1] {
            if f[a] == f[a - 1] {
                c += 1;
            } else {
                c -= 1;
            }
        }
        f[a] ^= true;
        println!("{}", c);
    }
}
