use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize, usize); n]
    }
    let mut r = 0;
    for i in 0..w {
        r |= 1 << i;
    }

    for p in (0..n).permutations(n) {
        if calc(n, h, w, &ab, r, &p, &mut vec![0; h], 0) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn get_space(
    h: usize,
    w: usize,
    r: u32,
    s: &mut Vec<u32>
) -> Option<(usize, usize)> {
    for y in 0..h {
        if s[y] != r {
            for x in 0..w {
                if s[y] & (1 << x) == 0 {
                    return Some((y, x));
                }
            }
        }
    }
    None
}

fn calc(
    n: usize,
    h: usize,
    w: usize,
    ab: &Vec<(usize, usize)>,
    r: u32,
    p: &Vec<usize>,
    s: &mut Vec<u32>,
    i: usize
) -> bool {
    if let Some((y, x)) = get_space(h, w, r, s) {
        if i == n {
            return false;
        }
        let (a, b) = ab[p[i]];
        if y + a <= h && x + b <= w {
            let mut s = s.clone();
            let mut f = true;
            'outer: for v in y..y + a {
                for u in x..x + b {
                    if s[v] & (1 << u) == 0 {
                        s[v] |= 1 << u;
                    } else {
                        f = false;
                        break 'outer;
                    }
                }
            }
            if f {
                if calc(n, h, w, ab, r, p, &mut s, i + 1) {
                    return true;
                }
            }
        }
        if y + b <= h && x + a <= w {
            let mut s = s.clone();
            let mut f = true;
            'outer: for v in y..y + b {
                for u in x..x + a {
                    if s[v] & (1 << u) == 0 {
                        s[v] |= 1 << u;
                    } else {
                        f = false;
                        break 'outer;
                    }
                }
            }
            if f {
                if calc(n, h, w, ab, r, p, &mut s, i + 1) {
                    return true;
                }
            }
        }
        false
    } else {
        true
    }
}
