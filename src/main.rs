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
    camera.center = vec2(50.0, 25.0);
    c.lighting.ambient_light_intensity = 2.0;
}

fn update(_c: &mut EngineContext) {

    let size_x = 100;
    let size_y = 100;
    let tile_size = 1.0;
    let map = map::TileMap::new(size_x, size_y, tile_size);
    
    let perlin = Perlin::new(69);
    let frequency = 0.1;

    for x in 0..size_x {
        for y in 0..size_y {        
            let coord = map.get_real_coord(x, y);
            let val = perlin.get([coord.0 as f64 * frequency + 0.05, coord.1 as f64 * frequency + 0.05]);
            let color = comfy::Color::new(val as f32 * 0.4 + 0.5, val as f32 * 0.4 + 0.5, val as f32 * 0.4 + 0.5, 1.0);
            draw_sprite(
                texture_id("tile"),
                vec2(coord.0, coord.1),
                color,
                map.get_z(x, y),
                splat(tile_size),
            );
        }
    }
}