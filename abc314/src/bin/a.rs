fn main() {
    const PI: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
    proconio::input! {n: usize}
    println!("{}", PI.chars().take(n + 2).collect::<String>());
}
