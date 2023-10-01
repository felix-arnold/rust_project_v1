mod map;

use comfy::*;

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

    let size_x = 10;
    let size_y = 10;
    let map = map::TileMap::new(size_x, size_y);
    for x in 0..size_x {
        for y in 0..size_y {        
            let coord = map.get_real_coord(x, y);
            
            let mut color = BLACK;
            color.b = 0.0;
            color.g = 0.0;
            color.r = 10.0;
 
            draw_sprite(
                texture_id("tile"),
                vec2(coord.0*1.9, coord.1*1.9),
                color,
                map.get_coord(x, y).0 as i32,
                splat(3.0),
            );
        }
    }
}