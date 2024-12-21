use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        mut a: [usize; n]
    }
    let mut b = vec![0; n];
    for (i, a) in a.iter().enumerate() {
        b[a - 1] = i + 1;
    }
    let mut c = vec![];
    for i in 0..n {
        if a[i] != i + 1 {
            let j = b[i] - 1;
            b.swap(i, a[i] - 1);
            a.swap(i, j);
            c.push((i + 1, j + 1));
        }
    }
    println!("{}", c.len());
    for (i, j) in c {
        println!("{} {}", i, j);
    }
}
