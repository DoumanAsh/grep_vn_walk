use std::env;

pub const HELP: &'static str = "Looks up walkthrough for VN on http://seiya-saiga

USAGE:
    grep_vn_walk [pattern]

PATTERN:
    Title of VN to look for.
    If omitted, prints every walkthrough.

FLAGS:
    -h, --help - Prints this help message.
";

///Describes arguments
pub enum Args {
    ///List all walks
    Full,
    ///Find walks that match following title
    Grep(String),
    ///Prints help
    Help
}

pub fn args() -> Args {
    let mut args = Vec::new();
    for arg in env::args().skip(1) {
        if arg.starts_with("-") {
            match &arg[1..] {
                "h" | "-help" => return Args::Help,
                _ => continue
            }
        } else {
            args.push(arg)
        }
    }

    match args.len() {
        0 => Args::Full,
        _ => Args::Grep(args.join(" "))
    }
}
