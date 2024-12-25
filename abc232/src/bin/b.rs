use proconio::{input, marker::Bytes};
const N: u8 = b'z' - b'a' + 1;

fn main() {
    input! {s: Bytes, t: Bytes}
    let s = s.iter().map(|b| b - b'a').collect::<Vec<_>>();
    let t = t.iter().map(|b| b - b'a').collect::<Vec<_>>();
    let k = t[0] + if t[0] < s[0] {N} else {0} - s[0];
    for i in 1..s.len() {
        if (s[i] + k) % N != t[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
