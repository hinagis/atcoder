fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }

    println!("{}", a.iter().fold(0, |s, &a| s + a.saturating_sub(10)));
}
