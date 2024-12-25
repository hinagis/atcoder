use proconio::{input, source::line::LineSource};
use std::io::{BufReader, stdin};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(stdin()));
    macro_rules! I(($($tt:tt)*) => (input!(from &mut stdin, $($tt)*)));
    I! {_: usize}
    let mut f = [false; 2002];
    let mut n = 1;
    loop {
        println!("{}", n);
        I!{a: usize}
        if a == 0 {return}
        f[a] = true;
        n += 1;
        while f[n] {n += 1}
    }
}
