fn main() {
    proconio::input! {d: String}
    println!("{}", match &d as &str {
        "N" => "S",
        "E" => "W",
        "W" => "E",
        "S" => "N",
        "NE" => "SW",
        "NW" => "SE",
        "SE" => "NW",
        "SW" => "NE",
        _ => ""
    });
}
