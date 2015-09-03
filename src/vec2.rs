// A vec2 implementation
use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new_xy(x: f32, y: f32) -> Vec2 {
        Vec2 {
            x: x,
            y: y,
        }
    }

    pub fn new_rt(r: f32, t: f32) -> Vec2 {
        Vec2 {
            x: r*t.cos(),
            y: r*t.sin(),
        }
    }

    pub fn get_r(&self) -> f32 {
        (self.x*self.x+self.y*self.y).sqrt()
    }

    pub fn set_r(&mut self, radius: f32) {
        let theta = self.get_t();
        self.x = radius*theta.cos();
        self.y = radius*theta.sin();
    }

    pub fn get_t(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn set_t(&mut self, theta: f32) {
        let radius = self.get_r();
        self.x = radius*theta.cos();
        self.y = radius*theta.sin();
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn scalar_product(&self, num: f32) -> Vec2 {
        Vec2 {
            x: self.x * num,
            y: self.y * num,
        }
    }
}

/*impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}*/

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}
