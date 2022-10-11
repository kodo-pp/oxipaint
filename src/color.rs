#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FloatRgbColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl FloatRgbColor {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}
