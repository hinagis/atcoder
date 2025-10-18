use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    let n = s.len();
    println!("{}{}",
        s[..n / 2].iter().collect::<String>(),
        s[n / 2 + 1..].iter().collect::<String>()
    );
}
