fn main() {
    proconio::input! {
        n: usize,
        x: u32,
        y: u32,
        a: [u32; n],
    }

    let mut c = 0;
    let mut l = 0;
    while l < n {
        let mut b = vec![];
        while l < n && a[l] >= y && a[l] <= x {
            b.push(a[l]);
            l += 1;
        }
        if b.len() > 0 {
            c += calc(x, y, &b);
        } else {
            l += 1;
        }
    }

    println!("{}", c);
}

fn calc(x: u32, y: u32, b: &Vec<u32>) -> u64 {
    let mut c = 0;
    let mut l = 0;
    let mut r = 0;
    let mut u = 0;
    let mut v = 0;
    while l < b.len() {
        while r < b.len() && (u == 0 || v == 0) {
            u += if b[r] == x {1} else {0};
            v += if b[r] == y {1} else {0};
            r += 1;
        }
        if u > 0 && v > 0 {
            c += (b.len() + 1 - r) as u64
        }
        u -= if b[l] == x {1} else {0};
        v -= if b[l] == y {1} else {0};
        l += 1
    }
    c
}
