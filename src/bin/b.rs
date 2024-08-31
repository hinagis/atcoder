use proconio::input as I;

fn main() {
    I! {n: u32}
    let mut l = vec![];
    let mut r = vec![];
    for _ in 0..n {
        I! {
            a: i32,
            s: char
        }
        if s == 'L' {
            l.push(a);
        } else {
            r.push(a);
        }
    }
    let mut d = 0;
    if l.len() > 0 {
        let mut p = l[0];
        for &a in l.iter().skip(1) {
            d += (p - a).abs();
            p = a;
        }
    }
    if r.len() > 0 {
        let mut p = r[0];
        for &a in r.iter().skip(1) {
            d += (p - a).abs();
            p = a;
        }
    }
    println!("{}", d);
}
