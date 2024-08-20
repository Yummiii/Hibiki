use gtk4::{prelude::BoxExt, ActionBar, Box, Button, Orientation};

pub fn build_footer() -> ActionBar {
    let footer = ActionBar::new();

    let btn = Box::new(Orientation::Horizontal, 0);
    btn.append(&Button::with_label("a"));

    footer.set_center_widget(Some(&btn));

    footer
}
