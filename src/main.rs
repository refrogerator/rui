use rui::widgets::*;
use rui::Window;
use rui::prelude::*;

struct Test {
}

impl App for Test {
    fn handle_command(&mut self, cmd: String) {
        let mut words = cmd.split(" ");
        if words.next().unwrap() == "print" {
            for word in words {
                print!("{} ", word);
            }
            println!("");
        }
    }
}

fn main() {
    let button = SingleContainer {
        layout: Layout {
            x: Offset::Auto,
            y: Offset::Auto,
            w: Offset::Percent(2.0 / 3.0),
            h: Offset::Percent(2.0 / 3.0),
            anchor: Anchor::center()
        },
        widget: row_container!([
                    button!("button1", "print chud"),
                    button!("button2", "print chud2")
        ], 0.0)
    };

    let handler = Test {};
    let mut window = window!(handler, button);
    window.run();
}

