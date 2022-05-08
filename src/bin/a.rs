fn main() {
    proconio::input! {
        h: u32,
        w: u32,
        r: u32,
        c: u32,
    }

    if h == 1 {
        if w == 1 {
            println!("0");
        } else if w == 2 || c == 1 || c == w {
            println!("1");
        } else {
            println!("2");
        }
    } else if h == 2 {
        if w == 1 {
            println!("1");
        } else if w == 2 || c == 1 || c == w {
            println!("2");
        } else {
            println!("3");
        }
    } else {
        if w == 1 {
            if r == 1 || r == h {
                println!("1");
            } else {
                println!("2");
            }
        } else if w == 2 || c == 1 || c == w {
            if r == 1 || r == h {
                println!("2");
            } else {
                println!("3");
            }
        } else {
            if r == 1 || r == h {
                println!("3");
            } else {
                println!("4");
            }
        }
    }
}
