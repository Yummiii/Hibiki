use super::GStreamerPlayer;
use crate::{messenger::types::MessageType, player::commons::media_controls::MediaControls};
use gstreamer::{prelude::ElementExt, Message, State};
use gtk4::prelude::ObjectExt;
use std::sync::Arc;

pub fn connect_events(player: Arc<GStreamerPlayer>) {
    let bus = player.playbin.bus().unwrap();
    bus.add_signal_watch();

    bus.connect("message::state-changed", false, {
        let player = player.clone();

        move |a| {
            let msg = &a[1].get::<Message>().unwrap();

            if msg.src().map(|s| *s == player.playbin).unwrap_or(false) {
                let a = msg.structure().unwrap();
                let state = a.get::<State>("new-state");

                if state.is_ok() {
                    //this is just a temporary fix, i'll make the conversor later
                    player
                        .messenger
                        .send(MessageType::StateChanged, player.state());
                }
            }
            None
        }
    });
}
