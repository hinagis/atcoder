fn main() {
    proconio::input! {
        n: usize,
        a: [u32; 2 * n]
    }
    let mut c = 0;
    for i in 2..2 * n {
        if a[i - 2] == a[i] {
            c += 1;
        }
    }
    println!("{}", c);
}
