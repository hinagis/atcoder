use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
    }

    let mut t = vec![true; n];
    for _ in 0..m {
        I! {
            a: U,
            b: char,
        }
        println!("{}", if b == 'M' {
            if t[a] {
                t[a] = false;
                "Yes"
            } else {
                "No"
            }
        } else {
            "No"
        });
    }
}
