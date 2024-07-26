use macroquad::prelude::*;
use std::env;

struct Vec2 {
    x: f32,
    y: f32,
}

#[macroquad::main("snak")]
async fn main() {
    let mut snaks: Vec<Vec2> = vec![Vec2 {x: 1.0, y: 0.0}];
    let mut dt: f32 = 0.0;
    
    println!("{}", env::consts::OS);
    loop {
        clear_background(BLACK);
        if is_key_down(KeyCode::W) {
            //println!("{} {}, {}", snak[0].x, snak[0].y, dt);
            //snak[0].x+=1;
        }
        for snak in snaks.iter() {
            draw_rectangle(snak.x * 16.0, snak.y * 16.0, 16.0, 16.0, WHITE);
        }

        dt = get_frame_time();
        next_frame().await;
    }
}