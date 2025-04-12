use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        k: usize,
        mut s: C
    }
    if n == 1 {
        if k == 1 {
            s[0] = 'o';
        } else {
            s[0] = '.';
        }
    } else {
        let mut o = s.iter().filter(|&&c| c == 'o').count();
        if o == k {
            for i in 0..n {
                if s[i] == '?' {
                    s[i] = '.';
                }
            }
            println!("{}", s.iter().collect::<String>());
            return;
        }

        if s[1] == 'o' {
            s[0] = '.';
        }
        for i in 1..n {
            if s[i - 1] == 'o' || (i < n - 1 && s[i + 1] == 'o') {
                s[i] = '.';
            }
        }
        let mut f = false;
        let mut p = 0;
        for i in 0..n {
            if s[i] == '?' {
                if !f {
                    p = i;
                }
                f = true;
            } else {
                if f {
                    o += (i + 1 - p) / 2;
                }
                f = false;
            }
        }
        if f {
            o += (n + 1 - p) / 2;
        }
        if o <= k {
            let mut f = false;
            let mut p = 0;
            for i in 0..n {
                if s[i] == '?' {
                    if !f {
                        p = i;
                    }
                    f = true;
                } else {
                    if f && (i - p) % 2 == 1 {
                        s[p] = 'o';
                        for j in (p + 1..i).step_by(2) {
                            s[j] = '.';
                            s[j + 1] = 'o';
                        }
                    }
                    f = false;
                }
            }
            if f && (n - p) % 2 == 1 {
                s[p] = 'o';
                for j in (p + 1..n).step_by(2) {
                    s[j] = '.';
                    s[j + 1] = 'o';
                }
            }
        }
    }
    println!("{}", s.iter().collect::<String>());
}
