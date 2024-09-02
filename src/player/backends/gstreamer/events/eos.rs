use crate::{
    messenger::types::MessageType,
    player::{backends::gstreamer::GStreamerPlayer, commons::media_controls::MediaStates},
};
use gtk4::glib::Value;
use log::debug;
use std::sync::Arc;

pub fn eos(_: &[Value], player: Arc<GStreamerPlayer>) -> Option<Value> {
    debug!("Reached EOF");
    // player.playbin.set_state(State::Null).unwrap();
    player.messenger.send(MessageType::Eos, ());
    player
        .messenger
        .send(MessageType::StateChanged, MediaStates::Stopped);
    None
}
