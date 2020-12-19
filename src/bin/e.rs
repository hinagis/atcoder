use proconio::input;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {
            n: usize,
            mut s: usize,
            k: usize,
        }
        let mut ns = s + k;
        if ns >= n {
            ns -= n;
        }
        if ns == 0 {
            println!("1");
        } else if ns == s {
            println!("-1");
        } else if ns > s {
            let mut r = 0;
            let mut dp = vec!{false; n};
            let d = ns - s;
            while dp[s] == false {
                dp[s] = true;
                let l = (n - s) / d;
                r += l;
                s += d * l;
                if s < n {
                    s += d;
                    r += 1;
                }
                s -= n;
                if s == 0 {
                    println!("{}", r);
                    break;
                }
                if dp[s] {
                    println!("-1");
                    break;
                }
            }
        } else {
            let mut r = 0;
            let mut dp = vec!{false; n};
            let d = s - ns;
            while dp[s] == false {
                dp[s] = true;
                let l = s / d;
                r += l;
                s -= d * l;
                if s > 0 {
                    s += n;
                    s -= d;
                    r += 1;
                }
                if s == 0 {
                    println!("{}", r);
                    break;
                }
                if dp[s] {
                    println!("-1");
                    break;
                }
            }
        }
    }
}
