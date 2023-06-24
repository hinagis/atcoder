fn main() {
    proconio::input! {
        _n: usize,
        mut s: String
    }
    let re = regex::Regex::new(r"\([^\(\)]*\)").unwrap();
    while true {
        let t = re.replace_all(&s, "");
        if s == t {
            println!("{}", s);
            return;
        }
        s = t.to_string();
    }
}
