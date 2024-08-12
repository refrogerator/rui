use crate::DrawingContext;
use super::{Color, IVec2};
use crate::widgets::Rect;
use crate::widgets::Layout;
use crate::widgets::Widget;

use sdl2::event::Event;

//#[derive(Debug)]
pub struct ColumnContainer {
    pub layout: Layout,
    pub widgets: Vec<Box<dyn Widget>>,
    pub spacing: f32,
}

impl Widget for ColumnContainer {
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) {
        let dim = self.layout.get_px_dims(self.get_size(&context), &dims);
        let size = dim.h / self.widgets.len() as f32;
        for (joe, widget) in self.widgets.iter_mut().enumerate() {
            let dims2 = Rect { x: dim.x + dims.x, y: dim.y + dims.y + size * joe as f32, w: dim.w, h: size - self.spacing };
            widget.render(context, &dims2);
        }
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) {
        let dim = self.layout.get_px_dims(self.get_size(&context), &dims);
        let size = dim.h / self.widgets.len() as f32;
        for (joe, widget) in self.widgets.iter_mut().enumerate() {
            let dims2 = Rect { x: dim.x + dims.x, y: dim.y + dims.y + size * joe as f32, w: dim.w, h: size };
            widget.handle_input(context, event, &dims2);
        }
        //let dim = self.layout.get_px_dims(self.widget.get_size(&context), &dims);
        //self.widget.handle_input(context, event, &dim);
    }
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        let mut size = IVec2::new(0, 0);
        //for widget in self.widgets.iter() {
        //    size += widget.get_size(context);
        //}
        size
        //self.widget.get_size(context)
    }
}
