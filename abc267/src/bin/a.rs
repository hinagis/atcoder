fn main() {
    proconio::input! {s: String}
    let w = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    for i in 0..5 {
        if s == w[i] {
            println!("{}", 5 - i);
            break;
        }
    }
}
