use gtk4::{
    prelude::{PopoverExt, RangeExt, WidgetExt, ButtonExt}, Button, Orientation, Popover, Scale
};

pub fn build_volume() -> Button {
    let menu = Button::builder()
        .icon_name("audio-volume-high-symbolic")
        .build();

    let popover = Popover::new();
    popover.set_position(gtk4::PositionType::Top);
    popover.set_parent(&menu);
    popover.set_hexpand(true);
    popover.set_vexpand(true);
    popover.set_height_request(100);
    popover.set_autohide(false);

    let scale = Scale::builder().orientation(Orientation::Vertical).build();
    scale.set_range(0., 100.);


    popover.set_child(Some(&scale));

    menu.connect_clicked(move |_| {
        popover.popup();
    });

    menu
}
