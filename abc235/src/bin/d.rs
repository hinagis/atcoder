fn main() {
    proconio::input! {a: usize, n: usize}
    let mut m = usize::max_value();
    let mut f = vec![usize::max_value(); 1_000_000];
    f[n] = 0;
    let mut q = std::collections::VecDeque::new();
    q.push_back((n, 0));
    while let Some((n, i)) = q.pop_front() {
        if n == a {
            m = m.min(i + 1)
        } else {
            if n % a == 0 {
                let n = n / a;
                if f[n] > i + 1 {
                    f[n] = i + 1;
                    q.push_back((n, i + 1));
                }
            }
            let mut n = n.to_string().chars().collect::<Vec<_>>();
            if n.len() >= 2 && n[1] != '0' {
                n.push(n[0]);
                n.remove(0);
                let n = n.iter().collect::<String>().parse::<usize>().unwrap();
                if f[n] > i + 1 {
                    f[n] = i + 1;
                    q.push_back((n, i + 1));
                }
            }
        }
    }

    if m == usize::max_value() {
        println!("-1")
    } else {
        println!("{}", m)
    }
}
