use proconio::{input as I, marker::Chars as C};

fn main(){
    I! {
        n: usize,
        a: i32,
        b: i32,
        s: C
    }

    let mut r = 0;
    let mut c = [0, 0];
    let mut i = (0, 0);
    'f: for &x in &s {
        while c[0] < a {
            if i.0 == n {
                break 'f
            }
            if s[i.0] == 'a' {c[0] += 1}
            i.0 += 1;
        }
        while c[1] < b && i.1 < n {
            if s[i.1] == 'b' {c[1] += 1}
            i.1 += 1;
        }
        r += if i.1 < i.0 {0} else {i.1 - i.0 + if c[1] < b {1} else {0}};

        c[if x == 'a' {0} else {1}] -= 1;
    }
    println!("{r}");
}
