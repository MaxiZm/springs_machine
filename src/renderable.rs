pub trait Renderable {
    fn render(&self, buffer: &mut Vec<Vec<u32>>);
    fn update(&mut self, dt: f64);
}