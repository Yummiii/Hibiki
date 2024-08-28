use crate::{messenger::{macros::on_message, types::MessageType}, streamer::ArcPipe};
use gstreamer::State;
use gtk4::{prelude::{BoxExt, ButtonExt}, Box, Button};

pub fn build_controls(pipeline: ArcPipe) -> Box {
    let controls = Box::new(gtk4::Orientation::Horizontal, 0);

    let play_button = Button::builder().icon_name("media-playback-pause-symbolic").build();

    play_button.connect_clicked({
        let pipeline = pipeline.clone();
        move |_| {
            pipeline.toggle();
        }
    });

    on_message!(pipeline.messenger, MessageType::StateChanged, State, {
        let play_button = play_button.clone();
        move |state| {
            if *state == State::Playing {
                play_button.set_icon_name("media-playback-pause-symbolic");
            } else {
                play_button.set_icon_name("media-playback-start-symbolic");
            }
        }
    });

    controls.append(&play_button);

    controls
}
