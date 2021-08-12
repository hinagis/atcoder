use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, s: Chars}
    let mut a = 0u64;
    let mut l = 0;
    while l < n {
        let mut r = l + 1;
        while r < n && s[r] == s[l] {
            r += 1;
        }
        a += (r - l) as u64 * (n - r) as u64;
        l = r;
    }

    println!("{}", a);
}
