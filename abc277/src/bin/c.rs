use proconio::input as I;
use std::collections::HashMap as H;

fn main() {
    I! {n: usize}
    let mut t = H::new();
    for _ in 0..n {
        I! {
            a: usize,
            b: usize
        }
        t.entry(a).or_insert(vec![]).push(b);
        t.entry(b).or_insert(vec![]).push(a);
    }
    let mut h = H::new();
    println!("{}", calc(&mut t, &mut h, 1, 1));
}

fn calc(t: &mut H<usize, Vec<usize>>, h: &mut H<usize, usize>, i: usize, s: usize) -> usize {
    let mut m = i.max(s);
    h.insert(i, m);
    for j in t.get(&i).unwrap_or(&vec![]).clone() {
        if ! h.contains_key(&j) {
            m = calc(t, h, j, m).max(m);
        }
    }
    m
}