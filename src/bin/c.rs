use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        s: String,
        t: C
    }
    let r = if t[2] == 'X' {
        format!("{}.*{}", t[0].to_ascii_lowercase(), t[1].to_ascii_lowercase())
    } else {
        format!("{}.*{}.*{}", t[0].to_ascii_lowercase(), t[1].to_ascii_lowercase(), t[2].to_ascii_lowercase())
    };
    let re = regex::Regex::new(&r).unwrap();

    println!("{}", if re.is_match(&s) {"Yes"} else {"No"});
}
