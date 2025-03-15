fn main() {
    proconio::input! {n: u128}
    let (mut x, mut y) = (2u128, 1u128);
    loop {
        if x.pow(3) - y.pow(3) == n {
            println!("{x} {y}");
            break;
        }

        if x.pow(3) - y.pow(3) > n {
            if x - y == 1 {
                println!("-1");
                break;
            } else {
                let m = (x + y) / 2;
                if x.pow(3) - m.pow(3) >= m {
                    y = m;
                } else {
                    y += 1;
                }
            }
        } else {
            x += 1;
        }
    }
}
