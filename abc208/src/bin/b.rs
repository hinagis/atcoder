fn main() {
    proconio::input! {
        mut p: u32,
    }
    let c = [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800];
    let mut r = 0;
    for i in (0..10).rev() {
        r += p / c[i];
        p %= c[i];
    }
    println!("{}", r);
}
