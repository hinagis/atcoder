fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut b = vec![false; 300_010];
    let mut c = 0;
    for &a in &a {
        if a > 300_000 || b[a] {
            c += 1;
        } else {
            b[a] = true;
        }
    }

    for i in 1..=300_000 {
        if ! b[i] {
            if c >= 2 {
                c -= 2;
                b[i] = true;
            } else {
                break;
            }
        }
    }

    let mut p = vec![];
    let mut l = 1;
    let mut r = 300_000;
    while l < r {
        if ! b[r] {
            r -= 1;
        } else {
            if ! b[l] {
                p.push(r);
                r -= 1;

                c += 1;
                if c >= 2 {
                    c -= 2;
                    while let Some(e) = p.pop() {
                        b[e] = false;
                    }

                    b[l] = true;
                    l += 1;
                }
            } else {
                l += 1;
            }
        }
    }

    l = 0;
    while b[l + 1] {
        l += 1;
    }
    println!("{}", l);
}
