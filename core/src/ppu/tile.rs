#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileData {
    UnsignedAddress,
    SignedAddress,
}

impl TileData {
    pub fn tile_start_address(self, tile_number: u8) -> u16 {
        match self {
            TileData::UnsignedAddress => tile_number as u16 * 16,
            TileData::SignedAddress => (0x1000 + (((tile_number as i8) as i16) * 16)) as u16,
        }
    }
}

impl From<bool> for TileData {
    fn from(value: bool) -> Self {
        match value {
            true => TileData::UnsignedAddress,
            false => TileData::SignedAddress,
        }
    }
}

impl From<TileData> for bool {
    fn from(value: TileData) -> Self {
        match value {
            TileData::UnsignedAddress => true,
            TileData::SignedAddress => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TileMap {
    Low,
    High,
}

impl TileMap {
    pub fn base_address(self) -> u16 {
        match self {
            TileMap::Low => 0x1800,
            TileMap::High => 0x1C00,
        }
    }
}

impl From<bool> for TileMap {
    fn from(value: bool) -> Self {
        match value {
            true => TileMap::High,
            false => TileMap::Low,
        }
    }
}

impl From<TileMap> for bool {
    fn from(value: TileMap) -> Self {
        match value {
            TileMap::High => true,
            TileMap::Low => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TileData;

    #[test]
    fn tile_addressing_unsigned() {
        let addressing = TileData::UnsignedAddress;
        let expected_indices: Vec<u16> = (0..=255).map(|x| x * 16).collect();

        let tile_indices: Vec<u16> = (0..=255).map(|x| addressing.tile_start_address(x)).collect();
        assert_eq!(tile_indices, expected_indices)
    }

    #[test]
    fn tile_addressing_signed() {
        let addressing = TileData::SignedAddress;
        let expected_indices: Vec<u16> = (-128..=127).map(|x| (0x1000 + (x as i16 * 16)) as u16).collect();

        let mut tile_indices: Vec<u16> = (128..=255).map(|x| addressing.tile_start_address(x)).collect();
        tile_indices.extend((0..=127).map(|x| addressing.tile_start_address(x)).collect::<Vec<u16>>());
        assert_eq!(tile_indices, expected_indices)
    }
}
