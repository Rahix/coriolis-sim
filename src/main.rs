#![allow(dead_code)]

extern crate sfml;
extern crate nalgebra as na;
extern crate eagre_ecs as ecs;

mod physics;
mod graphics;
mod world;

mod drawable;

mod station;
mod feet;
mod head;

pub const ANGULAR_VELOCITY: f32 = std::f32::consts::PI / 2.0;
pub const JUMP_SPEED: f32 = 500.0;

pub const STATION_RADIUS: f32 = 300.0;
pub const FEET_RADIUS: f32 = 20.0;
pub const HEAD_RADIUS: f32 = 20.0;

fn main() {
    world::coriolis_sim();
}
