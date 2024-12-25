use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n:usize,
        s:Chars
    }

    let mut q = std::collections::VecDeque::new();
    q.push_back(n);
    for i in (0..n).rev() {
        if s[i] == 'L' {
            q.push_back(i);
        } else {
            q.push_front(i);
        }
    }

    println!("{}", q.iter().join(" "));
}
