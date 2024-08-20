use gtk4::{
    prelude::{BoxExt, PopoverExt},
    Box, Button, MenuButton, Orientation, Popover,
};

pub fn build_menu() -> MenuButton {
    let menu = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .build();

    let popover = Popover::new();

    let options = Box::new(Orientation::Vertical, 7);
    options.append(&Button::with_label("opt 1"));
    options.append(&Button::with_label("opt 2"));
    options.append(&Button::with_label("opt 3"));

    popover.set_child(Some(&options));
    menu.set_popover(Some(&popover));

    menu
}
