fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [[u32; n]; n]
    }
    let mut m = u32::max_value();
    let mut kk = vec![0; k * k];
    let mi = kk.len() - 1 - kk.len() / 2;
    for i in 0..=(n - k) {
        for j in 0..=(n - k) {
            for y in 0..k {
                for x in 0..k {
                    kk[y * k + x] = a[i + y][j + x];
                }
            }
            kk.sort();
            m = m.min(kk[mi]);
        }
    }

    println!("{}", m);
}
