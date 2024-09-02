use crate::{
    messenger::types::MessageType,
    player::{backends::gstreamer::GStreamerPlayer, commons::media_controls::MediaControls},
};
use gstreamer::{Message, State};
use gtk4::glib::Value;
use std::sync::Arc;

pub fn state_changed(msg: &[Value], player: Arc<GStreamerPlayer>) -> Option<Value> {
    let msg = &msg[1].get::<Message>().unwrap();

    if msg.src().map(|s| *s == player.playbin).unwrap_or(false) {
        let structure = msg.structure().unwrap();
        let state = structure.get::<State>("new-state");

        if state.is_ok() {
            //this is just a temporary fix, i'll make the conversor later (this is a lie, i'll never do it)
            player
                .messenger
                .send(MessageType::StateChanged, player.state());
        }
    }

    None
}
