use itertools::Itertools;

fn main() {
    proconio::input! {
        mut n: usize,
        m: usize,
        mut v: [(usize, usize); m],
    }
    let v = v.iter()
        .map(|&(a, b)| (a, b, a - b))
        .sorted_by(|a, b| a.2.cmp(&b.2))
        .collect_vec();
    let mut c = 0;
    for i in 0..m {
        let (a, _, d) = v[i];
        if n < a {continue}
        let x = 0.max((n - a) / d + 1);
        n -= x * d;
        c += x;
    }
    println!("{}", c);
}
