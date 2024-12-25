use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        n: usize,
        mut k: usize,
        s: C
    }

    for i in 0..n {
        if k > 0 && s[i] == 'o' {
            print!("o");
            k -= 1;
        } else {
            print!("x");
        }
    }
    println!();
}
