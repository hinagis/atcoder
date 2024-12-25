fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut p = 0;
    let mut c = vec![0, 360];
    for a in a {
        p = (p + a) % 360;
        let i = c.binary_search(&p).unwrap_err();
        c.insert(i, p);
    }
    println!("{}", (0..=n).map(|i| c[i + 1] - c[i]).max().unwrap());
}
