use rand::rng;
use rand::seq::IndexedMutRandom;

pub fn run() {
    let mut pages: Vec<&str> = vec![
        "https://www.pi.fyi/",
        "https://neal.fun/",
        "https://bored.com/",
        "https://theuselessweb.com/",
        "https://www.pinterest.com/",
        "https://dev.to",
        "https://archive.org/",
        "https://substack.com/",
    ];

    let mut rng = rng();

    if let Some(url) = pages.choose_mut(&mut rng) {
        println!("Opening: {}", url);

        if let Err(err) = open::that(url) {
            eprintln!("Failed to open browser: {}", err);
        }
    }
}
