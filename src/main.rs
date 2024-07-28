use std::fs;
use std::path::Path;
use macroquad::prelude::*;
use json::*;
use rand::{srand, gen_range};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::exit;

#[derive(Copy, Clone)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl PartialEq<Vec2> for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        }
        return false;
    }
}

#[macroquad::main("snak")]
async fn main() -> std::io::Result<()> {
    let mut snaks: Vec<Vec2> = vec![Vec2 {x: 1., y: 0.}, Vec2 {x: 2., y: 0.}];
    let mut avail: Vec<Vec2> = Vec::new();
    let mut dt: f32;
    let mut curr: char = 'd';
    let mut food: Vec2;
    
    for x in 0..50 {
        for y in 0..50 {
            for i in 0..snaks.len()-1 {
                if x as f32 != snaks[i].x && y as f32 != snaks[i].y {
                    avail.push(Vec2 {x: x as f32, y: y as f32});
                }
            }
        }
    }
    srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    food = avail[gen_range(0, avail.len()-1)];

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
        draw_rectangle(food.x * 16., food.y * 16., 16., 16., RED);

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
            for i in 0..avail.len()-1 {
                if avail[i] == snaks[snaks.len()-1] {
                    avail.remove(i);
                    break;
                }
            }
            avail.push(snaks[0]);
            snaks.reverse();
            for i in snaks.clone().into_iter().skip(1) {
                if i == snaks[0] {
                    exit(0);
                }
            }
            snaks.reverse();
            snaks.remove(0);
            if snaks[snaks.len()-1] == food {
                snaks.push(Vec2 {x: snaks[snaks.len()-1].x + mov.x, y: snaks[snaks.len()-1].y + mov.y});
                food = avail[gen_range(0, avail.len()-1)];
            }
            tim = 0.;
            changed = true;
        }

        dt = get_frame_time();
        tim += dt;
        next_frame().await;
    }
}