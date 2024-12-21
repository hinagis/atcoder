fn main() {
    proconio::input! {d: [(i64, i64)]}
    let s = d.iter().fold(0, |s, (a, _)| s + a);
    let mut r = 0;
    for (a, b) in d {
        r = r.max(s - a + b);
    }

    println!("{}", r);
}
