use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {s: B}

    let mut c = 0;
    let mut i = 0;
    let mut j = i;
    while j < s.len() && s[j] == s[i] {
        j += 1;
    }
    while j < s.len() {
        let mut k = j;
        while k < s.len() && s[k] == s[j] {
            k += 1;
        }
        if s[i] + 1 == s[j] {
            c += (j - i).min(k - j);
        }
        i = j;
        j = k;
    }
    println!("{}", c);
}
