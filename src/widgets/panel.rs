use sdl2::event::Event;

use super::{Color, IVec2};
use crate::DrawingContext;
use crate::widgets::Rect;
use crate::widgets::Widget;
use crate::widgets::Offset;

#[derive(Debug)]
pub struct Panel {
    pub color: Color,
    pub rounding: Offset,
}

impl Widget for Panel {
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) -> Vec<String> {
        context.draw_rounded_quad(&dims, &self.color, 0.0);
        Vec::new()
    }
    fn handle_input(&mut self, _context: &mut DrawingContext, _event: &Event, _dims: &Rect) -> Vec<String> {
        Vec::new()
    }
    //fn handle_message(&mut self, _msg: &str) {}
    fn get_size(&self, _context: &DrawingContext) -> IVec2 {
        IVec2::new(0, 0)
    }
}
