fn main() {
    proconio::input! {
        n: usize,
    }
    let mut r = 0;
    for i in 1..=n {
        let mut f = true;
        for c in i.to_string().chars() {
            if c == '7' {
                f = false;
                break;
            }
        }
        let mut o = i;
        while o > 0 {
            if o % 8 == 7 {
                f = false;
                break;
            }
            o /= 8;
        }
        if f {
            r += 1;
        }
    }

    println!("{}", r);
}
