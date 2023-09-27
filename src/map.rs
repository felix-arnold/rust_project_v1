#[derive(Clone)]
pub struct HexagonTile {
    q: i16,
    r: i16,
    x: f32,
    y: f32,
}

impl HexagonTile {
    pub fn new(x_arr: i16, y_arr: i16, shift: f32) -> Self {
        let q = x_arr;
        let r = y_arr - x_arr/2;
        let x = 3.0 * (x_arr as f32 + 0.5 * (y_arr%2) as f32) - shift;
        let y = f32::sqrt(3.0) * y_arr as f32/2.0 - shift;
        Self { q, r, x, y }
    }
    
    pub fn new_empty() -> Self {
        Self { q: 0, r: 0, x: 0.0, y: 0.0 }
    }

    pub fn set_coordinates(&mut self, x_arr: i16, y_arr: i16, x_shift: f32, y_shift: f32) {
        self.q = x_arr;
        self.r = y_arr - x_arr/2;
        self.x = 0.9* 3.0 * (x_arr as f32 + 0.5 * (y_arr%2) as f32) - x_shift;
        self.y = 0.9 * f32::sqrt(3.0) * y_arr as f32/2.0 - y_shift;
    }
}

pub struct TileMap {
    map_array: Vec<Vec<HexagonTile>>,
    size_x: i16,
    size_y: i16,
}

impl TileMap {
    pub fn new(size_x: i16, size_y: i16) -> Self {
        let mut map_array = vec![vec![HexagonTile::new_empty(); size_y as usize]; size_x as usize];
        for x in 0..size_x {
            for y in 0..size_y {
                map_array[x as usize][y as usize].set_coordinates(x, y, size_x as f32, size_y as f32 /2.0);
            }
        }
        Self { map_array, size_x, size_y }
    }

    pub fn get_real_coord(&self, x: i16, y: i16) -> (f32, f32) {
        (self.map_array[x as usize][y as usize].x, self.map_array[x as usize][y as usize].y)
    }
}