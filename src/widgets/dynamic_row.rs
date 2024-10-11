use std::collections::HashMap;

use crate::{DrawingContext, Value};
use super::WidgetBase;
use super::{Color, IVec2};
use crate::widgets::Rect;
use crate::widgets::Layout;
use crate::widgets::Widget;
use crate::widget_list;

use sdl2::event::Event;

//#[macro_export]
//macro_rules! row_container {
//    ( [ $($x:expr),* ], $spacing:expr ) => {
//        {
//        RowContainer {
//            base: None,
//            widgets: vec![$(Box::new($x)),*],
//            spacing: $spacing
//        }}
//    };
//}

//#[derive(Debug)]
pub struct DynamicRow<T: Widget + Clone> {
    pub base: Option<WidgetBase>,
    pub widget: T,
    pub widgets: Vec<T>,
    pub source: String,
    pub spacing: f32,
}

impl<T: Widget + Clone> Widget for DynamicRow<T> {
    fn init(&mut self, base: &WidgetBase) {
        let chud = base.clone();
        self.base = Some(chud);
        self.widget.init(base);
    }
    fn name(&self) -> &str {
        "Panel"
    }
    fn get_widget_base(&mut self) -> &mut WidgetBase {
        self.base.as_mut().unwrap()
    }
    fn render(&mut self, context: &mut DrawingContext, dims: &Rect) -> Vec<String> {
        let list = self.base.as_ref().unwrap().get(&self.source).unwrap();
        println!("{:?}", &list);
        let mut events = Vec::new();
        match list {
            Value::Array(ar) => {
                let size = dims.w / ar.len() as f32;
                if self.widgets.len() < ar.len() {
                    for i in 0..(ar.len() - self.widgets.len()) {
                        let mut new_widget = self.widget.clone();
                        new_widget.init(&WidgetBase {
                            local: HashMap::from([(String::from("self"), ar[self.widgets.len() + i].clone())]),
                            root: self.get_widget_base().root.clone()
                        });
                        self.widgets.push(new_widget);
                    }
                }
                for (joe, item) in self.widgets.iter_mut().enumerate() {
                    let dims2 = Rect { x: dims.x + size * joe as f32, y: dims.y, w: size, h: dims.h };
                    //self.widget.get_widget_base().local.insert("self".to_owned(), item.clone());
                    //dbg!(self.widget.get_widget_base().get("self"));
                    events.append(&mut item.render(context, &dims2));
                }
            }
            _ => {
                panic!("source {} is neither a hashmap nor an array", self.source);
            }
        }
        events
    }
    fn handle_input(&mut self, context: &mut DrawingContext, event: &Event, dims: &Rect) -> Vec<String> {
        let list = self.base.as_ref().unwrap().get(&self.source).unwrap();
        let mut events = Vec::new();
        match list {
            Value::Array(ar) => {
                let size = dims.w / ar.len() as f32;
                for (joe, item) in ar.iter().enumerate() {
                    let dims2 = Rect { x: dims.x + size * joe as f32, y: dims.y, w: size, h: dims.h };
                    self.widget.get_widget_base().local.insert("self".to_owned(), item.clone());
                    events.append(&mut self.widget.handle_input(context, event, &dims2));
                    self.widget.get_widget_base().local.clear();
                }
            }
            _ => {
                panic!("source {} is neither a hashmap nor an array", self.source);
            }
        }
        //println!("{:?}", events);
        events
    }
    fn get_size(&self, context: &DrawingContext) -> IVec2 {
        IVec2::new(0, 0)
        //self.widget.get_size(context)
    }
}
