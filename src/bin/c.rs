const L: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        mut c: [u64; n]
    }
    c.sort_by(|a, b| b.cmp(a));
    let mut r = 1;
    for i in 0..n {
        if c[i] >= (n - i) as u64 {
            r *= c[i] - (n - i) as u64 + 1;
            r %= L;
        } else {
            println!("0");
            return;
        }
    }
    println!("{}", r);
}
