fn main() {
    proconio::input! {x: [u32; 7]}
    let c = x[6] / (x[0] + x[2]);
    let t = c * x[0] * x[1] + (x[6] - c * (x[0] + x[2])).min(x[0]) * x[1];
    let c = x[6] / (x[3] + x[5]);
    let a = c * x[3] * x[4] + (x[6] - c * (x[3] + x[5])).min(x[3]) * x[4];
    println!("{}", if t > a {"Takahashi"} else if t < a {"Aoki"} else {"Draw"});
}
