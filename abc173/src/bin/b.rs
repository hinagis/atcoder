fn main() {
    proconio::input! {
        n: usize,
        s: [String; n],
    }

    let mut dic = std::collections::HashMap::new();
    dic.entry("AC").or_insert(0);
    dic.entry("WA").or_insert(0);
    dic.entry("TLE").or_insert(0);
    dic.entry("RE").or_insert(0);
    for si in &s {
        *dic.entry(si.as_str()).or_insert(0) += 1;
    }
    for si in vec!["AC", "WA", "TLE", "RE"] {
        println!("{} x {}", si, dic.get(&si).unwrap());
    }
}
