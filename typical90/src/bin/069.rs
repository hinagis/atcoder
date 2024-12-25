const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: u64,
        k: u64,
    }
    println!("{}", if n == 1 {
        k
    } else if n == 2 {
        if k == 1 {
            0
        } else {
            (k * (k - 1)) % M
        }
    } else {
        if k <= 2 {
            0
        } else {
            (k * (k - 1) % M) * pow_mod(k - 2, n - 2, M) % M
        }
    });
}

fn pow_mod(mut a: u64, mut n: u64, m: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        n >>= 1;
    }
    r
}
