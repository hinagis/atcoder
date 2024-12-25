use proconio::input;
const M: u64 = 1000_000_007;

fn main() {
    input! {t: usize}
    for _ in 0..t {
        input! {
            n: u64,
            a: u64,
            b: u64,
        }

        let na = n - a + 1;
        let nb = n - b + 1;
        let ab = na * nb % M;

        let x = if n < a + b {0} else {let n = na - b; (n + 1) * n % M};
        let x = (ab + M - x) % M;
        let x = (x * x) % M;

        let r = (ab * ab) % M;
        let r = (r + M - x) % M;

        println!("{}", r);
    }
}
