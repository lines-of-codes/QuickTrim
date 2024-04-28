use fltk::{
    app,
    button::Button,
    enums::{Color, Font, FrameType},
    prelude::*,
    text::{StyleTableEntry, TextBuffer, TextDisplay, WrapMode},
    window::Window,
};

const TEXT_STYLES: &[StyleTableEntry] = &[
    StyleTableEntry {
        color: Color::Black,
        font: Font::Helvetica,
        size: 16,
    },
    StyleTableEntry {
        color: Color::Black,
        font: Font::HelveticaBold,
        size: 20,
    },
];

pub struct AboutDialog;

fn add_text_with_style(buf: &mut TextBuffer, sbuf: &mut TextBuffer, text: &str, style: &str) {
    buf.append(text);
    sbuf.append(&style.repeat(text.len()));
}

impl AboutDialog {
    pub fn default() -> Self {
        let win_width = 400;
        let win_height = 200;

        let mut win = Window::default()
            .with_size(win_width, win_height)
            .with_label("About QuickTrim");
        win.set_color(Color::White);

        let mut buf = TextBuffer::default();
        let mut sbuf = TextBuffer::default();

        add_text_with_style(&mut buf, &mut sbuf, "QuickTrim", "B");
        buf.append("\n");
        buf.append(format!("QuickTrim version: {}\n", env!("CARGO_PKG_VERSION")).as_str());
        buf.append("QuickTrim is a program allowing you to trim your media files quickly. ");
        buf.append("This program is licensed under the GNU GPLv3 license. ");
        buf.append("This software uses libraries from the FFmpeg project under the GPLv3 license.");

        let mut text = TextDisplay::new(15, 10, win_width - 30, win_height - 50, "");
        text.set_frame(FrameType::FlatBox);
        text.set_buffer(buf.clone());
        text.set_highlight_data(sbuf, TEXT_STYLES.to_vec());
        text.wrap_mode(WrapMode::AtBounds, 0);

        let mut close_btn = Button::new(win_width - 60 - 15, win_height - 10 - 20, 60, 20, "OK");
        close_btn.set_frame(FrameType::RFlatBox);

        win.end();
        win.make_modal(true);
        win.show();

        close_btn.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });

        while win.shown() {
            app::wait();
        }
        Self {}
    }
}
