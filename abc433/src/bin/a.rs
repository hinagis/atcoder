use proconio::input as I;

fn main() {
    I! {x: i32, y: i32, z: i32}
    let mut c = 0;
    while (x + c) >= (y + c) * z {
        if (x + c) == (y + c) * z {
            println!("Yes");
            return;
        }
        c += 1;
    }
    println!("No");
}
