fn main() {
    proconio::input! {
        n: usize,
        m: u64,
        mut a: [u64; n]
    }
    a.sort();
    let mut s = 0;
    for i in 0..n {
        let j = (n - i) as u64;
        let mut r = a[i];
        if s + r * j > m {
            let mut l = 0;
            while l < r {
                let h = (l + r + 1) / 2;
                if s + h * j <= m {
                    l = h;
                } else {
                    r = h - 1;
                }                
            }
            println!("{}", l);
            return;
        }
        s += a[i];
    }
    println!("infinite");
}
