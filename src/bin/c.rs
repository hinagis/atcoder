fn main() {
    proconio::input! {
        x: i128,
        a: i128,
        d: i128,
        n: i128,
    }

    if x == a {
        println!("0");
    } else if x < a {
        if d < 0 {
            let e = a + d * (n - 1);
            if x > e {
                let c = (x - a).abs() / d.abs();
                let s = (c - 100).max(0);
                let e = (c + 100).min(n);
                let mut r = d.abs();
                for i in s..e {
                    r = r.min((x - (a + d * i)).abs());
                }
                println!("{}", r);
            } else {
                println!("{}", (e - x).abs());
            }
        } else {
            println!("{}", (a - x).abs());
        }
    } else {
        if d > 0 {
            let e = a + d * (n - 1);
            if x < e {
                let c = (x - a).abs() / d.abs();
                let s = (c - 100).max(0);
                let e = (c + 100).min(n);
                let mut r = d.abs();
                for i in s..e {
                    r = r.min((x - (a + d * i)).abs());
                }
                println!("{}", r);
            } else {
                println!("{}", (e - x).abs());
            }
        } else {
            println!("{}", (a - x).abs());
        }
    }
}
