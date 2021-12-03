use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    const P: [char; 3] = ['o', 'x', 'x'];
    let mut i = 0;
    while i < s.len() && s[i] != 'o' {
        i += 1;
    }
    if i > 2 {
        println!("No");
        return;
    }
    for j in i..s.len() {
        if s[j] != P[(j - i) % 3] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
