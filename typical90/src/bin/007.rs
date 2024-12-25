use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
    }
    a.sort();
    for _ in 0..q {
        input! {b: i32}
        if let Err(i) = a.binary_search(&b) {
            if i >= n {
                println!("{}", b - a[n - 1]);
            } else {
                let d = a[i] - b;
                println!("{}", if i > 0 {d.min(b - a[i - 1])} else {d});
            }
        } else {
            println!("0");
        }
    }
}
