pub struct Printer;

impl Printer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn success(&self, message: &str) {
        println!("✓ {message}");
    }

    pub fn info(&self, message: &str) {
        println!("{message}");
    }

    pub fn error(&self, message: &str) {
        eprintln!("error: {message}");
    }

    pub fn plain(&self, message: &str) {
        println!("{message}");
    }
}
