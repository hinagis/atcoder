fn main() {
    proconio::input! {n: u64}
    let mut k = (n as f64).log2() as u32;
    while 2u64.pow(k) <= n {
        k += 1;
    }
    while 2u64.pow(k) > n {
        k -= 1;
    }
    println!("{}", k);
}
