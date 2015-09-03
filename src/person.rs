extern crate sfml;

use self::sfml::system::Time;
use self::sfml::graphics::{Color, Texture, Sprite, CircleShape, RenderWindow, RenderTarget, Vertex, VertexArray, PrimitiveType};
use self::sfml::system::Vector2f;

use entity::{Entity, CoriolisEvent};
use physicsobj::PhysicsObj;
use vec2::Vec2;
use std::f32::consts;

pub struct Person {
    feet: PhysicsObj,
    head: PhysicsObj,
    last_time: f32,
}

impl Person {
    pub fn new(feet_x: f32, feet_y: f32, omega: f32, rmax: f32) -> Person {
        let mut feet = PhysicsObj::new(Vec2::new_xy(feet_x,feet_y), omega, rmax);
        let mut head = PhysicsObj::new(Vec2::new_xy(feet_x, feet_y - 40.0), omega, rmax);
        Person {
            feet: feet,
            head: head,
            last_time: 0.0,
        }
    }
}

fn draw_vel_acc_arrows(pos: Vec2, vel: &mut Vec2, acc: &mut Vec2, target: &mut RenderWindow) {
    let mut velocity_arrow = VertexArray::new_init(PrimitiveType::Lines, 2).unwrap();
    velocity_arrow.append(
        &Vertex::new_with_pos_color(
            &Vector2f::new(
                pos.get_x(),
                pos.get_y()),
            &Color::green()));
    let mut r = vel.get_r();
    vel.set_r(r / 10.0);
    velocity_arrow.append(
        &Vertex::new_with_pos_color(
            &Vector2f::new(
                pos.get_x() + vel.get_x(),
                pos.get_y() + vel.get_y()),
            &Color::green()));
    let mut acceleration_arrow = VertexArray::new_init(PrimitiveType::Lines, 2).unwrap();
    acceleration_arrow.append(
        &Vertex::new_with_pos_color(
            &Vector2f::new(
                pos.get_x(),
                pos.get_y()),
            &Color::red()));
    r = acc.get_r();
    acc.set_r(r / 10.0);
    acceleration_arrow.append(
        &Vertex::new_with_pos_color(
            &Vector2f::new(
                pos.get_x() + acc.get_x(),
                pos.get_y() + acc.get_y()),
            &Color::red()));
    target.draw(&acceleration_arrow);
    target.draw(&velocity_arrow);
}

impl Entity for Person {
    fn update(&mut self, t: &Time) {
        let time = t.as_seconds();
        self.feet.tick(time - self.last_time);
        self.head.tick(time - self.last_time);
        self.last_time = time;
        //print!("{}\n", self.feet.get_position().get_t() * (360.0/(2.0*consts::PI)));
    }

    fn draw(&self, target: &mut RenderWindow) {
        let mut ballkugel = CircleShape::new_init(20.0, 16).unwrap();
        ballkugel.set_origin2f(20.0,20.0);
        let pos = self.feet.get_position();
        ballkugel.set_position2f(pos.get_x() as f32, pos.get_y() as f32);
        ballkugel.set_fill_color(&Color::cyan());
        let mut headball = CircleShape::new_init(20.0, 16).unwrap();
        headball.set_origin2f(20.0, 20.0);
        let headpos = self.head.get_position();
        headball.set_position2f(headpos.get_x(), headpos.get_y());
        headball.set_fill_color(&Color::yellow());
        draw_vel_acc_arrows(headpos, &mut self.head.get_vel(), &mut self.head.get_acc(), target);
        draw_vel_acc_arrows(pos, &mut self.feet.get_vel(), &mut self.feet.get_acc(), target);
        /*let mut velocity_arrow = VertexArray::new_init(PrimitiveType::Lines, 2).unwrap();
        velocity_arrow.append(
            &Vertex::new_with_pos_color(
                &Vector2f::new(
                    pos.get_x(),
                    pos.get_y()),
                &Color::green()));
        let mut vel = self.feet.get_vel();
        let mut r = vel.get_r();
        vel.set_r(r / 10.0);
        velocity_arrow.append(
            &Vertex::new_with_pos_color(
                &Vector2f::new(
                    pos.get_x() + vel.get_x(),
                    pos.get_y() + vel.get_y()),
                &Color::green()));
        let mut acceleration_arrow = VertexArray::new_init(PrimitiveType::Lines, 2).unwrap();
        acceleration_arrow.append(
            &Vertex::new_with_pos_color(
                &Vector2f::new(
                    pos.get_x(),
                    pos.get_y()),
                &Color::magenta()));
        let mut acc = self.feet.get_acc();
        r = acc.get_r();
        acc.set_r(r / 10.0);
        acceleration_arrow.append(
            &Vertex::new_with_pos_color(
                &Vector2f::new(
                    pos.get_x() + acc.get_x(),
                    pos.get_y() + acc.get_y()),
                &Color::magenta()));
        target.draw(&acceleration_arrow);
        target.draw(&velocity_arrow);*/
        target.draw(&ballkugel);
        target.draw(&headball);
    }

    fn handle_event(&mut self, ev: CoriolisEvent) {
        match ev {
            CoriolisEvent::Jump(vel) => self.feet.jump(vel),
        }
    }
}
