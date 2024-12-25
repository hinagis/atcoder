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
        d: [(i32, i32, i32); n],
    }
    let mut dp = vec![(0, 0, 0); n];
    dp[0].0 = d[0].0;
    dp[0].1 = d[0].1;
    dp[0].2 = d[0].2;
    for i in 1..n {
        dp[i].0 = d[i].0 + max(dp[i - 1].1, dp[i - 1].2);
        dp[i].1 = d[i].1 + max(dp[i - 1].0, dp[i - 1].2);
        dp[i].2 = d[i].2 + max(dp[i - 1].0, dp[i - 1].1);
    }

    println!("{}", max(max(dp[n - 1].0, dp[n - 1].1), dp[n - 1]. 2));
}
