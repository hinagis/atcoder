fn main() {
    proconio::input! {
        n: usize,
        mut x: [u32; 5 * n]
    }
    x.sort();
    let mut s = 0;
    for i in n..4 * n {
        s += x[i];
    }
    println!("{}", s as f64 / (3f64 * n as f64));
}
