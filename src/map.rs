#[derive(Clone)]
pub struct HexagonTile {
    q: i16,
    r: i16,
    x: f32,
    y: f32,
}

impl HexagonTile {
    
    pub fn new_empty() -> Self {
        Self { q: 0, r: 0, x: 0.0, y: 0.0 }
    }

    pub fn set_coordinates(&mut self, x_arr: i16, y_arr: i16, size: f32) {
        self.q = y_arr;
        self.r = x_arr - y_arr/2;
        self.x = size * (x_arr as f32 + 0.5 * (y_arr%2) as f32);
        self.y = size * y_arr as f32 / 2.0;
    }
}

pub struct TileMap {
    map_array: Vec<Vec<HexagonTile>>,
    size_x: i16,
    size_y: i16,
}

impl TileMap {
    pub fn new(size_x: i16, size_y: i16, tile_size: f32) -> Self {
        let mut map_array = vec![vec![HexagonTile::new_empty(); size_y as usize]; size_x as usize];
        for x in 0..size_x {
            for y in 0..size_y {
                map_array[x as usize][y as usize].set_coordinates(x, y, tile_size);
            }
        }
        Self { map_array, size_x, size_y }
    }

    pub fn get_real_coord(&self, x: i16, y: i16) -> (f32, f32) {
        (self.map_array[x as usize][y as usize].x, self.map_array[x as usize][y as usize].y)
    }

    pub fn get_coord(&self, x: i16, y: i16) -> (i16, i16) {
        (self.map_array[x as usize][y as usize].q, self.map_array[x as usize][y as usize].r)
    }
}