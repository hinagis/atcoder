fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [i32; n],
        mut b: [i32; m],
    }
    b.sort();

    let mut r = (a[0] - b[0]).abs();
    for &a in &a {
        let d = match b.binary_search(&a) {
            Err(j) => {
                if j == 0 {
                    (a - b[0]).abs()
                } else if j >= m {
                    (a - b[m - 1]).abs()
                } else {
                    (a - b[j - 1]).abs().min((a - b[j]).abs())
                }
            },
            _ => 0
        };
        r = r.min(d);
    }

    println!("{}", r);
}
