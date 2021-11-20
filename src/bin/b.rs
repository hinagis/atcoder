use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        a: [Usize1; n],
    }
    let mut f = vec![true; n];
    f[x] = false;
    let mut c = 1;
    let mut i = x;
    while f[a[i]] {
        c += 1;
        f[a[i]] = false;
        i = a[i];
    }
    println!("{}", c);
}
