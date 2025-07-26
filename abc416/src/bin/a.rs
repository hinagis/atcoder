use proconio::{input as I, marker::Chars as C};
fn main() {
    I! {
        _: usize,
        l: usize,
        r: usize,
        s: C
    }
    println!("{}", if (l..=r).all(|i| s[i - 1] == 'o') {"Yes"} else {"No"});
}
