use proconio::input as I;

fn main() {
    I! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n]
    }
    let mut d = [i64::MIN; 4];
    d[0] = 0;
	for i in 0..n {
		d = [0, d[0].max(d[1]) + a[i], d[1].max(d[2]) + b[i], d[2].max(d[3]) + c[i]];
	}
    println!("{}", d[3]);
}
