use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        mut r: i32,
        mut c: i32,
        s: C
    }
    let mut f = std::collections::HashSet::new();
    let mut b = (0, 0);
    f.insert(b);
    let mut t = Vec::with_capacity(n);
    for s in s {
        match s {
            'N' => {b.0 += 1; r += 1},
            'W' => {b.1 += 1; c += 1},
            'S' => {b.0 -= 1; r -= 1},
            'E' => {b.1 -= 1; c -= 1},
            _ => unreachable!()
        }
        f.insert(b);
        t.push(if f.contains(&(r, c)) {'1'} else {'0'});
    }
    println!("{}", t.iter().collect::<String>());
}
