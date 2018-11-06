use std::io::Write;

trait Element {
    fn label(&self) -> &str;
    fn width(&self) -> u16;
}

#[derive(Debug)]
struct Field {
    name: String,
    value: String,
}

impl Field {
    fn new(name: String) -> Self {
        Field {
            value: String::new(),
            name,
        }
    }

    fn boxed(name: String) -> Box<Self> {
        Box::new(Self::new(name))
    }
}

impl Element for Field {
    fn label(&self) -> &str {
        &self.name
    }

    fn width(&self) -> u16 {
        35
    }
}

pub struct Form {
    elems: Vec<Box<Element>>,
    width: u16,
}

impl Form {
    pub fn new() -> Self {
        let password = Field::boxed("Password".to_string());
        let domain = Field::boxed("Domain".to_string());
        let elems: Vec<Box<Element>> = vec![password, domain];
        let width = Self::calculate_width(&elems);

        Form {
            elems,
            width,
        }
    }

    pub fn render<W: Write>(&self, out: &mut W) {
        out.write(self.render_top_border().as_bytes()).unwrap();
        out.write(self.render_empty_row().as_bytes()).unwrap();
        out.write(self.render_elements().as_bytes()).unwrap();
        out.write(self.render_empty_row().as_bytes()).unwrap();
        out.write(self.render_bottom_border().as_bytes()).unwrap();
    }

    fn render_top_border(&self) -> String {
        let mut s = "╔".to_string();
        for _ in 2..self.width {
            s.push('═');
        }
        s.push_str("╗\n\r");
        s
    }

    fn render_empty_row(&self) -> String {
        let mut s = "║".to_string();
        for _ in 2..self.width {
            s.push(' ');
        }
        s.push_str("║\n\r");
        s
    }

    fn render_elements(&self) -> &'static str {
        "║ Algorithm: SHA1                     ║\n\r\
         ║ Length: 10                          ║\n\r\
         ║ Password: ∙∙∙∙∙∙                    ║\n\r\
         ║ Domain: github.com                  ║\n\r"
    }

    fn render_bottom_border(&self) -> String {
        let mut s = "╚".to_string();
        for _ in 2..self.width {
            s.push('═');
        }
        s.push_str("╝\n\r");
        s
    }

    fn calculate_width(elems: &Vec<Box<Element>>) -> u16 {
        let border_width = 1;
        let margin_width = 1;

        let elem_width = elems.iter().map(|x| x.width()).max().unwrap_or(0);

        2*border_width + 2*margin_width + elem_width
    }
}
