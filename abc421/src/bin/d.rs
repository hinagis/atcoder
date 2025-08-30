fn main() {
    proconio::input! {
        mut t: (i64, i64),
        mut a: (i64, i64),
        n: i64,
        m: usize,
        l: usize,
        s: [(char, i64); m],
        b: [(char, i64); l]
    }
    let mut c = 0;
    let mut p = 0;
    let mut i = (0, 0);
    let mut j = (0, 0);
    while p < n {
        let d = s[i.0].1 - i.1;
        let e = b[j.0].1 - j.1;
        let f = d.min(e);
        let g = t.0 == a.0 && t.1 == a.1;
        match s[i.0].0 {
            'U' => {
                match b[j.0].0 {
                    'U' => {if g {c += f;}},
                    'D' => {if !g && (t.0 - a.0).abs() % 2 == 0 && t.1 == a.1 && t.0 - f <= a.0 + f {c += 1;}},
                    'L' => {if !g && (t.0 - a.0).abs() == (t.1 - a.1).abs() && t.0 - f <= a.0 && a.1 - f <= t.1 {c += 1;}},
                    _ =>   {if !g && (t.0 - a.0).abs() == (t.1 - a.1).abs() && t.0 - f <= a.0 && a.1 + f >= t.1 {c += 1;}},
                }
                t.0 -= f;
            },
            'D' => {
                match b[j.0].0 {
                    'U' => {if !g && (t.0 - a.0).abs() % 2 == 0 && t.1 == a.1 && t.0 + f >= a.0 - f {c += 1;}},
                    'D' => {if g {c += f;}},
                    'L' => {if !g && (t.0 - a.0).abs() == (t.1 - a.1).abs() && t.0 + f >= a.0 && a.1 - f <= t.1 {c += 1;}},
                    _ =>   {if !g && (t.0 - a.0).abs() == (t.1 - a.1).abs() && t.0 + f >= a.0 && a.1 + f >= t.1 {c += 1;}},
                }
                t.0 += f;
            },
            'L' => {
                match b[j.0].0 {
                    'U' => {if !g && (t.0 - a.0).abs() == (t.1 - a.1).abs() && t.1 - f <= a.1 && a.0 - f <= t.0 {c += 1;}},
                    'D' => {if !g && (t.0 - a.0).abs() == (t.1 - a.1).abs() && t.1 - f <= a.1 && a.0 + f >= t.0 {c += 1;}},
                    'L' => {if g {c += f;}},
                    _ =>   {if !g && (t.1 - a.1).abs() % 2 == 0 && t.0 == a.0 && t.1 - f <= a.1 + f {c += 1;}},
                }
                t.1 -= f;
            },
            _ => {
                match b[j.0].0 {
                    'U' => {if !g && (t.0 - a.0).abs() == (t.1 - a.1).abs() && t.1 + f >= a.1 && a.0 - f <= t.0 {c += 1;}},
                    'D' => {if !g && (t.0 - a.0).abs() == (t.1 - a.1).abs() && t.1 + f >= a.1 && a.0 + f >= t.0 {c += 1;}},
                    'L' => {if !g && (t.1 - a.1).abs() % 2 == 0 && t.0 == a.0 && t.1 + f >= a.1 - f {c += 1;}},
                    _ => {if g {c += f;}},
                }
                t.1 += f;
            },
        }
        match b[j.0].0 {
            'U' => {a.0 -= f;},
            'D' => {a.0 += f;},
            'L' => {a.1 -= f;},
            _ =>   {a.1 += f;},
        }
        p += f;
        if d < e {
            i.0 += 1;
            i.1 = 0;
            j.1 += f;
        } else if e < d {
            i.1 += f;
            j.0 += 1;
            j.1 = 0;
        } else {
            i.0 += 1;
            i.1 = 0;
            j.0 += 1;
            j.1 = 0;
        }
    }
    println!("{}", c);
}
