use proconio::input as I;

fn main() {
    I! {n: usize}
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    let mut d = vec![];
    I! {
        x: i64,
        y: i64
    }
    a.push((x, y));
    for _ in 1..n {
        I! {
            x: i64,
            y: i64
        }
        let (u, v) = ((a[0].0 - x).abs(), (a[0].1 - y).abs());
        if u % 2 == v % 2 {
            if u > v {
                a.push((x, y));
            } else {
                b.push((x, y));
            }
        } else {
            if u > v {
                c.push((x, y));
            } else {
                d.push((x, y));
            }
        }
    }
    a.sort_by(|&a, &b| a.0.cmp(&b.0));
    b.sort_by(|&a, &b| a.1.cmp(&b.1));
    c.sort_by(|&a, &b| a.0.cmp(&b.0));
    d.sort_by(|&a, &b| a.1.cmp(&b.1));
    let mut r = 0;
    for i in 1..a.len() {
        r += (a[i].0 - a[i - 1].0).abs() * (a.len() - i) as i64;
    }
    for i in 1..b.len() {
        r += (b[i].1 - b[i - 1].1).abs() * (b.len() - i) as i64;
    }
    for i in 1..c.len() {
        r += (c[i].0 - c[i - 1].0).abs() * (c.len() - i) as i64;
    }
    for i in 1..d.len() {
        r += (d[i].1 - d[i - 1].1).abs() * (d.len() - i) as i64;
    }
    println!("{}", r);
}
