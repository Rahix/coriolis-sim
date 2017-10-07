extern crate sfml;

use self::sfml::system::Time;
use self::sfml::graphics::RenderWindow;

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum CoriolisEvent {
    Jump(f32),
}

pub trait Entity {
    fn update(&mut self, t: &Time);
    fn draw(&self, target: &mut RenderWindow);
    fn handle_event(&mut self, ev: CoriolisEvent);
}

pub struct World {
    ents: Vec<Box<Entity>>,
}

impl World {
    pub fn new() -> World {
        World {
            ents:Vec::new()
        }
    }

    pub fn add<T: 'static + Entity>(&mut self, ent: T) {
        self.ents.push(Box::new(ent));
    }

    pub fn update(&mut self, t: Time) {
        for ent in &mut self.ents {
            ent.update(&t);
        }
    }

    pub fn draw(&self, target: &mut RenderWindow) {
        for ent in self.ents.iter() {
            ent.draw(target);
        }
    }

    pub fn send_event(&mut self, ev: CoriolisEvent) {
        for ent in &mut self.ents {
            ent.handle_event(ev);
        }
    }
}
