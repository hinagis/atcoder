use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n < 4 {
        println!("impossible");
    } else if n % 2 == 0 {
        let m = n / 2;
        println!("{}", m);
        for i in 0..m {
            println!("2 {} {}", i * 2 + 1, (n - i) * 2 - 1);
        }
    } else {
        let m = || {
            let sum = n * n;
            for i in 2..(n / 2) {
                if sum % i == 0 {
                    return Some(i);
                }
            }
            None
        };
        if let Some(m) = m() {
            println!("{}", m);
            let l = n / m;
            for i in 0..m {
                print!("{}", l);
                for y in 0..m {
                    print!(" {}", (y * m + (i + y) % m) * 2 + 1);
                }
                for j in (0..((n - m * m) / 2)).skip(i).step_by(m) {
                    print!(" {} {}", (j + m * m) * 2 + 1, (n - j) * 2 - 1);
                }
                println!();
            }
        } else {
            println!("impossible");
        }
    }
}
