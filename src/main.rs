mod vector;
mod body;
mod renderable;
mod spring;

use minifb::{Key, Window, WindowOptions};
use body::Body;
use renderable::Renderable;
use vector::Vector;
use crate::body::BoxBody;

fn update_with_2d_buffer(window: &mut Window, buffer: &Vec<Vec<u32>>, screen_buffer: &mut Vec<u32>) {
    for (i, &val) in buffer.iter().flatten().enumerate() {
        screen_buffer[i] = val;
    }

    window.update_with_buffer(screen_buffer, buffer[0].len(), buffer.len()).unwrap();
}

const FPS: f64 = 60.0;
const TIME_SCALE: f64 = 1.0;
const DT: f64 = TIME_SCALE / FPS;
const BG: u32 = 0x000000;
const GRAVITY: Vector = Vector { x: 0.0, y: 100.0 };

const WINDOW_OPEN: (usize, usize) = (1000, 1000);

fn main() {
    let mut buffer: Vec<Vec<u32>> = vec![vec![0; WINDOW_OPEN.0]; WINDOW_OPEN.1];
    let mut screen_buffer: Vec<u32> = vec![0; WINDOW_OPEN.0 * WINDOW_OPEN.1];
    let mut window = Window::new("Spring",
                                  WINDOW_OPEN.0,
                                  WINDOW_OPEN.1,
                                  WindowOptions {
                                      resize: true,
                                      ..WindowOptions::default()
                                    }
    ).unwrap();

    let mut body = BoxBody::new(Vector::new(320.0, 240.0), (20.0, 20.0), 0xFF0000, 1.0);
    let mut body2: BoxBody = BoxBody::new(Vector::new(320.0, 480.0), (20.0, 20.0), 0xFF0000, 1.0);
    let mut body3 = BoxBody::new(Vector::new(320.0, 720.0), (20.0, 20.0), 0xFF0000, 1.0);
    body.set_velocity(Vector::new(80.0, 50.0));
    let mut ceiling_body = BoxBody::new_static(Vector::new(320.0, 0.0), (640.0, 20.0), 0x00FF00, Vector::new(0.0, 0.0));
    let mut spring = spring::Spring::new( 100.0, 1.0, 0x0000FF);
    let mut spring2 = spring::Spring::new( 100.0, 1.0, 0x0000FF);
    let mut spring3 = spring::Spring::new( 100.0, 1.0, 0x0000FF);
    let mut spring4 = spring::Spring::new( 100.0, 1.0, 0x0000FF);
    let mut spring5 = spring::Spring::new( 700.0, 1.0, 0x0000FF);

    let mut trajectory: Vec<Vector> = Vec::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let t1 = std::time::Instant::now();
        if window.get_size() != (buffer[0].len(), buffer.len()) {
            let (nw, nh) = window.get_size();
            for row in 0..buffer.len() {
                buffer[row].resize(nw, 0);
            }
            buffer.resize(nh, vec![0; nw]);
            screen_buffer.resize(nw * nh, 0);
        }

        let mut t_buffer;
        let mut t_spring;
        let mut t_body;
        let mut t_render;

        {
            // flash the buffer
            let xsize = buffer[0].len();
            buffer.fill(vec![BG; xsize]);



            body.set_acceleration(Vector::new(0.0, 0.0));
            body2.set_acceleration(Vector::new(0.0, 0.0));
            body3.set_acceleration(Vector::new(0.0, 0.0));

            t_buffer = std::time::Instant::now();

            {
                spring.update(DT, &mut body, &mut ceiling_body);
                spring2.update(DT, &mut body2, &mut body);
                spring3.update(DT, &mut body3, &mut body2);
                spring4.update(DT, &mut body3, &mut body);
                spring5.update(DT, &mut body3, &mut ceiling_body);
            }

            t_spring = std::time::Instant::now();

            {
                body.apply_force(GRAVITY);
                body2.apply_force(GRAVITY);
                body3.apply_force(GRAVITY);
                body.update(DT);
                body2.update(DT);
                body3.update(DT);
                ceiling_body.update(DT);
            }

            t_body = std::time::Instant::now();

            {
                spring.render(&mut buffer, &body, &ceiling_body);
                spring2.render(&mut buffer, &body2, &body);
                spring3.render(&mut buffer, &body3, &body2);
                spring4.render(&mut buffer, &body3, &body);
                spring5.render(&mut buffer, &body3, &ceiling_body);
                body.render(&mut buffer);
                body2.render(&mut buffer);
                body3.render(&mut buffer);
                ceiling_body.render(&mut buffer);
            }

            t_render = std::time::Instant::now();
        }


        trajectory.push(body.get_position());
        trajectory.push(body2.get_position());
        trajectory.push(body3.get_position());
        if trajectory.len() > 100 * 3 {
            trajectory.remove(0);
        }


        for i in 0..trajectory.len() {
            let (x, y) = (trajectory[i].x as i32, trajectory[i].y as i32);
            if x >= 0 && x < buffer[0].len() as i32 && y >= 0 && y < buffer.len() as i32 {
                buffer[y as usize][x as usize] = 0xFFFFFF;
            }
        }

        let t_trajectory = std::time::Instant::now();

        update_with_2d_buffer(&mut window, &buffer, &mut screen_buffer);
        let t2 = std::time::Instant::now();
        let elapsed = t2 - t1;
        // println!("Total: {:?}, Buffer: {:?}, Spring: {:?}, Body: {:?}, Render: {:?}, Trajectory: {:?}, Update: {:?}", elapsed, t_buffer - t1, t_spring - t_buffer, t_body - t_spring, t_render - t_body, t_trajectory - t_render, t2 - t_trajectory);
        if elapsed.as_millis() < (1000.0 / FPS) as u128 {
            std::thread::sleep(std::time::Duration::from_millis(((1000.0 / FPS) as u128 - elapsed.as_millis()) as u64));
        }

    }
}
