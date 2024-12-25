fn main() {
    proconio::input! {
        n: usize,
        s: [String; n]
    }

    println!("{}", calc(&s, n));
}

fn calc(s: &Vec<String>, i: usize) -> u64 {
    if i == 0 {
        1
    } else {
        calc(s, i - 1) + if s[i - 1] == "OR" {1 << i} else {0}
    }
}
