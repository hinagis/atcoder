use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}

    let (mut l, mut r) = (0, s.len() - 1);
    while l < r && s[l] == 'a' {
        if s[r] != 'a' {
            println!("No");
            return;
        }
        l += 1;
        r -= 1;
    }
    while l < r && s[r] == 'a' {
        r -= 1
    }
    while l < r {
        if s[l] != s[r] {
            println!("No");
            return;
        }
        l += 1;
        r -= 1;
    }

    println!("Yes");
}
