fn main() {
    proconio::input! {
        n: usize,
        l: u32,
        k: usize,
        mut a: [u32; n],
    }
    a.push(l);
    let mut r = l / (k as u32 + 1) + 1;
    let mut l = 0;

    while l < r {
        let m = (l + r + 1) / 2;
        let mut c = 0;
        let mut b = 0;
        for i in 0..=n {
            if a[i] - b >= m {
                c += 1;
                b = a[i];
            }
        }
        if c >= k + 1 {
            l = m;
        } else {
            r = m - 1;
        }
    }
    println!("{}", l);
}
