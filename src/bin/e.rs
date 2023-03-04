use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        e: [(U, U)],
    }

    let mut t = vec![vec![]; n];
    for &(u, v) in &e {
        t[u].push(v);
    }

    let mut r = 0;
    for i in 0..n {
        let mut f = vec![false; n];
        f[i] = true;
        let mut q = std::collections::VecDeque::new();
        q.push_back(i);
        while let Some(j) = q.pop_front() {
            for &k in &t[j] {
                if f[k] {continue}
                f[k] = true;
                q.push_back(k);
                r += 1;
            }
        }
    }        
    
    println!("{}", r - e.len());
}
