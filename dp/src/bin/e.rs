//use proconio::input;
use std::cmp::min;

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

const N_MAX: usize = 100;
const VI_MAX: usize = 1000;
const V_MAX: usize = VI_MAX * N_MAX;

fn main() {
    input! {
        n: usize,
        w: u64,
        d: [(u64, usize); n],
    }
    let mut dp = vec![vec![w + 1; V_MAX + 1]; n];
    let mut vi = d[0].1;
    for j in 0..(vi + 1) {
        dp[0][j] = d[0].0;
    }
    for i in 1..n {
        vi += d[i].1;
        for j in 0..(vi + 1) {
            if j > d[i].1 {
                dp[i][j] = min(dp[i - 1][j], dp[i - 1][j - d[i].1] + d[i].0);
            } else {
                dp[i][j] = min(dp[i - 1][j], d[i].0);
            }
        }
    }
    let mut r = 0;
    for j in 0..(V_MAX + 1) {
        if dp[n - 1][j] <= w {
            r = j;
        }
    }

    println!("{}", r);
}
