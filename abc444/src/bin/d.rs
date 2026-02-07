use itertools::Itertools;
use proconio::input as I;

fn main() {
    I! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();

    let mut s = Vec::with_capacity(1000000);
    let mut r = 0;
    let mut l = 0;
    for i in 1..=a[n - 1] {
        while a[l] < i {
            l += 1;
        }
        r += n - l;
        if r > 0 {
            s.push((r % 10) as u8);
        }
        r /= 10;
    }
    while r > 0 {
        s.push((r % 10) as u8);
        r /= 10;
    }
    println!("{}", s.iter().rev().join(""));
}
