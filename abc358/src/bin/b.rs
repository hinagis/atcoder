use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        a: u64,
        t: [u64; n]
    }
    let mut c = 0;
    let mut i = 0;
    while i < n {
        if c < t[i] {
            c = t[i];
        }
        i += 1;
        c += a;
        println!("{}", c);
    }
}
