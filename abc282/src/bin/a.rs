use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {n: u8}
    for i in 0..n {
        print!("{}", (b'A' + i) as char);
    }
    println!("");
}
