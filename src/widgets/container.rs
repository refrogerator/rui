use crate::DrawingContext;
use super::{Color, IVec2};
use crate::widgets::Rect;
use crate::widgets::Layout;
use crate::widgets::Widget;

use sdl2::event::Event;

#[derive(Debug)]
pub struct SingleContainer<T: Widget> {
    pub layout: Layout,
    pub widget: T,
}

impl<T: Widget> Widget for SingleContainer<T> {
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) {
        let dim = self.layout.get_px_dims(self.widget.get_size(&context), &dims);
        self.widget.render(context, &dim);
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) {
        let dim = self.layout.get_px_dims(self.widget.get_size(&context), &dims);
        self.widget.handle_input(context, event, &dim);
    }
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        self.widget.get_size(context)
    }
}
