fn main() {
    const M: i64 = 998244353;
    proconio::input! {n: i64}
    println!("{}", ((n % M) + M) % M);
}
