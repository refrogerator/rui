use sdl2::event::Event;

use crate::widgets::Label;
use crate::widgets::Panel;
use crate::widgets::Widget;
use crate::widgets::Rect;
use super::{Color, IVec2};
use crate::DrawingContext;

use super::Layout;
use super::Offset;

#[derive(Debug)]
enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

#[derive(Debug)]
pub struct Button {
    pub panel: Panel,
    pub label: Label,
    pub hovered_panel: Panel,
    pub hovered_label: Label,
    pub pressed_panel: Panel,
    pub pressed_label: Label,
    pub callback: fn(),
    state: ButtonState,
}

impl Button {
    pub fn new(text: &str, callback: fn()) -> Self {
        let mut button = Button::default();
        button.label.text = text.to_string();
        button.hovered_label.text = text.to_string();
        button.pressed_label.text = text.to_string();
        button.callback = callback;
        button
    }
}

impl Default for Button {
    fn default() -> Self {
        Button {
            panel: Panel {
                color: Color::from_hex("111111"),
                rounding: Offset::Px(0.0),
            },
            label: Label {
                font_size: 12.0,
                color: Color::from_hex("ffffff"),
                text: "joe".to_string()
            },
            hovered_panel: Panel {
                color: Color::from_hex("888888"),
                rounding: Offset::Px(0.0),
            },
            hovered_label: Label {
                font_size: 12.0,
                color: Color::from_hex("ffffff"),
                text: "joe".to_string()
            },
            pressed_panel: Panel {
                color: Color::from_hex("000000"),
                rounding: Offset::Px(0.0),
            },
            pressed_label: Label {
                font_size: 12.0,
                color: Color::from_hex("ffffff"),
                text: "joe".to_string()
            },
            callback: || {},
            state: ButtonState::Normal
        }
    }
}

impl Widget for Button {
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) {
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
        match self.state {
            ButtonState::Normal => {
                self.panel.render(context, dims);
                self.label.render(context, &dim);
            },
            ButtonState::Hovered => {
                self.hovered_panel.render(context, dims);
                self.hovered_label.render(context, &dim);
            }
            ButtonState::Pressed => {
                self.pressed_panel.render(context, dims);
                self.pressed_label.render(context, &dim);
            }
        }
        context.draw_rounded_quad_outline(dims, &Color::from_hex("ffffff"), 0.0, 4.0);
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) {
        match *event {
            Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x: _x, y: _y } => {
                let x = _x as f32;
                let y = _y as f32;
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        if x > dims.x && x < dims.x + dims.w && y > dims.y && y < dims.y + dims.h {
                            (self.callback)();
                            self.state = ButtonState::Pressed;
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
    }
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        self.label.get_size(context)
    }
}
