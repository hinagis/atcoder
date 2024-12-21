fn main() {
    proconio::input! {mut n: u64}
    fn d(mut n: u64, k: u64) -> u64 {
        while n > 0 && n % k == 0 {
            n /= k;
        }
        n
    }
    let n = d(d(n, 2), 3);
    println!("{}", if n == 0 || n == 1 {"Yes"} else {"No"});
}
