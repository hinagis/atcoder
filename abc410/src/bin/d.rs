use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize
    }
    let mut e = vec![vec![]; n];
    for _ in 0..m {
        I! {
            a: U,
            b: U,
            w: u32
        }
        e[a].push((b, w));
    }
    let mut r = vec![vec![]; n];
    r[0].push(0);
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0));
    while let Some((a, s)) = q.pop_front() {
        for &(b, w) in &e[a] {
            let s = s ^ w;
            if r[b].contains(&s) {continue}
            r[b].push(s);
            q.push_back((b, s));
        }
    }   

    if r[n - 1].is_empty() {
        println!("-1");
    } else {
        println!("{}", r[n - 1].iter().min().unwrap());
    }
}
