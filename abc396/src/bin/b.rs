use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: u8}
    let mut s = vec![0; 100];
    for _ in 0..q {
        I! {t: u8}
        if t == 1 {
            I! {x: u8}
            s.push(x);
        } else {
            println!("{}", s.pop().unwrap());
        }
    }
}
