fn main() {
    proconio::input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let e = c - a;
    let f = d - b;
    let g = e % 4;
    let h = f % 4;
    let mut r = (e - g) * (f - h);
    match a % 4 {
        0 => {
            if b % 2 == 0 {
                r += (e - g) * h;
                match g {
                    0 => {},
                    1 => {
                        r += 6 * (f - h);
                        match h {
                            0 => {},
                            1 => {r += 2},
                            2 => {r += 3},
                            _ => {r += 5},
                        }
                    },
                    2 => {
                        r += 12 * (f - h);
                        r += 3 * h;
                    },
                    _ => {
                        r += 14 * (f - h);
                        match h {
                            0 => {},
                            1 => {r += 3},
                            2 => {r += 7},
                            _ => {r += 10},
                        }
                    },
                }
            } else {
                match g {
                    0 => {},
                    1 => {},
                    2 => {},
                    _ => {},
                }
            }
        },
        1 | -3 => {
            if b % 2 == 0 {

            } else {

            }
        },
        2 | -2 => {
            if b % 2 == 0 {

            } else {
                
            }
        },
        _ => {
            if b % 2 == 0 {

            } else {
                
            }
        },
    }
    println!("{}", r);
}
