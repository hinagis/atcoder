fn main() {
    proconio::input! {n: u64}
    let mut c = 0;
    let mut a = 1;
    while a * a * a <= n {
        let mut b = a;
        while a * b * b <= n {
            c += n / (a * b) - b + 1;
            b += 1;
        }
        a += 1;
    }

    println!("{}", c);
}
