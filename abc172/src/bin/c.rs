fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut k: u64,
        a: [u64; n],
        b: [u64; m],
    }

    let mut ni = 0;
    while ni < n && a[ni] <= k {
        k -= a[ni];
        ni += 1;
    }

    let mut mi = 0;
    while mi < m && b[mi] <= k {
        k -= b[mi];
        mi += 1;
    }

    let mut nms = ni + mi;
    if nms == n + m {
        println!("{}", nms);
    } else {
        let mut r = nms;

        while ni > 0 {
            ni -= 1;
            k += a[ni];
            nms -= 1;
            while mi < m && b[mi] <= k {
                k -= b[mi];
                mi += 1;
                nms += 1;
            }
            r = std::cmp::max(r, nms);
        }
    
        println!("{}", r);
    }
}
