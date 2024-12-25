fn main() {
    proconio::input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n]
    }
    a.sort_unstable();
    b.sort_unstable();
    println!("{}", a.iter().zip(b.iter()).fold(0, |s, (a, b)| s + (a - b).abs()));
}
