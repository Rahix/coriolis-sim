extern crate sfml;

use self::sfml::system::Time;
use self::sfml::graphics::{Texture, Sprite, CircleShape, RenderWindow, RenderTarget};

use entity::{Entity, CoriolisEvent};
use std::rc::Rc;
use std::f32::consts;

pub struct Station<'a> {
    tex: Texture,
    circle: CircleShape<'a>,
    radius: f32,
    rotation: f32,
    omega: f32,
}

impl<'a> Station<'a> {
    pub fn new<'b>(radius: f32, omega: f32) -> Station<'b> {
        let mut station = Station {
            radius: radius,
            rotation: 0.0,
            omega: omega,
            tex: Texture::new_from_file("res/StationCross.png").unwrap(),
            circle: CircleShape::new_init(radius, 64).unwrap(),
        };
        station.circle.set_origin2f(radius,radius);
        //station.sprite.set_texture(&station.tex, true);
        //station.circle.set_position2f(0.0, 0.0);
        station
    }
}

impl<'a> Entity for Station<'a> {
    fn update(&mut self, t: &Time) {
        self.rotation = self.omega * t.as_seconds() * (360.0/(2.0*consts::PI));
        //print!("{}\t{}\t", t.as_seconds(), self.rotation);

    }

    fn draw(&self, target: &mut RenderWindow) {
        let mut sprite = Sprite::new().unwrap();
        sprite.set_texture(&self.tex, true);
        sprite.set_origin2f(256.0, 256.0);
        sprite.set_rotation(self.rotation);
        target.draw(&self.circle);
        target.draw(&sprite);
    }

    fn handle_event(&mut self, ev: CoriolisEvent) {
    }
}
