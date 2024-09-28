use sdl2::event::Event;

use crate::widgets::Label;
use crate::widgets::Panel;
use crate::widgets::Widget;
use crate::widgets::Rect;
use super::WidgetBase;
use super::{Color, IVec2};
use crate::DrawingContext;

use super::Layout;
use super::Offset;

#[derive(Debug)]
pub struct LineEdit {
    pub base: Option<WidgetBase>,
    pub panel: Panel,
    pub label: Label,
    //pub callback: fn(),
}

impl LineEdit {
    pub fn new(callback: fn()) -> Self {
        LineEdit {
            panel: Panel {
                color: Color::from_hex("777777"),
                rounding: Offset::Auto,
                base: None,
            },
            label: Label {
                color: Color::new(1.0, 1.0, 1.0, 1.0),
                font_size: 12.0,
                text: String::new(),
                base: None
            },
            base: None
            //callback,
        }
    }
}

impl Default for LineEdit {
    fn default() -> Self {
        LineEdit {
            panel: Panel {
                base: None,
                color: Color::from_hex("777777"),
                rounding: Offset::Auto,
            },
            label: Label {
                color: Color::new(1.0, 1.0, 1.0, 1.0),
                font_size: 12.0,
                text: String::new(),
                base: None
            },
            base: None
            //callback: || {},
        }
    }
}

impl Widget for LineEdit {
    fn init(&mut self, base: &WidgetBase) {
        let chud = base.clone();
        self.base = Some(chud);
    }
    fn name(&self) -> &str {
        "Panel"
    }
    fn get_widget_base(&mut self) -> &mut WidgetBase {
        self.base.as_mut().unwrap()
    }
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) -> Vec<String> {
        let layout = Layout {
            x: Offset::Px(0.0),
            y: Offset::Px(0.0),
            w: Offset::Auto,
            h: Offset::Auto,
            anchor: super::Anchor::center()
        };
        let mut dim = layout.get_px_dims(self.label.get_size(&context), &dims);
        dim.x += dims.x;
        dim.y += dims.y;
        context.draw_rounded_quad_outline(dims, &Color::from_hex("ffffff"), 0.0, 4.0);
        Vec::new()
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) -> Vec<String> {
        match *event {
            Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x: _x, y: _y } => {
                let x = _x as f32;
                let y = _y as f32;
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        if x > dims.x && x < dims.x + dims.w && y > dims.y && y < dims.y + dims.h {
                            //return (self.callback)();
                            //self.state = ButtonState::Pressed;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Vec::new()
    }
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        self.label.get_size(context)
    }
}
