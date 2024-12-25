use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut r = 0;
    for i in 0..(n - 1) {
        let (mut at, mut cg) = match s[i] {
            'A' => ( 1,  0),
            'T' => (-1,  0),
            'C' => ( 0,  1),
            'G' => ( 0, -1),
            _   => panic!(),
        };
        for j in (i + 1)..n {
            match s[j] {
                'A' => at += 1,
                'T' => at -= 1,
                'C' => cg += 1,
                'G' => cg -= 1,
                _   => panic!(),
            }
            if at == 0 && cg == 0 {
                r += 1;
            }
        }
    }
    println!("{}", r);
}
