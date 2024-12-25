use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let n = s.len();
    println!("{}", if k > 0 {
        let mut l = 0;
        let mut r = 0;
        while r < n && s[r] != '.' {
            r += 1;
        }
        let mut m = k;
        while r < n && m > 0 {
            if s[r] == '.' {
                m -= 1
            }
            r += 1;
        }
        while r < n && s[r] != '.' {
            r += 1;
        }
        let mut m = r;
        while r < n {
            while s[l] != '.' {
                l += 1;
            }
            l += 1;
            r += 1;
            while r < n && s[r] != '.' {
                r += 1;
            }
            m = m.max(r - l);
        }
        m
    } else {
        let mut m = 0;
        let mut l = 0;
        while l < n {
            while l < n && s[l] == '.' {
                l += 1;
            }
            let mut r = l;
            while r < n && s[r] != '.' {
                r += 1;
            }
            m = m.max(r - l);
            l = r;
        }
        m
    });
}
