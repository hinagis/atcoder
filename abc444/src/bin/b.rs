use proconio::input as I;

fn main() {
    I! {
        n: usize,
        k: usize
    }
    let mut c = 0;
    for i in 1..=n {
        if calc(i) == k {
            c += 1;
        }
    }
    println!("{}", c);
}

fn calc(mut n: usize) -> usize {
    let mut s = 0;
    while n > 0 {
        s += n % 10;
        n /= 10;
    }
    s
}