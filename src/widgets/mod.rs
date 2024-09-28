use std::{cell::RefCell, process::Output};

use crate::{DrawingContext, KeyValues, WidgetRootRef, Value};
use sdl2::event::Event;

#[macro_export]
macro_rules! widget_list {
    ( $(x:expr),* ) => {
        {
            let temp_vec: Vec<Box<dyn Widget>> = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub trait Widget {
    fn init(&mut self, base: &WidgetBase);
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) -> Vec<String>;
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) -> Vec<String>;
    fn get_size(&self, context: &DrawingContext) -> IVec2;
    fn get_widget_base(&mut self) -> &mut WidgetBase;
    fn name(&self) -> &str;
}

impl std::fmt::Debug for dyn Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone)]
pub struct WidgetBase {
    pub root: WidgetRootRef,
    pub local: KeyValues,
}

impl WidgetBase {
    fn get(&self, item: &str) -> Option<Value> {
        self.root.get(item).or(self.local.get(item).cloned())
    }
}

#[derive(Debug, Clone)]
pub struct Anchor {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor {
            top: true,
            left: true,
            bottom: false,
            right: false,
        }
    }
}

impl Anchor {
    pub fn new(left: bool, right: bool, top: bool, bottom: bool) -> Self {
        Anchor {
            top,
            bottom,
            left,
            right
        }
    }
    pub fn center() -> Self {
        Anchor::new(true, true, true, true)
    }
    pub fn top_left() -> Self {
        Anchor::new(true, false, true, false)
    }
    pub fn top_right() -> Self {
        Anchor::new(false, true, true, false)
    }
    pub fn bottom_left() -> Self {
        Anchor::new(true, false, false, true)
    }
    pub fn bottom_right() -> Self {
        Anchor::new(false, true, false, true)
    }
    pub fn center_left() -> Self {
        Anchor::new(true, false, true, true)
    }
    pub fn center_right() -> Self {
        Anchor::new(false, true, true, true)
    }
    pub fn center_top() -> Self {
        Anchor::new(true, true, true, false)
    }
    pub fn center_bottom() -> Self {
        Anchor::new(true, true, false, true)
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "center" => Anchor::center(),
            "top-left" => Anchor::top_left(),
            "top-right" => Anchor::top_right(),
            "bottom-left" => Anchor::bottom_left(),
            "bottom-right" => Anchor::bottom_right(),
            "center-left" => Anchor::center_left(),
            "center-right" => Anchor::center_right(),
            "center-top" => Anchor::center_top(),
            "center-bottom" => Anchor::center_bottom(),
            _ => {
                panic!("invalid anchor type: {}", s);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Offset {
    Auto,
    Px(f32),
    Percent(f32),
    Vw(f32),
    Vh(f32),
    Vmin(f32),
    Vmax(f32),
}

impl Offset {
    fn to_percent(&self, parent_pos: &Rect, size: &IVec2, horiz: bool) -> f32 {
        match self {
            Offset::Auto => {
                if horiz {
                    size.x as f32 / parent_pos.w
                } else {
                    size.y as f32 / parent_pos.h
                }
            }
            Offset::Px(s) => {
                if horiz {
                    *s / parent_pos.w
                } else {
                    *s / parent_pos.h
                }
            }
            Offset::Percent(s) => {
                *s
            }
            _ => { 1.0 }
            // Offset::Vw(s) => {
            //     Offset::Px(*s * parent_pos.size.x)
            // }
            // Offset::Vh(s) => {
            //     Offset::Px(*s * parent_pos.size.y)
            // }
            // Offset::Vmin(s) => {
            //     if parent_pos.size.x < parent_pos.size.y {
            //         Offset::Px(*s * parent_pos.size.x)
            //     } else {
            //         Offset::Px(*s * parent_pos.size.y)
            //     }
            // }
            // Offset::Vmax(s) => {
            //     if parent_pos.size.x > parent_pos.size.y {
            //         Offset::Px(*s * parent_pos.size.x)
            //     } else {
            //         Offset::Px(*s * parent_pos.size.y)
            //     }
            // }
        }
    }
}

#[derive(Debug)]
pub struct Layout {
    pub x: Offset,
    pub y: Offset,
    pub w: Offset,
    pub h: Offset,
    pub anchor: Anchor,
}

#[derive(Debug, Clone, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Default for Layout {
    fn default() -> Self {
        Layout {
            x: Offset::Px(0.0),
            y: Offset::Px(0.0),
            w: Offset::Percent(1.0),
            h: Offset::Percent(1.0),
            anchor: Anchor::center()
        }
    }
}

impl Layout {
    pub fn get_px_dims(&self, size: IVec2, parent_pos: &Rect) -> Rect {
        let mut ret = Rect {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        };

        ret.w = self.w.to_percent(parent_pos, &size, true);
        ret.h = self.h.to_percent(parent_pos, &size, false);

        ret.x = self.x.to_percent(parent_pos, &size, true);
        ret.y = self.y.to_percent(parent_pos, &size, false);


        if self.anchor.top && self.anchor.bottom {
            ret.y = 0.5 + ret.y - ret.h / 2.0;
        } else if self.anchor.top {
            ret.y = ret.y;
        } else if self.anchor.bottom {
            ret.y = 1.0 - ret.y - ret.h;
        }
        if self.anchor.left && self.anchor.right {
            ret.x = 0.5 + ret.x - ret.w / 2.0;
        } else if self.anchor.left {
            ret.x = ret.x;
        } else if self.anchor.right {
            ret.x = 1.0 - ret.x - ret.w;
        }

        ret.x *= parent_pos.w;
        ret.w *= parent_pos.w;

        ret.y *= parent_pos.h;
        ret.h *= parent_pos.h;

        ret
    }
}

#[derive(Clone, Debug, Default)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        return IVec2 { x, y };
    }
}

impl std::ops::AddAssign for IVec2 {
    fn add_assign(&mut self, other: IVec2) {
        *self = IVec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Add for IVec2 {
    type Output = IVec2;
    fn add(self, other: IVec2) -> Self::Output {
        IVec2::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r,
            g,
            b,
            a
        }
    }
    pub fn from_hex(code: &str) -> Self {
        let r = u8::from_str_radix(&code[0..2], 16).unwrap();
        let g = u8::from_str_radix(&code[2..4], 16).unwrap();
        let b = u8::from_str_radix(&code[4..6], 16).unwrap();
        let a = if code.len() == 8 {
            u8::from_str_radix(&code[6..8], 16).unwrap()
        } else {
            255
        };

        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0
        }
    }
    pub fn rgb_to_srgb(&self) -> Color {
        Color {
            r: self.r.powf(1.0/2.2),
            g: self.g.powf(1.0/2.2),
            b: self.b.powf(1.0/2.2),
            a: self.a
        }
    }
    pub fn srgb_to_rgb(&self) -> Color {
        Color {
            r: self.r.powf(2.2),
            g: self.g.powf(2.2),
            b: self.b.powf(2.2),
            a: self.a
        }
    }
}

mod label;
pub use label::Label;
mod panel;
pub use panel::Panel;
mod container;
pub use container::SingleContainer;
mod button;
pub use button::Button;
mod row_container;
pub use row_container::RowContainer;
mod column_container;
pub use column_container::ColumnContainer;
mod dynamic_row;
pub use dynamic_row::DynamicRow;
mod line_edit;
pub use line_edit::LineEdit;
