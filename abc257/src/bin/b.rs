use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k]
    }

    for _ in 0..q {
        I! {l: U}
        if l >= k - 1 {
            if a[l] < n {
                a[l] += 1;
            }
        } else {
            if a[l] < n && a[l] + 1 < a[l + 1] {
                a[l] += 1;
            }
        }
    }

    println!("{}", a.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
}
