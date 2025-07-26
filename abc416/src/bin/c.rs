fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        x: usize,
        s: [String; n]
    }
    let mut p = Vec::with_capacity(n.pow(k as u32));
    fn calc(p: &mut Vec<String>, s: &Vec<String>, t: String, n: usize, k: usize) {
        if k == 0 {
            p.push(t);
            return;
        }
        for i in 0..n {
            calc(p, s, format!("{}{}", t, s[i]), n, k - 1);
        }
    }
    for i in 0..n {
        calc(&mut p, &s, s[i].clone(), n, k - 1);
    }
    p.sort();
    println!("{}", p[x - 1]);
}
