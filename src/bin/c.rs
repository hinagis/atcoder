fn main() {
    proconio::input! {
        a: (f64, f64),
        b: (f64, f64),
        c: (f64, f64),
        d: (f64, f64),
    }

    println!("{}",
        if calc(a, b, c) &&
           calc(b, c, d) &&
           calc(c, d, a) &&
           calc(d, a, b) {
            "Yes"
        } else {
            "No"
        }
    );
}

fn calc(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    let vb = (b.0 - a.0, b.1 - a.1);
    let vc = (c.0 - b.0, c.1 - b.1);
    let c = vb.0 * vc.1 - vb.1 * vc.0;
    c > 0f64
}
