fn main() {
    proconio::input! {s: [String; 2]}

    println!("{}",
        if (s[0] == "#." && s[1] == ".#")
        || (s[0] == ".#" && s[1] == "#.") {
            "No"
        } else {
            "Yes"
        }
    );
}
