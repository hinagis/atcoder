use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
        a: [U; n],
    }
    let mut c = vec![0; m];
    for &i in &a {
        c[i] += 1;
    }
    if c.contains(&0) {
        println!("0");
        return;
    }
    for i in (0..n).rev() {
        let j = a[i];
        c[j] -= 1;
        if c[j] == 0 {
            println!("{}", n - i);
            return;
        }
    }
    println!("{}", n);
}
