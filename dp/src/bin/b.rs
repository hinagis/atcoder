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

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n],
    }
    let k = min(n, k);
    let mut dp = vec![0; n];
    for i in 1..k {
        let mut dpi = dp[i - 1] + (h[i] - h[i - 1]).abs();
        for j in 2..(i + 1) {
            dpi = min(dpi, dp[i - j] + (h[i] - h[i - j]).abs());
        }
        dp[i] = dpi;
    }
    for i in k..n {
        let mut dpi = dp[i - 1] + (h[i] - h[i - 1]).abs();
        for j in 2..(k + 1) {
            dpi = min(dpi, dp[i - j] + (h[i] - h[i - j]).abs());
        }
        dp[i] = dpi;
    }

    println!("{}", dp[n - 1]);
}
