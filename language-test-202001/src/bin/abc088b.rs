fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut p = vec![0, 0];
    for i in (0..n).rev() {
        p[(n - i) % 2] += a[i];
    }

    println!("{}", p[1] - p[0]);
}
