use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut v: Vec<Option<(usize, Option<usize>)>> = vec![None; n];
    let mut q = vec![VecDeque::new(); m];
    for i in 0..m {
        input! {k: usize};
        for _ in 0..k {
            input!{a: Usize1}
            if let Some(j) = v[a] {
                if j.0 == i {
                    println!("No");
                    return;
                } else {
                    v[a] = Some((j.0, Some(i)));
                    q[i].push_front(a);
                }
            } else {
                v[a] = Some((i, None));
                q[i].push_front(a);
            }
        }
        dfs(&v, &mut q, i);
    }
    for i in 0..m {
        if ! q[i].is_empty() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn dfs(v: &Vec<Option<(usize, Option<usize>)>>, q: &mut Vec<VecDeque<usize>>, i: usize) {
    if ! q[i].is_empty() {
        let a = q[i][0];
        if let Some(j) = v[a] {
            if i == j.0 {
                if let Some(j) = j.1 {
                    if q[j][0] == a {
                        q[i].remove(0);
                        q[j].remove(0);
                        dfs(v, q, i);
                        dfs(v, q, j);
                    }
                }
            } else {
                let j = j.0;
                if q[j][0] == a {
                    q[i].remove(0);
                    q[j].remove(0);
                    dfs(v, q, i);
                    dfs(v, q, j);
                }
            }
        }
    }
}