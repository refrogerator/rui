use rui::widget_list;
use rui::widgets::*;
use rui::Value;
use rui::Window;
use rui::prelude::*;

struct Test {
}

impl App for Test {
    fn handle_command(&mut self, root: WidgetRootRef, cmd: String) {
        if cmd == "add_todo" {
            let next = root.get("next").unwrap();
            root.modify_array("todos", |todos| {
                todos.push(next.clone());
            });
            root.modify_int("next", |next| {
                *next += 1;
            });
        }
        let mut words = cmd.split(" ");
        if words.next().unwrap() == "print" {
            for word in words {
                print!("{} ", word);
            }
            println!("");
            let mut _root = root.modify("mama", |j| {
                if let Value::Bool(ref mut mama) = j {
                    *mama = !*mama;
                }
            });
        }
    }
    fn update(&mut self, root: WidgetRootRef) {}
}

#[derive(Debug)]
enum Token {
    Ident(String),
    Num(f32),
    Offset(Offset),
    Op(char)
}

fn layout(text: &str) -> Layout {
    let mut cur_str = String::new();
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut number = false;
    for char in text.chars() {
        match char {
            ',' => {
                output.push(Token::Ident(cur_str.clone()));
                cur_str.clear();
                output.append(&mut stack);
            }
            ':' => {
                output.push(Token::Ident(cur_str.clone()));
                cur_str.clear();
                stack.push(Token::Ident(":".to_string()));
            }
            _ => {
                if char.is_numeric() || (char == '.' && number) {
                    if cur_str.is_empty() {
                        number = true;
                    }
                } else if char.is_whitespace() {
                    if number {
                        output.push(Token::Num(cur_str.parse().unwrap()));
                        cur_str.clear();
                        number = false;
                    } else {
                        if !cur_str.is_empty() {
                            output.push(Token::Ident(cur_str.clone()));
                            cur_str.clear();
                        }
                    }
                    continue;
                } else {
                    if number {
                        output.push(Token::Num(cur_str.parse().unwrap()));
                        cur_str.clear();
                        number = false;
                    }
                }
                cur_str.push(char);
            }
        }
    }
    output.push(Token::Ident(cur_str.clone()));
    cur_str.clear();
    output.append(&mut stack);
    println!("{:?}", output);
    let mut ret = Layout::default();
    for token in output {
        match token {
            Token::Ident(ref s) => {
                match s.as_str() {
                    "%" => {
                        if let Token::Num(val) = stack.pop().unwrap() {
                            stack.push(Token::Offset(Offset::Percent(val / 100.0)));
                        } else {
                            panic!();
                        }
                    }
                    "px" => {
                        if let Token::Num(val) = stack.pop().unwrap() {
                            stack.push(Token::Offset(Offset::Px(val)));
                        } else {
                            panic!();
                        }
                    }
                    ":" => {
                        let a2 = stack.pop().unwrap();
                        let a1 = stack.pop().unwrap();
                        if let Token::Ident(ref name) = a1 {
                            match a2 {
                                Token::Offset(val) => {
                                    match name.as_str() {
                                        "x" => {
                                            ret.x = val;
                                        }
                                        "y" => {
                                            ret.y = val;
                                        }
                                        "w" => {
                                            ret.w = val;
                                        }
                                        "h" => {
                                            ret.h = val;
                                        }
                                        _ => {}
                                    }
                                }
                                Token::Ident(ref val) => {
                                    match name.as_str() {
                                        "anchor" => {
                                            ret.anchor = Anchor::from_str(val);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {
                        stack.push(token);
                    }
                }
            }
            Token::Num(n) => {
                stack.push(token);
            }
            _ => {}
        }
    }
    ret
}

fn main() {
    let button = button!("press for more", "add_todo");
    let button2 = DynamicRow {
        base: None,
        widget: button!("jort: {self}", "print chud"),
        widgets: Vec::new(),
        source: "todos".to_string(),
        spacing: 0.0
    };
    let column = layoutc!(layout("w: 100%, h: 100%"), ColumnContainer {
        base: None,
        spacing: 0.0,
        widgets: (vec![Box::new(button), Box::new(button2)]),
    });
    let handler = Test {};
    let mut window = window!(handler, rui::KeyValues::from([
            ("todos".to_string(), Value::Array(vec![
                     Value::String("test".to_string()),
            ])),
            ("mama".to_string(), Value::Bool(true)),
            ("next".to_string(), Value::Int(0))
    ]), column);
    window.run();
}
