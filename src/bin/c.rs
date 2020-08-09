fn main() {
    proconio::input! {
        k: u64,
    }
    let mut r = 0;
    for i in 1..=k {
        r = (r * 10 + 7) % k;
        if r == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
