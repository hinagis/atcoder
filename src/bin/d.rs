use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut a: i64,
        mut b: i64,
        k: i64,
    }

    let mut l = 0;
    a -= 1;
    let mut v = a + b;
    for _ in 0..(a + b) {
        let c = comb(v, a);
        if k <= l + c {
            print!("{}", 'a');
            a -= 1;
        } else {
            print!("{}", 'b');
            l += c;
        }
        v -= 1;
    }
    println!("{}", if a == 0 {'a'} else {'b'});
}

fn comb(n: i64, r: i64) -> i64 {
    (0..n + 1)
        .rev()
        .zip(1..r + 1)
        .fold(1, |mut r, (n, d)| { r *= n; r /= d; r })
}
