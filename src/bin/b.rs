fn main() {
    const M: u64 = 998244353;
    proconio::input! {
        a: [u64; 6]
    }
    let a = a.iter().map(|a| a % M).collect::<Vec<_>>();
    let b = (a[0] * a[1]) % M;
    let c = (b * a[2]) % M;
    let e = (a[3] * a[4]) % M;
    let f = (e * a[5]) % M;
    println!("{}", (c + M - f) % M);
}
