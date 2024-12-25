fn main() {
    proconio::input! {
        n: u32,
    }
    let mut d = 1;
    while d * (1 + d) < 2 * n {
        d += 1;
    }

    println!("{}", d);
}
