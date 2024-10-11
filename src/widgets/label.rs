use sdl2::event::Event;

use crate::DrawingContext;
use crate::widgets::Rect;
use crate::widgets::Widget;
use crate::Value;
use super::WidgetBase;
use super::{Color, IVec2};

#[derive(Debug, Clone)]
pub struct LabelStyle {
    pub font_size: f32,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub base: Option<WidgetBase>,
    pub font_size: f32,
    pub color: Color,
    pub text: String,
}

#[derive(Debug)]
enum Token {
    Ident(String),
    Num(f32),
    Interp(String)
}

impl Label {
    fn interp_text(&self) -> String {
        let mut cur_str = String::new();
        let mut temp = String::new();
        let mut interp = false;
        let base = self.base.as_ref().unwrap();
        //base.get("joe");
        for char in self.text.chars() {
            match char {
                '{' => interp = true,
                '}' => if interp {
                    //dbg!(&temp);
                    //dbg!(&base.local);
                    println!("{:?}", base.get(&temp));
                    cur_str.push_str(&base.get(&temp).unwrap().to_str());
                    interp = false;
                } else {
                    panic!("invalid interp syntax");
                },
                _ => if interp { temp.push(char) } else { cur_str.push(char) },
            }
        }
        cur_str
    }
}

impl Widget for Label {
    fn init(&mut self, base: &WidgetBase) {
        let chud = base.clone();
        self.base = Some(chud);
    }
    fn name(&self) -> &str {
        "Label"
    }
    fn get_widget_base(&mut self) -> &mut WidgetBase {
        self.base.as_mut().unwrap()
    }
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) -> Vec<String> {
        context.draw_text(&self.interp_text(), IVec2::new(dims.x as i32, dims.y as i32), &self.color);
        Vec::new()
    }
    fn handle_input(&mut self, _context: &mut DrawingContext, _event: &Event, _dims: &Rect) -> Vec<String> {
        Vec::new()
    }
    //fn handle_message(&mut self, _msg: &str) {}
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        let mut size = IVec2::new(0, context.get_current_font().max_advance.1 as i32);
        for ch in self.interp_text().chars() {
            let glyph = &context.get_current_font().glyphs[ch as usize - 32];
            size.x += glyph.advance as i32;
        }
        size
    }
}
