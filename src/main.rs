pub mod map;

use comfy::*;

simple_game!("Text Example", update);

fn update(_c: &mut EngineContext) {

    draw_poly_z(vec2(0.0, 0.0), 6, 5.0, 0.0, BLUE, 0, TextureParams { blend_mode: BlendMode::None, shader: None });

}