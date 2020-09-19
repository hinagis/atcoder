fn main() {
    proconio::input! {
        n: u64,
        mut x: u64,
        m: u64,
    }

    let mut dp = vec![false; m as usize];
    dp[x as usize] = true;
    let mut r = 0;
    let mut i = 0;
    while i < n {
        r += x;
        i += 1;
        x = x * x % m;
        if dp[x as usize] {
            let l = n / i;
            if l > 1 {
                r = r * l % m;
                i = l * i;
            }
        }
    }
    println!("{}", r);
}
