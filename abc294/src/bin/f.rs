use itertools::Itertools;
use superslice::Ext;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        k: usize,
        mut t: [(f64, f64); n],
        mut a: [(f64, f64); m]
    }

    let mut r = [0f64, 1.];

    while (r[0] - r[1]).abs() > 1e-12 {
        let mid = (r[0] + r[1]) / 2.;

        let mut v = a.iter().map(|&(c, d)| c - (c + d) * mid).collect_vec();
        v.sort_unstable_by(|x, y| x.partial_cmp(&y).unwrap());

        let c: usize = t
            .iter()
            .map(|&(a, b)| v.lower_bound_by(|&x| x.partial_cmp(&((a + b) * mid - a)).unwrap()))
            .sum();

        r[if c <= n * m - k {0} else {1}] = mid;
    }

    println!("{}", r[0] * 100.);
}
