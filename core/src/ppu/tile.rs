#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileData {
    UnsignedAddressMode,
    SignedAddressMode,
}

impl TileData {
    pub fn address(self, tile_index: u8) -> u16 {
        match self {
            TileData::UnsignedAddressMode => 0x8000 + tile_index as u16 * 16,
            TileData::SignedAddressMode => {
                0x8000 + (0x1000 + ((tile_index as i8) as i16) * 16) as u16
            }
        }
    }
}

impl From<bool> for TileData {
    fn from(value: bool) -> Self {
        match value {
            true => TileData::UnsignedAddressMode,
            false => TileData::SignedAddressMode,
        }
    }
}

impl From<TileData> for bool {
    fn from(tile_data: TileData) -> Self {
        match tile_data {
            TileData::UnsignedAddressMode => true,
            TileData::SignedAddressMode => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileMap {
    Low,
    High,
}

impl TileMap {
    pub fn base_address(self) -> u16 {
        match self {
            TileMap::Low => 0x9800,
            TileMap::High => 0x9C00,
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
    fn from(tile_map: TileMap) -> Self {
        match tile_map {
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
        let addressing = TileData::UnsignedAddressMode;
        let expected_indices: Vec<u16> = (0..=255).map(|x| x * 16).collect();

        let tile_indices: Vec<u16> = (0..=255).map(|x| addressing.address(x)).collect();
        assert_eq!(tile_indices, expected_indices)
    }

    #[test]
    fn tile_addressing_signed() {
        let addressing = TileData::SignedAddressMode;
        let expected_indices: Vec<u16> = (-128..=127)
            .map(|x| (0x1000 + (x as i16 * 16)) as u16)
            .collect();

        let mut tile_indices: Vec<u16> = (128..=255).map(|x| addressing.address(x)).collect();
        tile_indices.extend(
            (0..=127)
                .map(|x| addressing.address(x))
                .collect::<Vec<u16>>(),
        );
        assert_eq!(tile_indices, expected_indices)
    }
}
