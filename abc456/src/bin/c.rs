use proconio::{input as I, marker::Chars as C};

const M: usize = 998244353;

fn main() {
    I! {s: C}
    let n = s.len();
    let calc = |i| {
        let mut j = i + 1;
        while j < n && s[j] != s[j - 1] {
            j += 1;
        }
        j
    };
    let mut c = 0;
    let mut l = 0;
    while l < n {
        let r = calc(l);
        let d = r - l;
        c += d * (d + 1) / 2;
        c %= M;
        l = r;
    }
    println!("{}", c);
}
