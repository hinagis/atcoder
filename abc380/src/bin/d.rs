use itertools::Itertools;
use proconio::{input as I, marker::{Usize1 as U, Chars as C}};

macro_rules! t {
    ($c: ident) => {
        if $c.is_ascii_lowercase() {$c.to_ascii_uppercase()} else {$c.to_ascii_lowercase()}
    }
}

fn main() {
    I! {
        s: C,
        q: usize
    }
    let mut r = Vec::with_capacity(q);
    for _ in 0..q {
        I! {k: U}
        let d = k / s.len();
        let c = s[k % s.len()];
        r.push(if d.count_ones() % 2 == 0 {c} else {t!(c)});
    }
    println!("{}", r.iter().join(" "));
}
