fn main() {
    proconio::input! {
        n: usize,
        s: [i64; n],
    }

    let mut a = Vec::with_capacity(n);
    let mut p = 0;
    for &s in &s {
        let b = s - p;
        a.push(b);
        p += b;
    }
    println!("{}", a.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(" "));
}
