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

#[macro_export]
macro_rules! button {
    ($text:expr, $callback:expr) => {
       Button::new($text, String::from($callback)) 
    };
}

#[derive(Debug, Clone)]
enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub panel: Panel,
    pub label: Label,
    pub hovered_panel: Panel,
    pub hovered_label: Label,
    pub pressed_panel: Panel,
    pub pressed_label: Label,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub base: Option<WidgetBase>,
    pub style: ButtonStyle,
    pub callback: String,
    state: ButtonState,
}

impl Button {
    pub fn new(text: &str, callback: String) -> Self {
        let mut style = ButtonStyle::default();
        style.label.text = text.to_string();
        style.hovered_label.text = text.to_string();
        style.pressed_label.text = text.to_string();
        Button {
            base: None,
            style,
            callback,
            state: ButtonState::Normal
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle {
            panel: Panel {
                base: None,
                color: Color::from_hex("111111"),
                rounding: Offset::Px(0.0),
            },
            label: Label {
                base: None,
                font_size: 12.0,
                color: Color::from_hex("ffffff"),
                text: "joe".to_string()
            },
            hovered_panel: Panel {
                base: None,
                color: Color::from_hex("888888"),
                rounding: Offset::Px(0.0),
            },
            hovered_label: Label {
                base: None,
                font_size: 12.0,
                color: Color::from_hex("ffffff"),
                text: "joe".to_string()
            },
            pressed_panel: Panel {
                base: None,
                color: Color::from_hex("000000"),
                rounding: Offset::Px(0.0),
            },
            pressed_label: Label {
                base: None,
                font_size: 12.0,
                color: Color::from_hex("ffffff"),
                text: "joe".to_string()
            },
        }
    }
}

impl Widget for Button {
    fn init(&mut self, base: &WidgetBase) {
        let chud = base.clone();
        self.base = Some(chud);
        self.style.label.init(base);
        self.style.hovered_label.init(base);
        self.style.pressed_label.init(base);
    }
    fn name(&self) -> &str {
        "Button"
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
        let mut dim = layout.get_px_dims(self.style.label.get_size(&context), &dims);
        dim.x += dims.x;
        dim.y += dims.y;
        let mouse_pos = context.get_mouse_pos();
        if  (mouse_pos.x as f32) > dims.x &&
            (mouse_pos.x as f32) < dims.x + dims.w && 
            (mouse_pos.y as f32) > dims.y &&
            (mouse_pos.y as f32) < dims.y + dims.h {
            if context.get_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                self.style.pressed_panel.render(context, dims);
                self.style.pressed_label.render(context, &dim);
            } else {
                self.style.hovered_panel.render(context, dims);
                self.style.hovered_label.render(context, &dim);
            }
        } else {
            self.style.panel.render(context, dims);
            self.style.label.render(context, &dim);
        }
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
                            self.state = ButtonState::Pressed;
                            return vec![self.callback.clone()];
                        }
                    }
                    _ => {}
                }
            }
            Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x: _x, y: _y } => {
                let x = _x as f32;
                let y = _y as f32;
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        if x > dims.x && x < dims.x + dims.w && y > dims.y && y < dims.y + dims.h {
                            self.state = ButtonState::Hovered;
                        } else {
                            self.state = ButtonState::Normal;
                        }
                    }
                    _ => {}
                }
            }
            Event::MouseMotion { timestamp, window_id, which, mousestate, x: _x, y: _y, xrel, yrel } => {
                let x = _x as f32;
                let y = _y as f32;
                if x > dims.x && x < dims.x + dims.w && y > dims.y && y < dims.y + dims.h {
                    self.state = ButtonState::Hovered;
                } else {
                    self.state = ButtonState::Normal;
                }
            }
            _ => {}
        }
        Vec::new()
    }
    //fn handle_message(&mut self, _msg: &str) {
    //}
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        self.style.label.get_size(context)
    }
}
