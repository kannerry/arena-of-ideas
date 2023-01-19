use super::*;

mod animate_unit;
mod field;
mod draw_model_units;

pub use animate_unit::*;
pub use field::*;
pub use draw_model_units::*;

pub trait VisualEffect {
    fn draw(
        &self,
        render: &ViewRender,
        framebuffer: &mut ugli::Framebuffer,
        t: Time,
        model: &VisualNodeModel,
    ) {
        #![allow(unused_variables)]
    }
    fn update(&self, model: &mut VisualNodeModel, t: Time) {
        #![allow(unused_variables)]
    }
    fn get_order(&self) -> i32 {
        0
    }
    fn get_duration(&self) -> Time {
        1.0
    }
}
