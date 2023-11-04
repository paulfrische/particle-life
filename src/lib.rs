#![allow(unused)]

use rayon::prelude::*;
use rand::prelude::*;
use ultraviolet as uv;

pub struct Particle<'a> {
    pub position: uv::Vec2,
    pub velocity: uv::Vec2,
    pub r#type: &'a str,
}

pub struct World<'a> {
    pub particles: Vec<Particle<'a>>,
    pub size: uv::Vec2,
    pub drag_coefficient: f32,
    pub max_velocity: f32,
}

impl Particle<'_> {
    pub fn random<'a>(r#type: &'a str, word_size: uv::Vec2) -> Particle<'a> {
        Particle::<'a> {
            position: uv::Vec2::new(
                rand::thread_rng().gen_range(0.0..word_size.x),
                rand::thread_rng().gen_range(0.0..word_size.x),
            ),
            velocity: uv::Vec2::zero(),
            r#type,
        }
    }
}

impl World<'_> {
    pub fn new<'a>(
        size: uv::Vec2,
        count: usize,
        drag_coefficient: f32,
        max_velocity: f32,
        r#types: Vec<&'a str>,
    ) -> World<'a> {
        let mut particles = Vec::new();
        for _ in 0..count {
            particles.push(Particle::random(
                types.choose(&mut rand::thread_rng()).unwrap(),
                size,
            ))
        }

        World {
            size,
            particles,
            drag_coefficient,
            max_velocity
        }
    }

    pub fn calculate(&mut self, f: &dyn Fn(&Particle, &Particle) -> uv::Vec2) {
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }
                let force = f(&self.particles[i], &self.particles[j]);
                self.particles.get_mut(i).unwrap().velocity += force;
            }
        }

        for p in self.particles.iter_mut() {
            if p.position.x < 0.0 || p.position.x > self.size.x {
                p.velocity.x *= -1.0;
            }
            if p.position.y < 0.0 || p.position.y > self.size.y {
                p.velocity.y *= -1.0;
            }
            if p.velocity.mag() > self.max_velocity {
                p.velocity = p.velocity.normalized() * self.max_velocity;
            }

            p.velocity *= self.drag_coefficient;
            p.position += p.velocity;
        }
    }
}
