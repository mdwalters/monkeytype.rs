use ansi_term::{Colour, Style};
use std::io;
use std::io::Write;

fn main() {
    let mut val = String::new();

    print!(
        "{}",
        Style::new()
            .fg(Colour::Fixed(240))
            .paint("never gonna give you up")
    );
    print!("\r");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut val).expect("words here");
}
