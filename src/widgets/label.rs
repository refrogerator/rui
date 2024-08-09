use sdl2::event::Event;

use crate::DrawingContext;
use crate::widgets::Rect;
use crate::widgets::Widget;
use super::{Color, IVec2};

#[derive(Debug)]
pub struct LabelStyle {
}

#[derive(Debug)]
pub struct Label {
    pub font_size: f32,
    pub color: Color,
    pub text: String,
}

impl Widget for Label {
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) {
        context.draw_text(&self.text, IVec2::new(dims.x as i32, dims.y as i32), &self.color);
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) {}
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        let mut size = IVec2::new(0, context.get_current_font().max_advance.1 as i32);
        for ch in self.text.chars() {
            let glyph = &context.get_current_font().glyphs[ch as usize - 32];
            size.x += glyph.advance as i32;
        }
        size
    }
}
