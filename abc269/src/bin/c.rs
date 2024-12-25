use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {n: u64}
    let mut t = vec![];
    for i in 0..15 {
        let mut b = if i == 0 {0} else {t[i - 1] + 1};
        while b <= 60 && (n & (1 << b)) == 0 {
            b += 1;
        }
        if b > 60 {
            break;
        }
        t.push(b);
    }

    let mut b = 0;
    while b < 2u64.pow(t.len() as u32) {
        let mut d = 0;
        for j in 0..t.len() {
            if b & (1 << j) != 0 {
                d |= 1u64 << t[j];
            }
        }
        println!("{}", d);
        b += 1;
    }
}
