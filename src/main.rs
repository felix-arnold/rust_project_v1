mod map;

use comfy::*;
use noise::{NoiseFn, Perlin};

simple_game!("Rust Project V1", setup, update);

fn setup(c: &mut EngineContext) {
    c.load_texture_from_bytes(
        "tile",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/tile.png"
        )),
    );
    let mut camera = main_camera_mut();
    camera.zoom = 100.0;
    c.lighting.ambient_light_intensity = 2.0;
}

fn update(_c: &mut EngineContext) {

    let size_x = 20;
    let size_y = 20;
    let tile_size = 4.0;
    let map = map::TileMap::new(size_x, size_y, tile_size);
    
    let perlin = Perlin::new(3);
    let frequency = 10.0;

    for x in 0..size_x {
        for y in 0..size_y {        
            let coord = map.get_real_coord(x, y);
            let val = perlin.get([coord.0 as f64 * frequency + 0.5, coord.1 as f64 * frequency + 0.5]);
            let color = comfy::Color::new(val as f32 * 0.5 + 0.55, val as f32 * 0.5 + 0.55, val as f32 * 0.5 + 0.55, 1.0);
            draw_sprite(
                texture_id("tile"),
                vec2(coord.0 - 40.0, coord.1 - 20.0),
                color,
                size_y as i32 - map.get_real_coord(x, y).1 as i32,
                splat(tile_size),
            );
        }
    }
}