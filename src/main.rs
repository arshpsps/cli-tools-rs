fn main() {
    let mut args: std::env::Args = std::env::args();

    let action: String = if let Some(x) = args.nth(1) {
        match x.as_str() {
            "uwu" => String::from("uwu"),
            _ => String::from("uwu"),
        }
    } else {
        String::from("sad")
    };

    println!("{}", action);
}
