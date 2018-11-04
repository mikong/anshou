extern crate termion;
extern crate anshou;

use termion::raw::IntoRawMode;
use termion::event;
use termion::cursor;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

use termion::{color, clear};

use anshou::Form;

fn main() {
    print!("{clear}{fg}{bg}",
        clear = clear::All,
        fg = color::Fg(color::LightWhite),
        bg = color::Bg(color::Blue));

    let mut stdout = stdout().into_raw_mode().unwrap();

    let form = Form::new();
    form.render(&mut stdout);

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
