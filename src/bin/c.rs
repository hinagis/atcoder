use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        tab: [(usize, usize, usize); q]
    }

    let mut f = false;
    for i in 0..q {
        let (t, a, b) = tab[i];
        if t == 1 {
            let (a, b) = if f {
                let a = if a > n {a - n} else {a + n};
                let b = if b > n {b - n} else {b + n};
                (a, b)
            } else {
                (a, b)
            };
            s.swap(a - 1, b - 1)
        } else {
            f = !f;
        }
    }
    if f {
        println!("{}{}", s[n..(2 * n)].iter().collect::<String>(), s[0..n].iter().collect::<String>());
    } else {
        println!("{}", s.iter().collect::<String>());
    }
}
