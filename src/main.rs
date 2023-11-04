use std::time::Instant;

use particle_life::{Particle, World};
use sdl2::{event::Event, pixels::Color, rect::Rect};
use ultraviolet as uv;

const GRAVITY: f32 = 6.67;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("particles", 1000, 1000)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut world = World::new(
        uv::Vec2::new(1000.0, 1000.0),
        1000,
        0.8,
        50.0,
        vec!["red", "green", "blue"],
    );

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for p in world.particles.iter() {
            match p.r#type {
                "red" => canvas.set_draw_color(Color::RGB(255, 0, 0)),
                "green" => canvas.set_draw_color(Color::RGB(0, 255, 0)),
                "blue" => canvas.set_draw_color(Color::RGB(0, 0, 255)),
                _ => panic!("unknown type {}", p.r#type),
            }
            let _ = canvas.fill_rect(Rect::new(p.position.x as i32, p.position.y as i32, 3, 3));
        }

        world.calculate(&compute);

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
}

fn compute(p1: &Particle, p2: &Particle) -> uv::Vec2 {
    let mut v = p2.position - p1.position;
    let mag = v.mag();
    if mag > 100.0 {
        return uv::Vec2::zero();
    }
    v = v.normalized() * GRAVITY / (mag * mag);
    match (p1.r#type, p2.r#type) {
        ("red", "red") => {
            v *= 3.0;
        }
        ("green", "red") => {}
        ("red", "green") => {
            v *= -1.0;
        }
        ("blue", "red") => v *= -1.0,
        ("blue", "blue") => v *= 2.0,
        _ => {}
    }

    return v;
}
