use crate::{
    messenger::{macros::on_message, types::MessageType},
    player::commons::{media_controls::MediaStates, Player},
    state::ArcPipe,
};
use gtk4::{
    glib::{clone, Propagation},
    prelude::{ButtonExt, PopoverExt, RangeExt, ScaleExt, WidgetExt},
    Button, Orientation, Popover, Scale,
};

pub fn build_volume(state: ArcPipe<impl Player>) -> Button {
    //I want to add a way for the user to manually type the volume if they want to

    let menu = Button::builder()
        .icon_name("audio-volume-high-symbolic")
        .build();

    let popover = Popover::new();
    popover.set_position(gtk4::PositionType::Top);
    popover.set_parent(&menu);
    popover.set_height_request(210);

    let scale = Scale::builder().orientation(Orientation::Vertical).build();
    scale.set_range(0., 100.);
    scale.set_inverted(true);
    scale.set_vexpand(true);
    scale.set_draw_value(true);

    // scale.add_mark(50., gtk4::PositionType::Top, None);

    on_message!(
        state.messenger,
        MessageType::StateChanged,
        MediaStates,
        clone!(
            #[strong]
            scale,
            #[strong]
            state,
            move |media_state| {
                if *media_state == MediaStates::Playing {
                    let vol = state.player.volume() * 100.;
                    scale.set_value(vol);
                }
            }
        )
    );

    scale.connect_change_value(clone!(
        #[strong]
        state,
        move |_, _, value| {
            state.player.set_volume(value / 100.);
            Propagation::Proceed
        }
    ));

    popover.set_child(Some(&scale));

    //is this redundant?
    menu.connect_clicked(move |_| {
        if popover.is_visible() {
            popover.hide();
        } else {
            popover.popup();
        }
    });

    menu
}
