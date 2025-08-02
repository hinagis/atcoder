fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    let mut p = vec![0; n];
    for i in 1..n {
        if a[i] < i {
            p[i - a[i]] += 1;
        }
    }
    let mut c = 0u64;
    for i in 0..n {
        if i + a[i] < n {
            c += p[i + a[i]];
        }
    }
    println!("{}", c);
}
