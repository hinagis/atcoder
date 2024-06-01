fn main() {
    const M: u64 = 998244353;
    proconio::input! {
        n: u64,
        m: u64,
    }
    let mut c = 0;
    for k in 1..=n {
        c += (k & m).count_ones() as u64 % M;
    }
    println!("{}", c);
}
