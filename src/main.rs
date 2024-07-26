use std::fs;
use std::path::Path;
use macroquad::prelude::*;
use json::*;
use std::fs::File;
use std::io;

struct Vec2 {
    x: f32,
    y: f32,
}

#[macroquad::main("snak")]
async fn main() -> std::io::Result<()> {
    let mut snaks: Vec<Vec2> = vec![Vec2 {x: 1., y: 0.}];
    let mut dt: f32 = 0.0;
    snaks.push(Vec2 {x: 2., y: 0.});
    
    if !Path::new(".config").exists() {
        assert!(!fs::create_dir(".config").is_err(), "Cannot access filesystem");
    }
    if !Path::new(".config/conf.json").exists() {
        assert!(!fs::write(".config/conf.json", "{\"fps\": 20}").is_err(), "Cannot access filesystem");
    }
    let cont = fs::read_to_string(".config/conf.json")?;
    let parsed = parse(&cont).unwrap();
    println!("{}", parsed["fps"]);
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