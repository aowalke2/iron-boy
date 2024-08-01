#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectSize {
    Size8x8,
    Size8x16,
}

impl From<bool> for ObjectSize {
    fn from(value: bool) -> Self {
        match value {
            true => ObjectSize::Size8x16,
            false => ObjectSize::Size8x8,
        }
    }
}

impl From<ObjectSize> for bool {
    fn from(value: ObjectSize) -> Self {
        match value {
            ObjectSize::Size8x16 => true,
            ObjectSize::Size8x8 => false,
        }
    }
}
