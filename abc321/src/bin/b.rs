fn main() {
    proconio::input! {
        n: usize,
        x: i32,
        mut a: [i32; n - 1]
    }
    a.sort();
    let s = (1..n - 2).fold(0, |s, i| s + a[i]);
    println!("{}", if a[0] + s >= x {
            0
        } else if x - s <= a[n - 2] {
            x - s
        } else {
            -1
        }
    )
}
