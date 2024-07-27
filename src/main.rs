use std::fs;
use std::path::Path;
use macroquad::prelude::*;
use json::*;

struct Vec2 {
    x: f32,
    y: f32,
}

#[macroquad::main("snak")]
async fn main() -> std::io::Result<()> {
    let mut snaks: Vec<Vec2> = vec![Vec2 {x: 1., y: 0.}];
    let mut dt: f32;
    let mut curr: char = 'd';
    snaks.push(Vec2 {x: 2., y: 0.});
    
    if !Path::new(".config").exists() {
        assert!(!fs::create_dir(".config").is_err(), "Cannot access filesystem");
    }
    if !Path::new(".config/conf.json").exists() {
        assert!(!fs::write(".config/conf.json", "{\"tps\": 5}").is_err(), "Cannot access filesystem");
    }
    let cont = fs::read_to_string(".config/conf.json")?;
    let parsed = parse(&cont).unwrap();
    println!("{}", parsed["tps"]);
    let tps = parsed["tps"].as_f32().unwrap();
    let ticktime = 1. / tps;
    let mut tim: f32 = 0.;
    let mut changed: bool = true;
    let mut mov: Vec2 = Vec2 {x: 1., y: 0.};
    request_new_screen_size(800., 800.);
    loop {
        clear_background(BLACK);
        if is_key_down(KeyCode::W) && curr != 's' && changed {
            curr = 'w';
            changed = false;
        } else if is_key_down(KeyCode::S) && curr != 'w' && changed {
            curr = 's';
            changed = false;
        } else if is_key_down(KeyCode::A) && curr != 'd' && changed {
            curr = 'a';
            changed = false;
        } else if is_key_down(KeyCode::D) && curr != 'a' && changed {
            curr = 'd';
            changed = false;
        }
        for snak in snaks.iter() {
            draw_rectangle(snak.x * 16.0, snak.y * 16.0, 16.0, 16.0, WHITE);
        }

        if tim >= ticktime {
            mov.x = match curr {
                'w' | 's' => 0.,
                'a' => -1.,
                'd' => 1.,
                _ => panic!(),
            };
            mov.y = match curr {
                'a' | 'd' => 0.,
                'w' => -1.,
                's' => 1.,
                _ => panic!(),
            };
            snaks.push(Vec2 {x: snaks[snaks.len()-1].x + mov.x, y: snaks[snaks.len()-1].y + mov.y});
            snaks.remove(0);
            tim = 0.;
            changed = true;
        }

        dt = get_frame_time();
        tim += dt;
        next_frame().await;
    }
}