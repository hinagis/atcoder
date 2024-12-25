use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    println!("{}", if s[0] == '0' {
        let mut c = [false; 7];
        c[0] = s[6] == '1';
        c[1] = s[3] == '1';
        c[2] = s[1] == '1' || s[7] == '1';
        c[3] = s[4] == '1';
        c[4] = s[2] == '1' || s[8] == '1';
        c[5] = s[5] == '1';
        c[6] = s[9] == '1';
        if c[0] {
            if c[6] {
                if c[1] && c[2] && c[3] && c[4] && c[5] {"No"} else {"Yes"}
            } else if c[5] {
                if c[1] && c[2] && c[3] && c[4] {"No"} else {"Yes"}
            } else if c[4] {
                if c[1] && c[2] && c[3] {"No"} else {"Yes"}
            } else if c[3] {
                if c[1] && c[2] {"No"} else {"Yes"}
            } else if c[2] {
                if c[1] {"No"} else {"Yes"}
            } else {
                "No"
            }
        } else if c[1] {
            if c[6] {
                if c[2] && c[3] && c[4] && c[5] {"No"} else {"Yes"}
            } else if c[5] {
                if c[2] && c[3] && c[4] {"No"} else {"Yes"}
            } else if c[4] {
                if c[2] && c[3] {"No"} else {"Yes"}
            } else if c[3] {
                if c[2] {"No"} else {"Yes"}
            } else {
                "No"
            }
        } else if c[2] {
            if c[6] {
                if c[3] && c[4] && c[5] {"No"} else {"Yes"}
            } else if c[5] {
                if c[3] && c[4] {"No"} else {"Yes"}
            } else if c[4] {
                if c[3] {"No"} else {"Yes"}
            } else {
                "No"
            }
        } else if c[3] {
            if c[6] {
                if c[4] && c[5] {"No"} else {"Yes"}
            } else if c[5] {
                if c[4] {"No"} else {"Yes"}
            } else {
                "No"
            }
        } else if c[4] {
            if c[6] {
                if c[5] {"No"} else {"Yes"}
            } else {
                "No"
            }
        } else {
            "No"
        }
    } else {
        "No"
    });
}
