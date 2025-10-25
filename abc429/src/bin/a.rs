use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: u32,
        m: u32,
    }
    for i in 1..=n {
        if i <= m {
            println!("OK");
        } else {
            println!("Too Many Requests");
        }
    }
}
