use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
    }

    let mut v = vec![0u32; w];
    for _ in 0..h {
        I! {c: C}
        for j in 0..w {
            if c[j] == '#' {
                v[j] += 1;
            }
        }
    }
    println!("{}", v.iter().map(|&c| c.to_string()).collect::<Vec<_>>().join(" "));
}
