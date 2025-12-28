use proconio::input as I;

fn main() {
    I! {n: usize}
    let mut r = vec![(0, 0)];
    let mut p = 0;
    for _ in 0..n {
        I! {a: usize}
        if a != p {
            r.push((a, 0));
        }
        let i = r.len() - 1;
        r[i].1 += 1;
        if r[i].1 > 3 {
            r.pop();
        }
        p = r[r.len() - 1].0;
    }
    println!("{}", r.iter().fold(0, |s, (_, c)| s + c));
}
