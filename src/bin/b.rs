use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    for i in 0..n {
        if s[i] == '1' {
            println!("{}", if i % 2 == 0 {"Takahashi"} else {"Aoki"});
            return;
        }
    }
}
