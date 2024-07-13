fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n]
    }
    let mut v = vec![0; n];
    v[0] = n as i64;
    if n == 1 {
        println!("{}", v[0]);
        return;
    }
    v[1] = v[0] * (v[0] - 1) / 2;
    for k in 2..n {
        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            for j in i + 1..n {
                q.push_back((j, a[j] - a[i], k - 2));
            }
        }
        while let Some((i, d, r)) = q.pop_front() {
            for j in i + 1..n {
                if a[j] - a[i] == d {
                    if r > 0 {
                        q.push_back((j, d, r - 1));
                    } else {
                        v[k] += 1; 
                    }
                }
            }
        }
    }
    println!("{}", v.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
}
