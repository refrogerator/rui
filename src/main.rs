use rui::widgets::*;
use rui::Window;

fn main() {
    let button = RowContainer {
        layout: Layout {
            x: Offset::Px(0.0),
            y: Offset::Px(0.0),
            w: Offset::Percent(1.0 / 3.0),
            h: Offset::Percent(0.1),
            anchor: Anchor::center()
        },
        widgets: vec![Box::new(Button::new("chudnite", || println!("chud"))), Box::new(Button::new("chudnite2", || println!("chud")))],
        spacing: 0.0
    };

    let mut window = Window::new(vec![Box::new(button)]);
    window.run();
}
