use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {n: usize}

    let mut a = [0; 200_002];
    for _ in 0..n {
        I! {l: usize, r: usize}
        a[l] += 1;
        a[r] -= 1;
    }
    for i in 1..=200_001 {
        a[i] += a[i - 1]
    }
    for i in 1..=200_001 {
        if a[i - 1] == 0 && a[i] != 0 {
            print!("{} ", i)
        }
        if a[i - 1] != 0 && a[i] == 0 {
            println!("{}", i)
        }
    }
}
