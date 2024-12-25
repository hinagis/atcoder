fn main() {
    proconio::input! {
        n: u64,
        mut x: u64,
        m: u64,
    }

    let mut dp = vec![None; m as usize];
    let mut r = 0;
    let mut i = 0;
    while i < n {
        if let Some((c, v)) = dp[x as usize] {
            let w = i - c;
            let l = n / w - 3;
            if l > 3 {
                r += (r - v) * l;
                i += w * l;
            }
            while i < n {
                r += x;
                i += 1;
                x = x * x % m;
            }
        } else {
            dp[x as usize] = Some((i, r));
            r += x;
            i += 1;
            x = x * x % m;
        }
    }
    println!("{}", r);
}
