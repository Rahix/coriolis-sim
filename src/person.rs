extern crate sfml;

use self::sfml::system::Time;
use self::sfml::graphics::{Color, CircleShape, RenderWindow, RenderTarget, Vertex, VertexArray, PrimitiveType, Transformable, Shape};
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
        let feet = PhysicsObj::new(Vec2::new_xy(feet_x,feet_y), omega, rmax);
        let head = PhysicsObj::new(Vec2::new_xy(feet_x, feet_y + 40.0), omega, rmax);
        Person {
            feet: feet,
            head: head,
            last_time: 0.0,
        }
    }
}

fn draw_vel_acc_arrows(pos: Vec2, vel: &mut Vec2, acc: &mut Vec2, target: &mut RenderWindow) {
    let mut velocity_arrow = VertexArray::new_init(PrimitiveType::sfLines, 2).unwrap();
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
    let mut acceleration_arrow = VertexArray::new_init(PrimitiveType::sfLines, 2).unwrap();
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
        //Constraint
        let feet_pos = self.feet.get_position();
        let head_pos = self.head.get_position();
        let dist_vec = head_pos + feet_pos.scalar_product(-1.0);
        let acc = (dist_vec.get_r() - 40.0) * 16000.0;
        let mut acc_damp = Vec2::new_xy(0.0, 0.0); //(self.head.get_vel() + self.feet.get_vel().scalar_product(-1.0)).scalar_product(-5.0);
        let mut acc_standup = Vec2::new_xy(0.0, 0.0);
        if self.feet.on_floor() {
            acc_standup = Vec2::new_rt(8000.0, head_pos.get_t() + consts::PI);
            acc_damp = (self.head.get_vel() + self.feet.get_vel().scalar_product(-1.0)).scalar_product(-3.0);
        }
        //self.feet.custom_acc(Vec2::new_rt(acc, dist_vec.get_t()));
        self.head.custom_acc(Vec2::new_rt(acc, dist_vec.scalar_product(-1.0).get_t())+ acc_standup + acc_damp);
        self.feet.tick(time - self.last_time);
        self.head.tick(time - self.last_time);
        self.last_time = time;
        //print!("{}\t{}\t{}\t{}\n", feet_pos.get_x(), feet_pos.get_y(), head_pos.get_x(), head_pos.get_y());
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
            CoriolisEvent::Jump(vel) => {self.feet.jump(vel);self.head.jump(vel);},
        }
    }
}
