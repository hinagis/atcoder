fn main() {
    proconio::input! {
        s: proconio::marker::Chars,
    }

    let r = if s[0] == 'R' {
        if s[1] == 'R' {
            if s[2] == 'R' {
                3
            } else {
                2
            }
        } else {
            1
        }
    } else {
        if s[1] == 'R' {
            if s[2] == 'R' {
                2
            } else {
                1
            }
        } else {
            if s[2] == 'R' {
                1
            } else {
                0
            }
        }
    };
    println!("{}", r);
}
