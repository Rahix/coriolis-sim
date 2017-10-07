mod sim;
mod entity;
mod station;
mod person;
mod physicsobj;
mod vec2;

use sim::CoriolisSim;

fn main() {
    let mut sim = CoriolisSim::new();
    sim.start();
}
