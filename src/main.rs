extern crate termion;

use termion::raw::IntoRawMode;
use termion::event;
use termion::cursor;
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

    write!(stdout, "Password: ").unwrap();
    stdout.flush().unwrap();

    let mut s = String::new();

    for c in stdin().keys() {
        match c.unwrap() {
            event::Key::Char('\n') => {
                println!("\n\r{}\n\r", s);
                break;
            },
            event::Key::Char(c) => {
                write!(stdout, "∙").unwrap();
                stdout.flush().unwrap();
                s.push(c);
            },
            event::Key::Backspace => {
                write!(stdout, "{} {}", cursor::Left(1), cursor::Left(1)).unwrap();
                stdout.flush().unwrap();
                s.pop();
            },
            _ => {},
        }
    }

    print!("{fg}{bg}{clear}",
        fg = color::Fg(color::Reset),
        bg = color::Bg(color::Reset),
        clear = clear::CurrentLine,
    );
}
