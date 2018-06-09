#[macro_use(set_panic_message)]
extern crate lazy_panic;
extern crate encoding;
extern crate regex;

mod cli;
mod actix;

use self::regex::Regex;
use self::actix::{Future, HttpMessage};
use self::encoding::types::{Encoding, DecoderTrap};
use self::encoding::codec::japanese::Windows31JEncoding as ShiftJS;

const WALK_ROOT: &'static str = "http://seiya-saiga.com/game/";

fn print_walks(html: String) {
    let regex = Regex::new("<td align=\"left\"><B><A href=\"(?P<url>[^\"]+)\">(?P<title>[^<]+)</A></B></td>").expect("To create regex");
    for caps in regex.captures_iter(&html) {
        println!("{} - {}{}", &caps["title"], WALK_ROOT, &caps["url"]);
    }
}

fn get_walk<F: FnOnce(String) -> ()>(printer: F) {
    const URL: &'static str = "http://seiya-saiga.com/game/kouryaku.html";

    let mut system = actix::System::new("grep_vn_walk");

    let req = actix::get(URL).finish().expect("To create empty HTTP request")
                             .send()
                             .map_err(|error| eprintln!("Error: {}", error))
                             .and_then(|rsp| rsp.body().limit(usize::max_value()).map_err(|error| eprintln!("Error reading body: {}", error)))
                             .and_then(|body| ShiftJS.decode(&body, DecoderTrap::Strict).map_err(|error| eprintln!("Failed to decode ShiftJS: {}", error)))
                             .map(printer);

    let _ = system.run_until_complete(req);

}

fn main() {
    set_panic_message!(lazy_panic::formatter::Simple);

    match cli::args() {
        cli::Args::Full => get_walk(print_walks),
        cli::Args::Grep(title) => get_walk(move |html| {
            let regex = Regex::new("<td align=\"left\"><B><A href=\"(?P<url>[^\"]+)\">(?P<title>[^<]+)</A></B></td>").expect("To create regex");
            for caps in regex.captures_iter(&html).filter(|caps| caps["title"].contains(&title)) {
                println!("{} - {}{}", &caps["title"], WALK_ROOT, &caps["url"]);
            }
        }),
        cli::Args::Help => println!("{}", cli::HELP)
    }
}
