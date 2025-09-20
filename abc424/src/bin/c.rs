fn main() {
    proconio::input! {
        n: usize,
        p: [(usize, usize); n]
    }
    let mut f = vec![false; n];
    for i in 0..n {
        if p[i].0 == 0 {
            f[i] = true;
        }
    }
    let mut t = vec![vec![]; n];
    for i in 0..n {
        if f[i] {continue}
        let (a, b) = p[i];
        t[a - 1].push(i);
        t[b - 1].push(i);
    }
    let mut q = std::collections::VecDeque::new();
    for i in 0..n {
        if f[i] {continue}
        let (a, b) = p[i];
        if f[a - 1] || f[b - 1] {
            f[i] = true;
            q.push_back(i);
        }
    }
    while let Some(i) = q.pop_front() {
        for &j in &t[i] {
            if f[j] {continue;}
            f[j] = true;
            q.push_back(j);
        }
    }

    println!("{}", f.iter().filter(|&&x| x).count());
}
