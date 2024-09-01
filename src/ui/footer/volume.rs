use crate::{
    messenger::{macros::on_message, types::MessageType},
    player::commons::{media_controls::MediaStates, Player},
    state::ArcPipe,
};
use gtk4::{
    glib::{clone, Propagation},
    prelude::{BoxExt, ButtonExt, PopoverExt, RangeExt, WidgetExt},
    Box, Button, Entry, InputPurpose, Label, Orientation, Popover, Scale,
};
use log::debug;

pub fn build_volume(state: ArcPipe<impl Player>) -> Button {
    let menu = Button::builder()
        .icon_name("audio-volume-high-symbolic")
        .build();

    let popover = Popover::new();
    popover.set_position(gtk4::PositionType::Top);
    popover.set_parent(&menu);
    popover.set_height_request(200);
    popover.set_width_request(75);

    let container = Box::builder().orientation(Orientation::Vertical).build();

    let scale = Scale::builder().orientation(Orientation::Vertical).build();
    scale.set_range(0., 100.);
    scale.set_inverted(true);
    scale.set_vexpand(true);

    let label = Label::new(Some(&scale.value().to_string()));
    let entry = Entry::builder()
        .input_purpose(InputPurpose::Number)
        .visible(false)
        .build();

    //todo: make when the label is clicked, the entry is shown and the label is hidden

    on_message!(
        state.messenger,
        MessageType::StateChanged,
        MediaStates,
        clone!(
            #[strong]
            label,
            #[strong]
            scale,
            #[strong]
            state,
            move |state| {
                if *state == MediaStates::Playing {
                    // let vol = pipeline.get_volume() * 100.;
                    // label.set_text(&format!("{:.1}%", vol));
                    // scale.set_value(vol)
                }
            }
        )
    );

    scale.connect_change_value(clone!(
        #[strong]
        label,
        #[strong]
        state,
        move |_, _, value| {
            debug!("Volume: {}", value);
            label.set_text(&format!("{:.1}%", value));
            state.player.set_volume(value / 100.);
            Propagation::Proceed
        }
    ));

    container.append(&label);
    container.append(&entry);
    container.append(&scale);

    popover.set_child(Some(&container));

    menu.connect_clicked(move |_| {
        if popover.is_visible() {
            popover.hide();
        } else {
            popover.popup();
        }
    });

    menu
}
