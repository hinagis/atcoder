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
        r[i] = calc(c, d);
    }
    println!("{}", r.iter().join("\n"));
}

fn calc(c: u64, d: u64) -> u64 {
	let mut n = 0;

    let mut u = 1;
    let mut v = 9;
    let mut k = 10;
	while u <= c + d {
		let l = u.max(c + 1);
		let r = v.min(c + d);
		if l <= r {
			let vl = c * k + l;
			let vr = c * k + r;
			n += f(vr) - f(vl - 1);
		}
		u = u * 10;
		v = (v + 1) * 10 - 1;
		k *= 10;
	}
	n
}

fn f(x: u64) -> u64 {
	let mut y = (x as f64).sqrt() as u64;
	while y * y > x {
        y -= 1;
    }
	while (y + 1).pow(2) <= x {
        y += 1;
    }
	y
}