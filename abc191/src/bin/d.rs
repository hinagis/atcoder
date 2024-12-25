fn main() {
    proconio::input! {
        x: f64,
        y: f64,
        r: f64,
    }
    let p = 10000i64;
    let pf = p as f64;
    let x = (x * pf).round() as i64;
    let y = (y * pf).round() as i64;
    let r = (r * pf).round() as i64;

    let mut ans = 0;

    let mut j = (x - r) / p * p;
    let mut t = (y + p - 1) / p * p;
    let mut b = y / p * p;
    while j <= x {
        while (x - j).pow(2) + (y - t).pow(2) <= r * r {
            t += p;
        }
        while (x - j).pow(2) + (y - b).pow(2) <= r * r {
            b -= p;
        }
        ans += 0.max((t - b) / p - 1);
        j += p;
    }

    let mut j = (x + r) / p * p;
    let mut t = (y + p - 1) / p * p;
    let mut b = y / p * p;
    while j > x {
        while (x - j).pow(2) + (y - t).pow(2) <= r * r {
            t += p;
        }
        while (x - j).pow(2) + (y - b).pow(2) <= r * r {
            b -= p;
        }
        ans += 0.max((t - b) / p - 1);
        j -= p;
    }

    println!("{}", ans);
}
