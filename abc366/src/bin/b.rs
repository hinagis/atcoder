use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        n: usize,
        s: [C; n]
    }
    let m = s.iter().fold(0, |m, s| m.max(s.len()));
    let mut t = vec![std::collections::VecDeque::new(); m];
    for j in 0..m {
        let mut f = false;
        for i in 0..n {
            if j >= s[i].len() {
                if f {
                    t[j].push_front('*');
                }
            } else {
                f = true;
                t[j].push_front(s[i][j]);
            }
        }
    }
    for j in 0..m {
        println!("{}", t[j].iter().collect::<String>());
    }
}
