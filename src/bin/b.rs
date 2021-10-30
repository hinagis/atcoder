use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize}
    let mut c = vec![0; n];
    for _ in 0..n - 1 {
        input! {a: Usize1, b: Usize1}
        c[a] += 1;
        c[b] += 1;
    }
    for i in 0..n {
        if c[i] != 1 && c[i] != n -1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
