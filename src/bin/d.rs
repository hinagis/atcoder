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
    for i in 0..n {
        if calc(n, h, w, &ab, r, &mut vec![0; h], i) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn calc(
    n: usize,
    h: usize,
    w: usize,
    ab: &Vec<(usize, usize)>,
    r: u32,
    s: &mut Vec<u32>,
    i: usize
) -> bool {
    if i == n {
        for y in 0..h {
            if s[y] != r {
                return false;
            }
        }
        return true;
    }

    let (a, b) = ab[i];
    for y in 0..h {
        for x in 0..w {
            if y + a <= h && x + b <= w {
                let mut s = s.clone();
                let mut f = true;
                'outer: for v in y..y + a {
                    for u in x..x + b {
                        if s[v] & (1 << u) == 0 {
                            s[v] |= 1 << u;
                        } else {
                            f = false;
                            for y in 0..h {
                                if s[y] != r {
                                    break 'outer;
                                }
                            }
                            return true;
                        }
                    }
                }
                if f {
                    for i in i + 1..=n {
                        if calc(n, h, w, ab, r, &mut s, i) {
                            return true;
                        }
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
                            for y in 0..h {
                                if s[y] != r {
                                    break 'outer;
                                }
                            }
                            return true;
                        }
                    }
                }
                if f {
                    for i in i + 1..=n {
                        if calc(n, h, w, ab, r, &mut s, i) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}
