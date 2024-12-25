use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }
    let mut v = vec![];
    calc(n, &mut v);

    let mut r = 0;
    for e in v {
        r += get_count(e);
    }

    println!("{}", r);
}

fn calc(n: u64, v: &mut Vec<u64>) {
    fn sub(n: u64, m: u64) -> (u64, u64) {
        let mut c = 0;
        let mut r = n;
        while r % m == 0 {
            c += 1;
            r /= m;
        }
        (c, r)
    };

    let (c, n) = sub(n, 2);
    if c > 0 {
        v.push(c);
    }
    let mut m = 3;
    let mut r = n;
    while m * m <= r {
        let (c, n) = sub(r, m);
        if c > 0 {
            v.push(c);
        }
        r = n;
        m += 2;
    }
    if r > 1 {
        v.push(1);
    }
}

fn get_count(n: u64) -> u64 {
    if n <= 2 {
        1
    } else if n <= 2 + 3 {
        2
    } else if n <= 2 + 3 + 4 {
        3
    } else if n <= 2 + 3 + 4 + 5 {
        4
    } else if n <= 2 + 3 + 4 + 5 + 6 {
        5
    } else if n <= 2 + 3 + 4 + 5 + 6 + 7 {
        6
    } else if n <= 2 + 3 + 4 + 5 + 6 + 7 + 8 {
        7
    } else if n <= 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 {
        8
    } else {
        9
    }
}