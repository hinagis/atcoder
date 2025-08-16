use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: u32}
    let mut b = vec![];
    for _ in 0..q {
        I! {t: u8}
        if t == 1 {
            I! {x: u32}
            b.push(x);
            b.sort_by(|a, b| b.cmp(a));
        } else {
            let x = b.pop().unwrap();
            println!("{}", x);
        }
    }
}
