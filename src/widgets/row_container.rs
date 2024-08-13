use crate::DrawingContext;
use super::{Color, IVec2};
use crate::widgets::Rect;
use crate::widgets::Layout;
use crate::widgets::Widget;
use crate::widget_list;

use sdl2::event::Event;

#[macro_export]
macro_rules! row_container {
    ( [ $($x:expr),* ], $spacing:expr ) => {
        {
        RowContainer {
            widgets: vec![$(Box::new($x)),*],
            spacing: $spacing
        }}
    };
}

//#[derive(Debug)]
pub struct RowContainer {
    pub widgets: Vec<Box<dyn Widget>>,
    pub spacing: f32,
}

impl Widget for RowContainer {
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) -> Vec<String> {
        let size = dims.w / self.widgets.len() as f32;
        let mut events = Vec::new();
        for (joe, widget) in self.widgets.iter_mut().enumerate() {
            let dims2 = Rect { x: dims.x + size * joe as f32, y: dims.y, w: size, h: dims.h };
            events.append(&mut widget.render(context, &dims2));
        }
        events
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) -> Vec<String> {
        let size = dims.w / self.widgets.len() as f32;
        let mut events = Vec::new();
        for (joe, widget) in self.widgets.iter_mut().enumerate() {
            let dims2 = Rect { x: dims.x + size * joe as f32, y: dims.y, w: size, h: dims.h };
            events.append(&mut widget.handle_input(context, event, &dims2));
        }
        //println!("{:?}", events);
        events
        //let dim = self.layout.get_px_dims(self.widget.get_size(&context), &dims);
        //self.widget.handle_input(context, event, &dim);
    }
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        IVec2::new(0, 0)
        //self.widget.get_size(context)
    }
}
