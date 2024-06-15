fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; m],
    }
    a.sort();
    b.sort();
    let mut c = 0;
    let mut i = 0;
    for j in 0..m {
        while i < n && b[j] > a[i] {
            i += 1;
        }
        if i >= n {
            println!("-1");
            return;
        }
        c += a[i];
        i += 1;
    }
    println!("{}", c);
}
