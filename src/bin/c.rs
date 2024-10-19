fn main() {
    proconio::input! {
        n: usize,
        mut a: [u32; n],
        mut b: [u32; n - 1]
    }
    a.sort();
    b.sort();
    for i in 0..n - 1 {
        if a[i] > b[i] {
            println!("-1");
            return;
        }
    }
    for i in (1..n).rev() {
        if a[i] > b[i - 1] {
            println!("{}", a[i]);
            return;
        }
    }
    println!("{}", a[0]);
}
