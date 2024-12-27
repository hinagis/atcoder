fn main() {
    proconio::input! {n: String}
    let f = n.chars().filter(|&c| c == '1').count() == 1 &&
                  n.chars().filter(|&c| c == '2').count() == 2 &&
                  n.chars().filter(|&c| c == '3').count() == 3;
    println!("{}", if f { "Yes" } else { "No" });
}
