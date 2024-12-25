fn main() {
    proconio::input! {a: [i32; 3]}

    println!("{}",
        if a[2] - a[1] == a[1] - a[0] 
        || a[2] - a[0] == a[0] - a[1]
        || a[1] - a[0] == a[0] - a[2]
        || a[1] - a[2] == a[2] - a[0]
        || a[0] - a[2] == a[2] - a[1]
        || a[0] - a[1] == a[1] - a[2]
        {
            "Yes"
        } else {
            "No"
        }
    );
}
