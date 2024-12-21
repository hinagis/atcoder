fn main() {
    proconio::input! {
        n: u32,
        t: u32,
        a: u32,
    }
    let n = n / 2;
    println!("{}", if t > n || a > n {"Yes"} else {"No"});
}
