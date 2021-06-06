fn main() {
    proconio::input! {
        n: usize,
        mut t: [i32; n]
    }
    let a = t.iter().fold(0, |s, &t| s + t);
    let h = a - a / 2;
    t.sort_by(|a, b| b.cmp(a));
    println!("{}", calc(&t, &n, &a, &h, t[0], 0));
}

fn calc(t: &Vec<i32>, n: &usize, a: &i32, h: &i32, mut s: i32, i: usize) -> i32 {
    if s >= a / 2 {
        if s == a / 2 {
            s = *h
        }
    } else {
        let ss = s;
        for j in (i + 1)..*n {
            let c = calc(t, n, a, h, ss + t[j], j);
            if (h - c).abs() < (h - s).abs() {
                s = c
            }
        }
    }
    s
}
