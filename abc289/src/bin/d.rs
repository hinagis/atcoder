fn main() {
    proconio::input! {
        a: [usize],
        b: [usize],
        x: usize
    }
    let mut f = vec![true; x + 1];
    for &b in &b {
        f[b] = false;
    }

    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    while let Some(e) = q.pop_front() {
        if e == x {
            println!("Yes");
            return;
        }
        for a in &a {
            if e + a > x {
                continue;
            }
            if f[e + a] {
                f[e + a] = false;
                q.push_back(e + a);
            }
        }
    }
    println!("No");
}
