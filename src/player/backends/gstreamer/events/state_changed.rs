use crate::{
    messenger::types::MessageType,
    player::{
        backends::gstreamer::GStreamerPlayer,
        commons::media_controls::{MediaControls, MediaStates},
    },
};
use gstreamer::{Message, State, TagList};
use gtk4::{glib::Value, prelude::ObjectExt};
use std::sync::Arc;

pub fn state_changed(msg: &[Value], player: Arc<GStreamerPlayer>) -> Option<Value> {
    let msg = &msg[1].get::<Message>().unwrap();

    if msg.src().map(|s| *s == player.playbin).unwrap_or(false) {
        let structure = msg.structure().unwrap();
        let state = structure.get::<State>("new-state");

        if state.is_ok() {
            if player.state() == MediaStates::Playing {
                analyze_streams(player.clone());
            }

            //this is just a temporary fix, i'll make the conversor later (this is a lie, i'll never do it)
            player
                .messenger
                .send(MessageType::StateChanged, player.state());
        }
    }

    None
}

//for now it'll probably be just the subtitles lol
fn analyze_streams(player: Arc<GStreamerPlayer>) {
    for i in 0..player.playbin.property::<i32>("n-text") {
        if let Some(tags) = player
            .playbin
            .emit_by_name::<Option<TagList>>("get-text-tags", &[&i])
        {
            println!("{}", i);
            println!("{:#?}", tags);

            // let a = tags.get::<gstreamer::tags::TrackId>();
            // println!("{:#?}", a);
        }
    }
}
