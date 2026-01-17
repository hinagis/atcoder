use proconio::input as I;

fn main() {
    I! {
        n: usize,
        k: usize,
        x: u64,
        mut a: [u64; n],
    }
    a.sort_by(|a, b| b.cmp(a));
    let mut v = 0;
    for i in n - k..n {
        v += a[i];
        if v >= x {
            println!("{}", i + 1);
            return;
        }
    }
    if x > v {
        println!("-1");
    } else {
        println!("{}", n);
    }
}
