use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    let mut t = s.clone();
    let mut f = true;
    for i in 0..s.len() {
        if s[i] == '#' {
            f = true;
            continue;
        }
        if f {
            t[i] = 'o';
            f = false;
        }
    }
    println!("{}", t.iter().collect::<String>());
}
