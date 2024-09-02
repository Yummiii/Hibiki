use crate::{
    messenger::{macros::on_message, types::MessageType},
    player::commons::{media_controls::MediaStates, Player},
    state::ArcPipe,
};
use gtk4::{
    prelude::{BoxExt, ButtonExt},
    Box, Button,
};
use log::error;

pub fn build_controls(state: ArcPipe<impl Player>) -> Box {
    let controls = Box::new(gtk4::Orientation::Horizontal, 0);

    let play_button = Button::builder()
        .icon_name("media-playback-start-symbolic")
        .build();

    play_button.connect_clicked({
        let state = state.clone();
        move |_| {
            if state.player.state() == MediaStates::Stopped {
                error!("No media loaded");
            } else {
                let media_state = if state.player.state() == MediaStates::Playing {
                    MediaStates::Paused
                } else {
                    MediaStates::Playing
                };

                state.player.set_state(media_state);
            }
        }
    });

    on_message!(state.messenger, MessageType::StateChanged, MediaStates, {
        let play_button = play_button.clone();
        move |state| {
            println!("{:?}", state);
            if *state == MediaStates::Playing {
                play_button.set_icon_name("media-playback-pause-symbolic");
            } else {
                play_button.set_icon_name("media-playback-start-symbolic");
            }
        }
    });

    controls.append(&play_button);

    controls
}
