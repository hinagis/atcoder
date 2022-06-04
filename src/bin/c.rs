use proconio::{input as I};

fn main() {
    I! {
        n: usize,
        k: usize
    }

    let mut b = vec![vec![]; k];
    for i in 0..n {
        I! {a: usize}
        b[i % k].push(a)
    }

    for b in b.iter_mut() {
        b.sort();
    }

    let mut c = 0;
    for i in 0..n {
        let b = b[i % k][i / k];
        if c > b {
            println!("No");
            return;
        }
        c = b;
    }
    println!("Yes");
}
