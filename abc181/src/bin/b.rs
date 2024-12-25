use proconio::input;

fn main() {
    input! {n: usize}

    let mut r = 0;
    for _ in 0..n {
        input! {a: u64, b: u64}
        let c = b * (b + 1) / 2;
        let d = (a - 1) * a / 2;
        r += c - d;
    }
    println!("{}", r);
}
