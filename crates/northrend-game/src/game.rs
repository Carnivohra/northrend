use northrend_input::InputState;
use northrend_render::Camera;

pub trait Game {
    fn name(&self) -> &str;
    fn base_archive_order(&self) -> &[&str];
    fn terrain_map(&self) -> &str;
    fn tick(&mut self, _input: &InputState, _delta_time: f32) {}
    fn camera(&self, aspect_ratio: f32) -> Camera;
}
