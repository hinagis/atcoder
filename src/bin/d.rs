use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    let mut v = vec![true; n + 1];
    let mut j = n;
    for i in 0..k {
        v[p[i]] = false;
        j = j.min(p[i]);
    }
    println!("{}", j);

    for i in k..n {
        v[p[i]] = false;
        if p[i] > j {
            j += 1;
            while v[j] {
                j += 1;
            }
        }
        println!("{}", j);
    }
}
