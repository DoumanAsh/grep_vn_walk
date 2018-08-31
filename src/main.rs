#[macro_use(set_panic_message)]
extern crate lazy_panic;
extern crate regex;
extern crate yukikaze;

mod cli;

use regex::Regex;
use yukikaze::futures::Future;
use yukikaze::rt::{AutoClient, AutoRuntime};
use yukikaze::client::Request;
use yukikaze::encoding::types::{Encoding, DecoderTrap};
use yukikaze::encoding::codec::japanese::Windows31JEncoding as ShiftJS;

const WALK_ROOT: &'static str = "http://seiya-saiga.com/game/";

fn print_walks(html: String) {
    let regex = Regex::new("<td align=\"left\"><B><A href=\"(?P<url>[^\"]+)\">(?P<title>[^<]+)</A></B></td>").expect("To create regex");
    for caps in regex.captures_iter(&html) {
        println!("{} - {}{}", &caps["title"], WALK_ROOT, &caps["url"]);
    }
}

fn get_walk<F: FnOnce(String) -> ()>(printer: F) {
    const URL: &'static str = "http://seiya-saiga.com/game/kouryaku.html";

    let _ = Request::get(URL).expect("To create request")
                             .empty()
                             .send()
                             .map_err(|error| eprintln!("Error: {:?}", error))
                             .and_then(|rsp| rsp.body().limit(u64::max_value()).map_err(|error| eprintln!("Error reading body: {:?}", error)))
                             .and_then(|body| ShiftJS.decode(&body, DecoderTrap::Strict).map_err(|error| eprintln!("Failed to decode ShiftJS: {}", error)))
                             .map(printer)
                             .finish();
}

fn main() {
    set_panic_message!(lazy_panic::formatter::Simple);

    let _guard = yukikaze::rt::init();
    yukikaze::rt::set_default();

    match cli::args() {
        cli::Args::Full => get_walk(print_walks),
        cli::Args::Grep(title) => get_walk(move |html| {
            let regex = Regex::new("<td align=\"left\"><B><A href=\"(?P<url>[^\"]+)\">(?P<title>[^<]+)</A></B></td>").expect("To create regex");
            for caps in regex.captures_iter(&html).filter(|caps| caps["title"].to_lowercase().contains(&title)) {
                println!("{} - {}{}", &caps["title"], WALK_ROOT, &caps["url"]);
            }
        }),
        cli::Args::Help => println!("{}", cli::HELP)
    }
}
