use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        k: usize,
        mut s: String,
    }
    if s.len() > k {
        s.truncate(k);
        s += "...";
    }
    println!("{}", s)
}
