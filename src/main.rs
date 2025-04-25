use html_link_to_text::Link;
use std::process::exit;

fn main() {
    // 1. read text from command line!
    let args: Vec<String> = std::env::args().collect();

    // args must not be empty!
    if args.len() < 2 || args[1].is_empty() {
        eprintln!("You have to provide a link to the program!");
        exit(1);
    }
    // 2. strip down to url - for the moment assume we'll just paste the url to the tool, nothing
    //    more!
    let link = Link::new(&args[1]);
    // 3. download from link
    let newsletter = link.download().expect("DEBUG: error downloading from link");
    // 4. print result!
    println!("{}", newsletter);
}
