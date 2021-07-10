use proconio::{input, fastout, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        cd: [(Usize1, Usize1); q]
    }
    let mut t = vec![Vec::with_capacity(n); n];
    for &(a, b) in &ab {
        t[a].push(b);
        t[b].push(a);
    }

    let mut r = vec![None; n];
    r[0] = Some(0);
    let mut que = std::collections::VecDeque::new();
    que.push_back(0);
    while let Some(i) = que.pop_front() {
        for &j in &t[i] {
            if r[j] == None {
                r[j] = Some(r[i].unwrap() + 1);
                que.push_back(j);
            }
        }
    }

    for &(c, d) in &cd {
        println!("{}", if r[c].unwrap() % 2 == r[d].unwrap() % 2 {"Town"} else {"Road "});
    }
}
