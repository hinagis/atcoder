fn main() {
    proconio::input! {
        n: usize,
        m: usize,
    }

    let mut d = Vec::new();
    for i in -400..=400 {
        for j in -400..=400 {
            if (i * i + j * j) as usize == m {
                d.push((i, j));
            }
        }
    }

    let mut f = vec![vec![-1; n]; n];
    f[0][0] = 0;
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0));

    while let Some((i, j)) = q.pop_front() {
        for &(u, v) in &d {
            let k = i as isize + u;
            let l = j as isize + v;
            if 0 <= k && k < n as isize && 0 <= l && l < n as isize {
                let k = k as usize;
                let l = l as usize;
                if f[k][l] == -1 {
                    f[k][l] = f[i][j] + 1;
                    q.push_back((k, l));
                }
            }
        }
    }

    for i in 0..n {
        println!("{}", f[i].iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
    }
}
