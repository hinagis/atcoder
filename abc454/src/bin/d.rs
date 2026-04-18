use proconio::{input as I, fastout as F, marker::Chars as C};
use std::collections::VecDeque as Q;

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            a: C,
            b: C
        }
        let u = calc(&a);
        let v = calc(&b);

        println!("{}", if u == v {"Yes"} else {"No"});
    }
}

fn calc(s: &Vec<char>) -> Q<char> {
    let mut u = Q::new();
    for &c in s {
        if c == ')' && u.len() > 2
        && *u.iter().nth_back(2).unwrap() == '('
        && *u.iter().nth_back(1).unwrap() == 'x'
        && *u.iter().nth_back(0).unwrap() == 'x' {
            u.pop_back();
            u.pop_back();
            u.pop_back();
            u.push_back('x');
            u.push_back('x');
        } else {
            u.push_back(c);
        }
    }
    u
}