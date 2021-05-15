fn main() {
    proconio::input! {
        n: usize,
        mut st: [(String, u32); n]
    }
    st.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{}", st[1].0);
}
