use crate::DrawingContext;
use super::{Color, IVec2};
use crate::widgets::Rect;
use crate::widgets::Layout;
use crate::widgets::Widget;

use sdl2::event::Event;

//#[derive(Debug)]
pub struct RowContainer {
    pub layout: Layout,
    pub widgets: Vec<Box<dyn Widget>>,
    pub spacing: f32,
}

impl Widget for RowContainer {
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) {
        let dim = self.layout.get_px_dims(self.get_size(&context), &dims);
        let size = dim.w / self.widgets.len() as f32;
        for (joe, widget) in self.widgets.iter_mut().enumerate() {
            let dims2 = Rect { x: dims.x + dim.x + size * joe as f32, y: dims.y + dim.y, w: size, h: dim.h };
            widget.render(context, &dims2);
        }
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) {
        let dim = self.layout.get_px_dims(self.get_size(&context), &dims);
        let size = dim.w / self.widgets.len() as f32;
        for (joe, widget) in self.widgets.iter_mut().enumerate() {
            let dims2 = Rect { x: dims.x + dim.x + size * joe as f32, y: dims.y + dim.y, w: size, h: dim.h };
            widget.handle_input(context, event, &dims2);
        }
        //let dim = self.layout.get_px_dims(self.widget.get_size(&context), &dims);
        //self.widget.handle_input(context, event, &dim);
    }
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        IVec2::new(0, 0)
        //self.widget.get_size(context)
    }
}
