fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut t: u64,
        a: [u64; n - 1],
        xy: [(usize, u64); m]
    }

    let mut i = 1;
    let mut j = 0;
    while i < n {
        if a[i - 1] >= t {
            println!("No");
            return;
        }
        t -= a[i - 1];
        i += 1;
        if j < m && i == xy[j].0 {
            t += xy[j].1;
            j += 1;
        }
    }

    println!("Yes");
}
