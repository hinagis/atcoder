fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut c = vec![0; 200_001];
    for &a in &a {
        c[a] += 1;
    }
    for i in 0..200_000 {
        c[i + 1] += c[i];
    }
    let mut r = 0;
    for i in 0..n {
        r += c[a[i] - 1] * (n - c[a[i]]);
    }
    println!("{}", r);
}
