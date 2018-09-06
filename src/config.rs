pub struct Config {
    pub master_password: String,
    pub domain: String,
    pub length: usize,
}

impl Config {
    pub fn new(password: &str, domain: &str) -> Self {
        let master_password = password.to_string();
        let domain = domain.to_string();
        let length = 10;

        Config {
            master_password,
            domain,
            length,
        }
    }
}
