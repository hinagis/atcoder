use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        a: [U; n]
    }
    let mut f = vec![true; n];
    for i in 0..n {
        if f[i] {
            f[a[i]] = false;
        }
    }
    println!("{}", f.iter().filter(|&&f| f).count());
    println!("{}", f.iter().enumerate().filter(|&(_, &f)| f).map(|(i, _)| (i + 1).to_string()).collect::<Vec<_>>().join(" "));
}
