const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {n: u64, l: u64}
    let s = n / l;
    let o = n % l;
    let mut r = 1;
    for i in 1..=s {
        r = (r + combination(i + (s - i) * l + o, i, M)) % M;
    }

    println!("{}", r);
}

fn combination(mut n: u64, r: u64, m: u64) -> u64 {
    let mut f = 1;
    let mut d = 1;
    for i in 1..r + 1 {
        f = f * n % m;
        d = d * i % m;

        n -= 1;
    }
    f * pow_mod(d, m - 2, m) % m
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
