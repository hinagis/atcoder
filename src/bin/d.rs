fn main() {
    proconio::input! {
        n: usize,
        m: u64,
        x: [u64; n]
    }
    let mut l = x.iter().fold(0, |s, &x| if x > s {x} else {s});
    let mut r = 1000_000_000_000_000u64;
    while l < r {
        let c = (l + r) / 2;
        dbg!(l, r, c);
        if calc(&x, c) > m {
            l = c + 1;
        } else {
            r = c;
        }
    }

    println!("{}", r);
}

fn calc(l: &Vec<u64>, w: u64) -> u64 {
    let mut r = 1;
    let mut c = 0;
    for &l in l {
        if c == 0 {
            c = l;
        } else if c + 1 + l > w {
            r += 1;
            c = l;
        } else {
            c += 1 + l;
        }
    }
    r
}