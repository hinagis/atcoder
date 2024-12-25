use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        a: [U]
    }

    let mut d = vec![false; n];
    for &a in &a {
        d[a] = true;
    }

    let mut r = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        if d[i] {
            let mut j = i + 1;
            while d[j] {
                j += 1;
            }
            for k in (i..=j).rev() {
                r.push(k + 1);
            }
            i = j + 1;
        } else {
            r.push(i + 1);
            i += 1;
        }
    }
    println!("{}", r.iter().map(|&i| i.to_string()).collect::<Vec<String>>().join(" "));
}
