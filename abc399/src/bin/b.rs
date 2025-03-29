use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n]
    }
    let mut p = p.iter().enumerate().map(|p| (p.0, *p.1)).collect::<Vec<_>>();
    p.sort_by(|a, b| b.1.cmp(&a.1));
    let mut r = (1, p[0].1);
    for i in 0..n {
        if p[i].1 != r.1 {
            r = (i + 1, p[i].1);
        }
        p[i].1 = r.0;
    }
    p.sort_by_key(|a| a.0);
    println!("{}", p.iter().map(|(_, r)| r).join("\n"));
}
