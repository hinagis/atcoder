fn main() {
    proconio::input! {x: String}
    let x = x.trim_end_matches('0').to_string();
    let x = x.trim_end_matches('.').to_string();
    println!("{}", x);
}
