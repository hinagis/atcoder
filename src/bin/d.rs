//use proconio::input;
use std::cmp::max;

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

fn main() {
    input! {
        n: usize,
        w: usize,
        d: [(usize, u64); n],
    }
    let mut dp = vec![vec![0; w + 1]; n];
    for j in d[0].0..(w + 1) {
        dp[0][j] = d[0].1;
    }
    for i in 1..n {
        for j in 0..(w + 1) {
            if j >= d[i].0 {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - d[i].0] + d[i].1);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    println!("{}", dp[n - 1].iter().max().unwrap());
}
