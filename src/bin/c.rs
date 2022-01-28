use proconio::{input as I, fastout};

#[fastout]
fn main() {
    I! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m]
    }
    let mut j = 0;
    for i in 0..n {
        if s[i] == t[j] {
            println!("Yes");
            j += 1;
        } else {
            println!("No");
        }
    }
}
