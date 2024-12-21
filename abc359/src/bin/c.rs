use std::cmp::Ordering::*;

fn main() {
    proconio::input! {
        s: (i64, i64),
        t: (i64, i64),
    }
    let d = ((s.0 - t.0).abs(), (s.1 - t.1).abs());
    let p = (s.0 + s.1) % 2 == 0;
    let v = t.0.cmp(&s.0);
    println!("{}", d.1 + match (p, v) {
        (_, Equal) => 0,
        (true, Greater) | (false, Less) => (d.0 - d.1).max(0) / 2,
        (true, Less) | (false, Greater) => (d.0 - d.1 + 1).max(0) / 2,
    });
}
