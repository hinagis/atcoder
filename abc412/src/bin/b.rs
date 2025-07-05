use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        s: C,
        t: C
    }
    for i in 1..s.len() {
        if s[i].is_uppercase() && !t.contains(&s[i - 1]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
