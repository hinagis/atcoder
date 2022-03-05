use proconio::{input as I, fastout, marker::Bytes};

#[fastout]
fn main() {
    I! {s: Bytes, q: usize}

    for _ in 0..q {
        I! {t: u64, k: u64}
        println!("{}", calc(&s, t, k - 1) as char);
    }
}

fn calc(s: &Vec<u8>, t: u64, k: u64) -> u8 {
    if t == 0 {
        s[k as usize]
    } else if k == 0 {
        b'A' + (s[0] - b'A' + (t % 3) as u8) % 3
    } else {
        b'A' + (calc(s, t - 1, k / 2) - b'A' + (k % 2) as u8 + 1) % 3
    }
}
