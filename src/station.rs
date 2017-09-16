extern crate sfml;

use self::sfml::system::Time;
use self::sfml::graphics::{Texture, Sprite, CircleShape, RenderWindow, RenderTarget, Transformable};

use entity::{Entity, CoriolisEvent};
use std::f32::consts;

pub struct Station<'a> {
    tex: Texture,
    circle: CircleShape<'a>,
    rotation: f32,
    omega: f32,
    align_view: bool,
}

impl<'a> Station<'a> {
    pub fn new<'b>(radius: f32, omega: f32) -> Station<'b> {
        let mut station = Station {
            rotation: 0.0,
            omega: omega,
            tex: Texture::new_from_file("res/StationCross.png").unwrap(),
            circle: CircleShape::new_init(radius, 64).unwrap(),
            align_view: true, // Set to true if you want the view to be aligned to the rotation
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

        if self.align_view {
            let mut view = target.get_view().to_owned();
            view.set_rotation(self.rotation);
            target.set_view(&view);
        }
    }

    fn handle_event(&mut self, _: CoriolisEvent) {
    }
}
