use itertools::Itertools;
use proconio::{input as I, fastout as F, marker::Bytes as B};

#[F]
fn main() {
    I! {a: [B]}
    for s in a {
        let n = s.len();
        let v = s
            .into_iter()
            .sorted_unstable()
            .dedup_with_count()
            .sorted_unstable()
            .rev()
            .collect_vec();
        if v[0].0 > (n + 1) / 2 {
            println!("No");
            continue;
        }
        let mut t = vec![0; n];
        let mut i = 0;
        for (e, c) in v {
            for _ in 0..e {
                t[i] = c;
                i += 2;
                if i >= n {
                    i = 1;
                }
            }
        }
        println!("Yes");
        println!("{}", String::from_utf8(t).unwrap());
    }
}
