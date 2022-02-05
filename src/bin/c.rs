const M: u64 = 998244353;

fn main() {
    proconio::input! {n: u64}

    let mut r = 0;
    let mut s = 1;
    let mut e = s * 10 - 1;
    while n > e {
        let c = s * 9 % M;
        let d = (c * (1 + c) / 2) % M;
        r = (r + d % M) % M;
        s *= 10;
        e = s * 10 - 1;
    }
    let c = (n + 1 - s) % M;
    let d = (c * (1 + c) / 2) % M;
    r = (r + d % M) % M;

    println!("{}", r);
}
