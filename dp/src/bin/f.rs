//use proconio::input;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

const SIZE: usize = 3000;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut dp = vec![vec![(0, None); SIZE + 1]; SIZE + 1];

    let mut f = false;
    if s[0] == t[0] {
        f = true;
        dp[0][0] = (1, None);
    }
    let mut fi = f;
    for i in 1..s.len() {
        if s[i] == t[0] {
            fi = true;
        }
        if fi {
            dp[i][0] = (1, Some((i - 1, 0)));
        }
    }
    for j in 1..t.len() {
        if s[0] == t[j] {
            f = true;
        }
        if f {
            dp[0][j] = (1, Some((0, j - 1)));
        }
    }
    for i in 1..s.len() {
        for j in 1..t.len() {
            let mut dpij = (0, None);
            if s[i] == t[j] {
                dpij = (dp[i - 1][j - 1].0 + 1, Some((i - 1, j - 1)));
            }
            if dp[i][j - 1].0 > dpij.0 {
                dpij = (dp[i][j - 1].0, Some((i, j - 1)));
            }
            if dp[i - 1][j].0 > dpij.0 {
                dpij = (dp[i - 1][j].0, Some((i - 1, j)));
            }
            dp[i][j] = dpij;
        }
    }

    // traceback
    let mut r = String::new();
    let mut dpij = Some((s.len() - 1, t.len() - 1));
    while let Some(ij) = dpij {
        dpij = dp[ij.0][ij.1].1;
        if let Some(next) = dpij {
            if dp[ij.0][ij.1].0 > dp[next.0][next.1].0 {
                r = [s[ij.0].to_string(), r].concat();
            }
        }
    }

    if dp[0][0].0 > 0 {
        r = [s[0].to_string(), r].concat();
    }
    println!("{}", r);
}
