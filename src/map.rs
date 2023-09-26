#[derive(Clone)]
struct HexagonTile {
    q: u16,
    r: u16,
}

impl HexagonTile {
    pub fn new(q: u16, r: u16) -> Self {
        Self { q, r }
    }
}

struct TileMap {
    map_array: Vec<Vec<HexagonTile>>,
    size_x: u16,
    size_y: u16,
}

impl TileMap {
    pub fn new(size_x: u16, size_y: u16) -> Self {
        let map_array = vec![vec![HexagonTile::new(0, 0); size_y as usize]; size_x as usize];
        Self { map_array, size_x, size_y }
    }
}