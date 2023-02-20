use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {
            n: usize,
            d: usize,
            k: U
        }
        println!("{}", k / (n / num_integer::gcd(d, n)) + k * d % n);
    }
}
