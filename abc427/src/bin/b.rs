fn main() {
    proconio::input! {n: usize}
    let mut a = 1;
    let mut f = 0;
    for _ in 0..n {
        f += a;
        a = f.to_string().bytes().fold(0, |s, b| s + (b - b'0') as u32);
    }
    println!("{}", f);
}
