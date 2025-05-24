use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {s: B}
    let s = s.iter().map(|b| (b - b'0') as usize).collect::<Vec<_>>();
    let mut t = s.len();
    for w in s.windows(2) {
        t += w[0] + if w[0] >= w[1] {0} else {10} - w[1];
    }
    t += s[s.len() - 1];
    println!("{}", t);
}
