fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut l = vec![0; n];
    l[0] = 1;
    for i in 1..n {
        l[i] = if a[i] > l[i - 1] {l[i - 1] + 1} else {a[i]};
    }
    let mut r = vec![0; n];
    r[n - 1] = 1;
    for i in (0..n - 1).rev() {
        r[i] = if a[i] > r[i + 1] {r[i + 1] + 1} else {a[i]};
    }
    let mut m = 1;
    for i in 0..n {
        m = m.max(l[i].min(r[i]));
    }

    println!("{}", m);
}
