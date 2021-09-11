use proconio::{input, marker::{Usize1, Chars}};

fn main() {
    input! {
        n: Usize1,
        s: Chars
    }
    println!("{}", if s[n] == 'o' {"Yes"} else {"No"});
}
