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
    ($iter:expr, ( $($t:tt),* )) => {( $(read_value!($iter, $t)),* )};
    ($iter:expr, [ $t:tt ; $len:expr ]) => {(0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()};
    ($iter:expr, chars) => {read_value!($iter, String).chars().collect::<Vec<char>>()};
    ($iter:expr, usize1) => {read_value!($iter, usize) - 1};
    ($iter:expr, $t:ty) => {$iter.next().unwrap().parse::<$t>().expect("Parse error")};
}


fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m],
    }

    let mut g = vec![Vec::new(); n];
    for (x, y) in xy {
        g[x - 1].push(y - 1);
    }

    let mut r = 0;
    let mut dp = vec![None; n];
    for x in 0..n {
        r = max(r, rec(&g, &mut dp, x));
    }
    println!("{}", r);
}

fn rec(g: &Vec<Vec<usize>>, dp: &mut Vec<Option<usize>>, x: usize) -> usize {
    if let Some(dpx) = dp[x] { return dpx };

    let mut r = 0;
    for nx in &(*g)[x] {
        r = max(r, rec(g, dp, *nx) + 1);
    }
    (*dp)[x] = Some(r);
    r
}