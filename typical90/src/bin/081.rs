use proconio::input as I;

fn main(){
    I!{
        n: i32,
        k: usize
    }

    let mut c=vec![vec![0; 5001]; 5001];
    for _ in 0..n{
        I!{
            a: usize,
            b: usize
        }
        c[a][b] += 1;
    }

    for i in 0..5000 {
        for j in 0..5000 {
            c[i + 1][j + 1] += c[i][j + 1] + c[i + 1][j] - c[i][j];
        }
    }

    let mut m = 0;
    let l = 5000.min(k + 1);
    for i in l..5001 {
        for j in l..5001 {
            m = m.max(c[i][j] - c[i - l][j] - c[i][j - l] + c[i - l][j - l]);
        }
    }
    println!("{m}");
}
