fn main() {
    proconio::input! {n: u32}
    for i in n.. {
        if (i / 100) * ((i / 10) % 10) == i % 10 {
            println!("{i}");
            break
        }
    }
}
