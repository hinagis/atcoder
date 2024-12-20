use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {n: usize}
    let mut a = vec![];
    for i in 1..=n {
        I! {b: [U; i]}
        a.push(b);
    }
    let mut i = 0;
    for j in 0..n {
        let (u, v) = if i >= j {(i, j)} else {(j, i)};
        i = a[u][v];
    }
    println!("{}", i + 1);
}
