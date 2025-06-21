fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n]
    }
    a.sort_by(|a, b| b.cmp(a));
    for i in (0..n).rev() {
        if a[i] >= i + 1 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("0");
}
