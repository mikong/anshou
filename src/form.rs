use std::io::Write;

trait Element {
    fn label(&self) -> &str;
}

struct Password {
    value: String,
}

impl Password {
    fn new() -> Self {
        Password { value: String::new() }
    }

    fn boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Element for Password {
    fn label(&self) -> &'static str {
        &"Password"
    }
}

struct Domain {
    value: String,
}

impl Domain {
    fn new() -> Self {
        Domain { value: String::new() }
    }

    fn boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Element for Domain {
    fn label(&self) -> &'static str {
        &"Domain"
    }
}

pub struct Form {
    elems: Vec<Box<Element>>,
}

impl Form {
    pub fn new() -> Self {
        Form {
            elems: vec![Password::boxed(), Domain::boxed()]
        }
    }

    pub fn render<W: Write>(&self, out: &mut W) {
        out.write(self.render_top_border().as_bytes()).unwrap();
        out.write(self.render_empty_row().as_bytes()).unwrap();
        out.write(self.render_elements().as_bytes()).unwrap();
        out.write(self.render_empty_row().as_bytes()).unwrap();
        out.write(self.render_bottom_border().as_bytes()).unwrap();
    }

    fn render_top_border(&self) -> &'static str {
        "╔══════════════════════════════════╗\n\r"
    }

    fn render_empty_row(&self) -> &'static str {
        "║                                  ║\n\r"
    }

    fn render_elements(&self) -> &'static str {
        "║ Algorithm: SHA1                  ║\n\r\
         ║ Length: 10                       ║\n\r\
         ║ Password: ∙∙∙∙∙∙                 ║\n\r\
         ║ Domain: github.com               ║\n\r"
    }

    fn render_bottom_border(&self) -> &'static str {
        "╚══════════════════════════════════╝\n\r"
    }
}
