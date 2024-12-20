fn main(){
    proconio::input!{
        n: usize,
        x: i64,
        a: [(i64, i64, i64, i64); n]
    }

    let mut l = 0;
    let mut r = 10i64.pow(10);
    while l + 1 < r {
        let w = (l + r) / 2;
        let mut y = 0;
        for &(a, p, b, q) in &a {
            let mut t = x + 1;
            if a * q < b * p {
                for c in 0..b {
                    let d = if w > a * c {(w - a * c + b - 1) / b} else {0};
                    t = t.min(p * c + q * d)
                }
            } else {
                for d in 0..a {
                    let c = if w > b * d {(w - b * d + a - 1) / a} else {0};
                    t = t.min(p * c + q * d)
                }
            }
            y += t
        }
        if y > x {
            r = w;
        } else {
            l = w;
        }
    }
    println!("{l}");
}
