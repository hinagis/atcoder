fn main() {
    proconio::input! {n: u128}

    let mut p = vec![true; 5000_000];
    for i in (4..p.len()).step_by(2) {
        p[i] = false;
    }

    let mut c = 0;
    let mut i = 3;
    while 2 * i * i * i <= n {
        if p[i as usize] {
            let mut j = i * 2;
            while j < p.len() as u128 {
                p[j as usize] = false;
                j += i;
            }
            let mut j = 2;
            while j < i && j * i * i * i <= n {
                if p[j as usize] {
                    c += 1;
                }
                j += 1;
            }
        }
        i += 2;
    }
    println!("{}", c);
}
