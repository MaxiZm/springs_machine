use std::mem;
use std::ops::Mul;
use crate::body::Body;
use crate::renderable::Renderable;

pub(crate) struct Spring {
    pub(crate) rest_length: f64,
    pub(crate) spring_constant: f64,
    color: u32
}


impl Spring {
    pub(crate) fn new(rest_length: f64, spring_constant: f64, color: u32) -> Spring {
        Spring {
            rest_length,
            spring_constant,
            color
        }
    }

    pub(crate) fn apply_force(&mut self, body1: &mut dyn Body, body2: &mut dyn Body) {
        let displacement = body2.get_position() - body1.get_position();
        let distance = displacement.magnitude();

        if distance != 0.0 {
            let direction = displacement / distance;
            let force_magnitude = -self.spring_constant * (distance - self.rest_length);
            let force = direction * force_magnitude;

            body1.apply_force(-force);
            body2.apply_force(force);
        }
    }
}

impl Spring {
    pub(crate) fn render(&self, buffer: &mut Vec<Vec<u32>>, body1: &dyn Body, body2: &dyn Body) {
        let (mut x1, mut y1) = (body1.get_position().x as i32, body1.get_position().y as i32);
        let (mut x2, mut y2) = (body2.get_position().x as i32, body2.get_position().y as i32);

        if x1 > x2 {
            mem::swap(&mut x1, &mut x2);
            mem::swap(&mut y1, &mut y2);
        }

        let (dx, dy) = (x2 - x1, y2 - y1);

        let mut set_pixel = |x: i32, y: i32, color: u32| {
            if x >= 0 && x < buffer[0].len() as i32 && y >= 0 && y < buffer.len() as i32 {
                buffer[y as usize][x as usize] = color;
            }
        };

        if dx.abs() >= dy.abs() {
            for i in x1..=x2 {
                let j = y1 + dy * (i - x1) / dx;
                set_pixel(i, j, self.color);
                set_pixel(i, j + 1, self.color);
                set_pixel(i, j - 1, self.color);
            }
        } else {
            if y1 > y2 {
                mem::swap(&mut x1, &mut x2);
                mem::swap(&mut y1, &mut y2);
            }
            for j in y1..=y2 {
                let i = x1 + dx * (j - y1) / dy;
                set_pixel(i, j, self.color);
                set_pixel(i + 1, j, self.color);
                set_pixel(i - 1, j, self.color);
            }
        }
    }

    pub(crate) fn update(&mut self, _dt: f64, body1: &mut dyn Body, body2: &mut dyn Body) {
        self.apply_force(body1, body2);
        let distance = (body2.get_position() - body1.get_position()).magnitude() - self.rest_length;
        // change color from red to green based on distance
        self.color = 0xFF0000 + (((0xFF as f64 * distance / 100.0) as u32) << 8);
    }
}