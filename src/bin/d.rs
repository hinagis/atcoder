fn main() {
    proconio::input! {
        n: usize,
        s: [String; n]
    }

    println!("{}", calc(&s, n));
}

fn calc(s: &Vec<String>, i: usize) -> u32 {
    if i == 0 {
        1
    } else {
        calc(s, i - 1) + if s[i - 1] == "OR" {1 << (i as u32)} else {0}
    }
}
