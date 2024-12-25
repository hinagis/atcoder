fn main() {
    proconio::input! {
        s: i32,
        t: i32,
        x: i32,
    }

    if s < t {
        if x >= s && x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if x >= s || x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
