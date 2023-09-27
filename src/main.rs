mod map;

use comfy::*;

simple_game!("Text Example", update);

fn update(_c: &mut EngineContext) {

    let size_x = 8;
    let size_y = 16;
    let map = map::TileMap::new(size_x, size_y);
    for x in 0..size_x {
        for y in 0..size_y {        
            let coord = map.get_real_coord(x, y);
            draw_poly_z(vec2(coord.0, coord.1), 6, 0.8, 0.0, BLUE, 0, TextureParams { blend_mode: BlendMode::None, shader: None });
        }
    }
}