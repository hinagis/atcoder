use itertools::Itertools;
use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        t: usize
    }
    for _ in 0..t {
        I! {
            n: usize,
            e: [(U, U)]
        }
        let mut g = vec![vec![]; n];
        for (u, v) in e {
            g[u].push(v);
            g[v].push(u);
        }
        if !f(&g, &mut vec![None; n], &mut vec![], 0, 0) {
            println!("-1");
        }
    }
}

fn f(g: &Vec<Vec<usize>>, c: &mut [Option<i32>], h: &mut Vec<usize>, u: usize, uc: i32) -> bool {
    h.push(u + 1);
    c[u] = Some(uc);
    for &v in g[u].iter() {
        if c[v] == None {
            if f(g, c, h, v, uc ^ 1) {return true}
        } else if c[v] == c[u] {
            let s = h.iter().position(|&w| w == v + 1).unwrap();
            println!("{}\n{}", h.len() - s, h[s..].iter().join(" "));
            return true;
        }
    }
    h.pop();
    false
}
