use proconio::{input, marker::Chars};
const M: u64 = 1000_000_007;

fn main() {
    input! {n: usize, s: Chars}
    let mut c = vec![0; 7];

    for i in 0..n {
        match s[i] {
            'a' => {c[0] += 1},
            't' => {c[1] = (c[0] + c[1]) % M},
            'c' => {c[2] = (c[1] + c[2]) % M},
            'o' => {c[3] = (c[2] + c[3]) % M},
            'd' => {c[4] = (c[3] + c[4]) % M},
            'e' => {c[5] = (c[4] + c[5]) % M},
            'r' => {c[6] = (c[5] + c[6]) % M},
            _ => {}
        }
    }
    println!("{}", c[6]);
}
