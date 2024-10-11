use crate::DrawingContext;
use super::WidgetBase;
use super::{Color, IVec2};
use crate::widgets::Rect;
use crate::widgets::Layout;
use crate::widgets::Widget;

use sdl2::event::Event;

//#[derive(Debug)]
pub struct ColumnContainer {
    pub base: Option<WidgetBase>,
    pub widgets: Vec<Box<dyn Widget>>,
    pub spacing: f32,
}

impl Widget for ColumnContainer {
    fn init(&mut self, base: &WidgetBase) {
        let chud = base.clone();
        self.base = Some(chud);
        for w in &mut self.widgets {
            w.init(base);
        }
    }
    fn name(&self) -> &str {
        "Panel"
    }
    fn get_widget_base(&mut self) -> &mut WidgetBase {
        self.base.as_mut().unwrap()
    }
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) -> Vec<String> {
        let mut events = Vec::new();
        let size = dims.h / self.widgets.len() as f32;
        for (joe, widget) in self.widgets.iter_mut().enumerate() {
            let dims2 = Rect { x: dims.x, y: dims.y + size * joe as f32, w: dims.w, h: size - self.spacing };
            events.append(&mut widget.render(context, &dims2));
        }
        events
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) -> Vec<String> {
        let mut events = Vec::new();
        let size = dims.h / self.widgets.len() as f32;
        for (joe, widget) in self.widgets.iter_mut().enumerate() {
            let dims2 = Rect { x: dims.x, y: dims.y + size * joe as f32, w: dims.w, h: size };
            events.append(&mut widget.handle_input(context, event, &dims2));
        }
        //let dim = self.layout.get_px_dims(self.widget.get_size(&context), &dims);
        //self.widget.handle_input(context, event, &dim);
        events
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
