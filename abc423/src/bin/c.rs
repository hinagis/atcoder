fn main() {
    proconio::input! {
        n: usize,
        r: usize,
        l: [u32; n]
    }
    let mut a = 0;
    while a < r && l[a] == 1 {
        a += 1;
    }
    let mut c = (0, 0);
    while a < r {
        c.0 += (l[a] == 0) as u32;
        c.1 += l[a] as u32;
        a += 1;
    }
    let mut b = n;
    while b > r && l[b - 1] == 1 {
        b -= 1;
    }
    while b > r {
        c.0 += (l[b - 1] == 0) as u32;
        c.1 += l[b - 1] as u32;
        b -= 1;
    }
    println!("{}", c.0 + c.1 * 2);
}
