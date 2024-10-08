use crate::vector::Vector;
use crate::renderable::Renderable;

pub struct BoxBody {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
    render_box: (f64, f64),
    color: u32,
    mass: f64,
    is_static: bool,
}

impl BoxBody {
    pub(crate) fn new (position: Vector, render_box: (f64, f64), color: u32, mass: f64) -> BoxBody {
        BoxBody {
            position,
            velocity: Vector::new(0.0, 0.0),
            acceleration: Vector::new(0.0, 0.0),
            render_box,
            color,
            mass,
            is_static: false,
        }
    }

    pub(crate) fn new_static (position: Vector, render_box: (f64, f64), color: u32, velocity: Vector) -> BoxBody {
        BoxBody {
            position,
            velocity,
            acceleration: Vector::new(0.0, 0.0),
            render_box,
            color,
            mass: 0.0,
            is_static: true,
        }
    }

    pub(crate) fn get_acceleration(&self) -> Vector {
        self.acceleration
    }

    pub(crate) fn set_acceleration(&mut self, acceleration: Vector) {
        self.acceleration = acceleration;
    }

    pub(crate) fn get_velocity(&self) -> Vector {
        self.velocity
    }

    pub(crate) fn set_velocity(&mut self, velocity: Vector) {
        self.velocity = velocity;
    }
}

impl Renderable for BoxBody {
    fn render(&self, buffer: &mut Vec<Vec<u32>>) {
        let x = self.position.x;
        let y = self.position.y;
        let (w, h) = self.render_box;

        let lx = (x - self.render_box.0 / 2f64) as i32;
        let rx = (x + self.render_box.0 / 2f64) as i32;
        let ty = (y - self.render_box.1 / 2f64) as i32;
        let by = (y + self.render_box.1 / 2f64) as i32;

        for i in lx..rx {
            for j in ty..by {
                if i >= 0 && i < buffer[0].len() as i32 && j >= 0 && j < buffer.len() as i32 {
                    buffer[j as usize][i as usize] = self.color;
                }
            }
        }
    }

    fn update(&mut self, dt: f64) {
        self.velocity = self.velocity + self.acceleration * dt;
        self.position = self.position + self.velocity * dt;
    }
}

impl Body for BoxBody {
    fn apply_force(&mut self, force: Vector) {
        if self.is_static {
            return;
        }
        self.acceleration = self.acceleration + force / self.mass;
    }



    fn get_position(&self) -> Vector {
        self.position
    }
}

pub trait Body {
    fn apply_force(&mut self, force: Vector);

    fn get_position(&self) -> Vector;
}