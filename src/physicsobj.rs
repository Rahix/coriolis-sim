// A physics object

use vec2::Vec2;
use std::f32::consts;

pub struct PhysicsObj {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    custom_acc: Vec2,
    omega: f32,
    max_radius: f32,
}

impl PhysicsObj {
    pub fn new(pos: Vec2, omega: f32, max_radius: f32) -> PhysicsObj {
        PhysicsObj {
            pos: pos,// + Vec2::new_xy(100.0, 0.0),
            vel: Vec2::new_xy(0.0, -300.0),
            acc: Vec2::new_xy(0.0,0.0),
            custom_acc: Vec2::new_xy(0.0,0.0),
            omega: omega,
            max_radius: max_radius,
        }
    }

    pub fn tick(&mut self, elapsed: f32) {
        // If person touches max_radius, set acceleration
        let mut acc_circle = Vec2::new_xy(0.0, 0.0);
        if self.pos.get_r() >= self.max_radius {
            //self.acc.set_x(-(self.omega * self.omega) * self.pos.get_x());
            //self.acc.set_y(-(self.omega * self.omega) * self.pos.get_y());
            acc_circle = self.pos.scalar_product(-(self.omega * self.omega));
            self.vel = Vec2::new_rt(self.omega * self.max_radius, self.pos.get_t() + consts::PI / 2.0);
        }
        self.acc = acc_circle + self.custom_acc;
        // Next step of velocity based on acceleration
        self.vel = self.vel + self.acc.scalar_product(elapsed);
        // Next step of position based on velocity
        self.pos = self.pos + self.vel.scalar_product(elapsed);
        //self.pos.set_r(self.max_radius);
        //let t = (self.pos.get_t() + self.omega * elapsed);
        //print!("{}\n", self.pos.get_t() * (360.0 / (2.0 * consts::PI)));
        //println!("{}\t{}", self.pos.get_x(), self.pos.get_y());
        //self.pos.set_t(t);
        //self.acc = Vec2::new_xy(0.0, 0.0);
    }

    pub fn get_position(&self) -> Vec2 {
        self.pos
    }

    pub fn get_vel(&self) -> Vec2 {
        self.vel
    }

    pub fn get_acc(&self) -> Vec2 {
        self.acc
    }

    // vel in px/s
    pub fn jump(&mut self, vel: f32) {
        let jump_vel = Vec2::new_rt(vel, self.pos.get_t() + consts::PI);
        if self.pos.get_r() >= ( self.max_radius - 5.0 ) {
            self.vel = self.vel + jump_vel;
            self.pos.set_r(self.max_radius - 1.0);
        }
    }

    pub fn on_floor(&self) -> bool {
        self.pos.get_r() >= ( self. max_radius - 5.0 )
    }

    pub fn custom_acc(&mut self, acc: Vec2) {
        self.custom_acc = acc;
    }
}
