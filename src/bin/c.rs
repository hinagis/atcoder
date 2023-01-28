use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize
    }

    let mut t = vec![vec![]; n];
    for _ in 0..m {
        I! {
            u: U,
            v: U
        }
        t[u].push(v);
        t[v].push(u);
    }

    let mut f = vec![false; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    while let Some(i) = q.pop_front() {
        if f[i] {
            println!("No");
            return;
        }
        f[i] = true;
        for &j in &t[i] {
            if ! f[j] {
                q.push_back(j);
            }
        }
    }
    println!("{}", if f.iter().filter(|&&c| c).count() == n {"Yes"} else {"No"});
}
