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

fn read_file(path: String) -> String {
    match std::fs::read_to_string(path) {
        Ok(val) => val,
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                String::from("")
            } else {
                String::from("") //TODO: panic here
            }
        }
    }
}
