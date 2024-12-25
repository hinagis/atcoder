const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: u64,
    }

    if n <= 1 {
        println!("0");
    } else {
        let mut v = (pow(10, n) + pow(8, n)) % M;
        let no09 = pow(9, n) * 2 % M;
        if v < no09 {
            v += M;
        }
        println!("{}", v - no09);
    }
}

fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 > 0 {
            r = r * a % M;
        }
        a = a * a % M;
        n >>= 1;
    }
    r
}
