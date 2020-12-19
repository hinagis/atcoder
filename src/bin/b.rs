fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [u32; h * w],
    }
    let min = a.iter().fold(a[0], |min, &e| min.min(e));
    let sum = a.iter().fold(0, |s, &e| s + e - min);

    println!("{}",sum);
}
