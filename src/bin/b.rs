fn main() {
    proconio::input! {
        l: [i32; 3],
        r: [i32; 3],
        u: [i32; 3],
        v: [i32; 3],
    }
    let a = (0..3).map(|i| l[i] <= u[i] && u[i] < r[i]).collect::<Vec<_>>();
    let b = (0..3).map(|i| u[i] <= l[i] && l[i] < v[i]).collect::<Vec<_>>();
    let f = a[0] && a[1] && (a[2] || (u[2] < l[2] && l[2] < v[2]));
    let g = a[1] && a[2] && (a[0] || (u[0] < l[0] && l[0] < v[0]));
    let h = a[2] && a[0] && (a[1] || (u[1] < l[1] && l[1] < v[1]));
    let i = b[0] && b[1] && (b[2] || (l[2] < u[2] && u[2] < r[2]));
    let j = b[1] && b[2] && (b[0] || (l[0] < u[0] && u[0] < r[0]));
    let k = b[2] && b[0] && (b[1] || (l[1] < u[1] && u[1] < r[1]));
    println!("{}", if f || g || h || i || j || k {"Yes"} else {"No"});
}
