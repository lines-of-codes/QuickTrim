use fltk::{
    app,
    enums::{Color, FrameType, Shortcut},
    menu::{MenuBar, MenuFlag},
    prelude::*,
    window::Window,
};

mod about;
use crate::about::AboutDialog;

fn main() {
    println!("Hello, world!");
    let app = app::App::default();
    let win_width = 800;
    let win_height = 600;
    let mut win = Window::new(100, 100, win_width, win_height, "QuickTrim");
    win.set_color(Color::White);

    let mut menu_bar = MenuBar::new(0, 0, win_width, 25, "");
    menu_bar.set_frame(FrameType::BorderBox);
    menu_bar.set_color(Color::from_hex(0xd0d0d0));
    menu_bar.add("Help/About", Shortcut::None, MenuFlag::Normal, |_| {
        AboutDialog::default();
    });

    win.end();
    win.show();
    app.run().unwrap();
}
