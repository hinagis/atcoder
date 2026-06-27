use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    let e = s.iter().filter(|&&c| c == 'E').count() > s.len() / 2;
    println!("{}", if e {"East"} else {"West"});
}
