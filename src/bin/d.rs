use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        x: u64,
        y: u64,
        z: u64,
        s: C
    }
    let mut t = [0; 2];
    t[0] = if s[0] == 'a' {x.min(2 * z + y)} else {y.min(2 * z + x)};
    t[1] = z + x.min(y);
    for i in 1..s.len() {
        let l = t[0].min(t[1] + z);
        let r = t[1].min(t[0] + z);
        if s[i] == 'a' {
            t[0] = (l + x.min(2 * z + y)).min(r + z + x.min(y));
            t[1] = (r + y.min(2 * z + x)).min(l + z + x.min(y));
        } else {
            t[0] = (l + y.min(2 * z + x)).min(r + z + x.min(y));
            t[1] = (r + x.min(2 * z + y)).min(l + z + x.min(y));
        }
    }
    println!("{}", t[0].min(t[1]));
}
