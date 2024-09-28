use crate::DrawingContext;
use super::WidgetBase;
use super::{Color, IVec2};
use crate::widgets::Rect;
use crate::widgets::Layout;
use crate::widgets::Widget;
use crate::Value;

use sdl2::event::Event;

#[macro_export]
macro_rules! layoutc {
    ( $layout:expr, $widget:expr ) => {
        SingleContainer {
            base: None,
            layout: $layout,
            widget: $widget
        }
    };
}

#[derive(Debug)]
pub struct SingleContainer<T: Widget> {
    pub base: Option<WidgetBase>,
    pub layout: Layout,
    pub widget: T,
}

impl<T: Widget> Widget for SingleContainer<T> {
    fn init(&mut self, base: &WidgetBase) {
        let chud = base.clone();
        self.base = Some(chud);
        self.widget.init(base);
    }
    fn name(&self) -> &str {
        "SingleContainer"
    }
    fn get_widget_base(&mut self) -> &mut WidgetBase {
        self.base.as_mut().unwrap()
    }
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) -> Vec<String> {
        let dim = self.layout.get_px_dims(self.widget.get_size(&context), &dims);
        self.widget.render(context, &dim)
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) -> Vec<String> {
        let dim = self.layout.get_px_dims(self.widget.get_size(&context), &dims);
        self.widget.handle_input(context, event, &dim)
    }
    //fn handle_message(&mut self, msg: &str) {
    //    self.widget.handle_message(msg);
    //}
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        self.widget.get_size(context)
    }
}
