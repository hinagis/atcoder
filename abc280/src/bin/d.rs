use num_integer::gcd;

fn main() {
    proconio::input! {k: u64}
    let mut n = k;
    let mut x = 2;
    while x * x <= k {
        n /= gcd(n, x);
        if n == 1 {
            println!("{}", x);
            return;
        }
        x += 1;
    }
    println!("{}", ((x - 1 + n) / n) * n);
}
