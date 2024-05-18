#[derive(PartialEq, Copy, Clone)]
pub enum Mode {
    Monochrome,
    Color,
    ColorAsMonochrome,
}

pub enum Speed {
    Normal,
    Double,
}
