use std::i64::MAX;

fn main() {
    proconio::input! {
        n: i64,
        m: i64
    }

    let mut r = if m <= n {m} else {MAX};
    if r == MAX {
        for i in 1..=n {
            let x = (m + i - 1) / i;
            if x <= n {
                r = r.min(i * x);
            }
            if i > x {break}
        }
    }
    println!("{}", if r == MAX {-1} else {r});
}
