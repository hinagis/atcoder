fn main() {
    proconio::input! {
        n: usize,
        s: String,
    }
    let c: Vec<char> = s.chars().collect();
    let mut wl = 0;
    while wl < n && c[wl] != 'W' {
        wl += 1;
    }
    let mut rr = n - 1;
    while rr > 0 && c[rr] != 'R' {
        rr -= 1;
    }

    let mut r = 0;
    while wl < rr {
        r += 1;
        wl += 1;
        while wl < n && c[wl] != 'W' {
            wl += 1;
        }
        rr -= 1;
        while rr > 0 && c[rr] != 'R' {
            rr -= 1;
        }
    }
    println!("{}", r);
}
