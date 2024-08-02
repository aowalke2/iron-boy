use super::BACKGROUND_WIDTH;

const TILES_PER_ROW: u16 = BACKGROUND_WIDTH as u16 / 8;
pub const TILE_WIDTH: u8 = 8;
pub const TILE_HEIGHT: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileData {
    UnsignedAddressMode,
    SignedAddressMode,
}

impl TileData {
    pub fn address(self, tile_index: u8) -> u16 {
        match self {
            TileData::UnsignedAddressMode => 0x8000 + tile_index as u16 * 16,
            TileData::SignedAddressMode => 0x8000 + (0x1000 + ((tile_index as i8) as i16) * 16) as u16,
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

pub trait TileAccess {
    fn tile_map_coordinates(&self, lx: u8, ly: u8) -> (u8, u8);
    fn tile_pixel_coordinates(&self, x: u8, y: u8) -> (u8, u8);
    fn tile_index_address(&self, base_address: u16, tile_map_x: u8, tile_map_y: u8) -> u16 {
        let tile_x = (tile_map_x / TILE_WIDTH) as u16;
        let tile_y = (tile_map_y / TILE_HEIGHT) as u16;
        let offset = tile_y * TILES_PER_ROW + tile_x;
        base_address + offset
    }
}

#[cfg(test)]
mod tests {
    use super::TileData;

    #[test]
    fn tile_addressing_unsigned() {
        let addressing = TileData::UnsignedAddressMode;
        let expected_indices: Vec<u16> = (0..=255).map(|x| 0x8000 + x * 16).collect();

        let tile_indices: Vec<u16> = (0..=255).map(|x| addressing.address(x)).collect();
        assert_eq!(tile_indices, expected_indices)
    }

    #[test]
    fn tile_addressing_signed() {
        let addressing = TileData::SignedAddressMode;
        let expected_indices: Vec<u16> = (-128..=127).map(|x| 0x8000 + (0x1000 + (x as i16 * 16)) as u16).collect();

        let mut tile_indices: Vec<u16> = (128..=255).map(|x| addressing.address(x)).collect();
        tile_indices.extend((0..=127).map(|x| addressing.address(x)).collect::<Vec<u16>>());
        assert_eq!(tile_indices, expected_indices)
    }
}
