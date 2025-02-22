use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let m = *a.iter().max().unwrap() as usize + 1;
    let mut s = vec![0; m];
    for &i in &a {
        s[i] += 1;
    }
    let mut t = vec![0; m];
    for d in 1..m {
        for i in (d..m).step_by(d) {
            t[d] += s[i];
        }
    }
    let mut u = vec![0; m];
    for d in 1..m {
        if t[d] < k {continue}
        for i in (d..m).step_by(d) {
            u[i] = u[i].max(d);
        }
    }
    println!("{}", a.iter().map(|&i| u[i]).join("\n"));
}
