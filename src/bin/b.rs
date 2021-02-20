use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}

    for i in 0..s.len() {
        if (s[i].is_lowercase()) ^ (i % 2 == 0) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
