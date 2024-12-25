use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {n: usize}
    let mut a = vec![vec![]; n];
    a[0].push(1);
    println!("1");
    for i in 1..n {
        a[i].push(1);
        for j in 1..i {
            let v = a[i - 1][j - 1] + a[i - 1][j];
            a[i].push(v);
        }
        a[i].push(1);
        println!("{}", a[i].iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" "));
    }
}
