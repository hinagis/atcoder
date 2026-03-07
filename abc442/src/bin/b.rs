use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: u32}
    let mut p = false;
    let mut v = 0;
    for _ in 0..q {
        I! {a: u8}
        match a {
            1 => {
                v += 1;
            },
            2 => {
                if v > 0 {
                    v -= 1;
                }
            },
            _ => {
                p = !p;
            }
        }
        println!("{}", if p && v > 2 {"Yes"} else {"No"});
    }
}
