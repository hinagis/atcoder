use itertools::Itertools;
use proconio::input as I;

fn main() {
    I! {t: usize}
    let mut r = vec!{0; t};
    for i in 0..t {
        I! {
            c: u64,
            d: u64
        }
        let mut s = 0;

        let mut k = 10;
        while k < d {
            let u = ((c * k + if k == 10 {1} else {0}) as f64).sqrt().ceil() as u64;
            let v = ((c * k + k - 1) as f64).sqrt().floor() as u64;
            for j in u..=v {
                if j.pow(2) as u64 / k == c {
                    s += 1;
                }
            }
            k *= 10;
        }
        let u = ((c * k + if k == 10 {1} else {0}) as f64).sqrt().ceil() as u64;
        let v = ((c * k + d) as f64).sqrt().floor() as u64;
        for j in u..=v {
            if j.pow(2) as u64 / k == c {
                s += 1;
            }
        }
        r[i] = s;
    }
    println!("{}", r.iter().join("\n"));
}
