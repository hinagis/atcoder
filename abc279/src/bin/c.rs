use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h],
        t: [C; h],
    }
    let s = trans(&s, h, w);
    let t = trans(&t, h, w);
    let mut v = std::collections::HashMap::new();
    for i in 0..w {
        *v.entry(&s[i]).or_insert(0) += 1;
    }
    for i in 0..w {
        if let Some(e) = v.get_mut(&t[i]) {
            if *e > 0 {
                *e -= 1;
            } else {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn trans(s: &Vec<Vec<char>>, h: usize, w: usize) -> Vec<String> {
    let mut t = vec![String::with_capacity(h); w];
    for i in 0..w {
        for j in 0..h {
            t[i].push(s[j][i]);
        }
    }
    t
}
