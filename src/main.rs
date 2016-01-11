use std::env;
use std::io::{self, Write};

fn main() {
    let mut args = env::args();
    let _ = args.next(); // Throw away program name.
    let mut word_owned = args.next();
    let word = match word_owned {
        Some(ref mut x) => {
            x.push('\n');
            &x[..]
        },
        None => "y\n",
    };
    loop {
        io::stdout().write(word.as_bytes()).expect("Failed to write to stdout.");
    }
}
