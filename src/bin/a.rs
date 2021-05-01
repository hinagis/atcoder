fn main() {
    proconio::input! {s: String}
    println!("{}", s.matches("ZONe").collect::<Vec<&str>>().len());
}
