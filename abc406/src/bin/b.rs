fn main() {
    proconio::input! {
        n: usize,
        k: u32,
        a: [u128; n]
    }
    let mut r = 1;
    for a in a {
        r *= a;
        if r >= 10u128.pow(k) {
            r = 1;
        }
    }
    println!("{}", r);
}
