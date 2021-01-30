fn main() {
    proconio::input! {
        n: i64,
    }

    let mut r = 0;
    for i in 1..=n {
        if (2 * n - i * (i - 1)) % (2 * i) == 0 {
            r += 1;
        }
        if 2 * n <= i * (i - 1) {
            break;
        }
    }
    println!("{}", r * 2);
}
