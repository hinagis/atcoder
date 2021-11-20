const N: i64 = 1_048_576; // 2^20

use proconio::{input, fastout};

#[fastout]
fn main() {
    let mut a = vec![-1; N as usize];
    let mut b = vec![0; N as usize];
    input! {q: usize}
    for _ in 0..q {
        input! {t: u8, x: i64}
        let s = (x % N) as usize;
        if t == 1 {
            let mut h = s;
            while a[h] != -1 {
                h = b[h];
            }
            a[h] = x;
            let n = (h + 1) % N as usize;
            b[h] = n;
            b[(x % N) as usize] = n;
        } else {
            println!("{}", a[s]);
        }
    }
}
