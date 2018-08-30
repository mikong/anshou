extern crate termion;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

use termion::{color, clear};

const FORM: &'static str = "╔══════════════════════════════════╗\n\r\
                            ║                                  ║\n\r\
                            ║ Algorithm: SHA1                  ║\n\r\
                            ║ Length: 10                       ║\n\r\
                            ║ Password: ∙∙∙∙∙∙                 ║\n\r\
                            ║ Domain: github.com               ║\n\r\
                            ║                                  ║\n\r\
                            ╚══════════════════════════════════╝\n\r";

fn main() {
    print!("{clear}{fg}{bg}",
        clear = clear::All,
        fg = color::Fg(color::LightWhite),
        bg = color::Bg(color::Blue));

    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.write(FORM.as_bytes()).unwrap();

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => continue,
        }
    }

    println!("{fg}{bg}",
        fg = color::Fg(color::Reset),
        bg = color::Bg(color::Reset));
}
