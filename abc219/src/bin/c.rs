use proconio::{input, fastout, marker::Bytes};

#[fastout]
fn main() {
    input! {
        x: Bytes,
        n: usize,
        s: [Bytes; n]
    }
    let mut d = vec![0; 26];
    for i in 0..26 {
        d[(x[i] - b'a') as usize] = i as u8 + b'a';
    }
    let mut t = s.iter().enumerate().map(|(i, s)|
        (s.iter().map(|&c| d[(c - b'a') as usize] as char).collect::<String>(), i)
    ).collect::<Vec<_>>();
    t.sort();

    for t in t {
        println!("{}", s[t.1].iter().map(|&c| c as char).collect::<String>());
    }
}
