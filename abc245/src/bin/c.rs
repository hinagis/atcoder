fn main() {
    proconio::input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    }
    let mut f = (true, true);
    for i in 0..n - 1 {
        let t = (
            (a[i] - a[i + 1]).abs() <= k,
            (a[i] - b[i + 1]).abs() <= k,
            (b[i] - a[i + 1]).abs() <= k,
            (b[i] - b[i + 1]).abs() <= k
        );
        f = match f {
            (true, true) => (t.0 | t.2, t.1 | t.3),
            (true, false) => (t.0, t.1),
            (false, true) => (t.2, t.3),
            (false, false) => (false, false),
        }
    }

    println!("{}", if f.0 | f.1 {"Yes"} else {"No"});
}
