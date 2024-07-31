#[derive(Debug, Clone, Copy)]
pub enum OamSize {
    Size8x8,
    Size8x16,
}

impl From<bool> for OamSize {
    fn from(value: bool) -> Self {
        match value {
            true => OamSize::Size8x16,
            false => OamSize::Size8x8,
        }
    }
}

impl From<OamSize> for bool {
    fn from(value: OamSize) -> Self {
        match value {
            OamSize::Size8x16 => true,
            OamSize::Size8x8 => false,
        }
    }
}
