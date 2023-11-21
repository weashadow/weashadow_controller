pub struct Printer {
  is_printing: bool,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            is_printing: false,
        }
    }

    pub fn print(&self) {
    }
}