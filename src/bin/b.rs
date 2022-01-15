fn main() {
    proconio::input! {
        n: usize,
        h: [u32; n]
    }
    let mut i = 0;
    while i < n - 1 && h[i + 1] > h[i] {
        i += 1
    }
    println!("{}", h[i]);
}
