use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut t = 0;
    let mut c = vec![0; n];
    for i in 0..n {
        t += c[i];
        a[i] = i + a[i] - t;
        if a[i] >= n - i - 1 {
            a[i] -= n - i - 1;
        } else {
            c[i + a[i] + 1] += 1;
            a[i] = 0;
        }
    }
    println!("{}", a.iter().join(" "));
}
