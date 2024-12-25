use proconio::{input as I, marker::Chars as C};
fn main() {
    I! {
        h: i32,
        w: i32,
    }
    let mut p = vec![];
    for i in 0..h {
        I! {s: C}
        for j in 0..w {
            if s[j as usize] == 'o' {
                p.push((i, j));
            }
        }
    }
    println!("{}", (p[0].0 - p[1].0).abs() + (p[0].1 - p[1].1).abs());
}
