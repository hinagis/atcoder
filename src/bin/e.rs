use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(u64, u64); n],
    }
    let mut l: Vec<u64> = d.iter().map(|(l, _)| *l).collect();
    let mut r: Vec<u64> = d.iter().map(|(_, r)| *r).collect();
    l.sort();
    r.sort();
    if n % 2 == 1 {
        println!("{}", r[n/2] - l[n/2] + 1);
    } else {
        let l2 = l[n/2 - 1] + l[n/2];
        let r2 = r[n/2 - 1] + r[n/2];
        println!("{}", r2 - l2 + 1);
    }
}
