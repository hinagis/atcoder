fn main() {
    proconio::input! {
        x: String,
        y: String
    }
    println!("{}", match y.as_str() {
        "Ocelot" => "Yes",
        "Serval" => if x == "Ocelot" {"No"} else {"Yes"},
        _ => if x == "Lynx" {"Yes"} else {"No"}
    });
}
