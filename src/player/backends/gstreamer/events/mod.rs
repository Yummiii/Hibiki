use super::GStreamerPlayer;
use eos::eos;
use gstreamer::prelude::ElementExt;
use gtk4::prelude::ObjectExt;
use state_changed::state_changed;
use std::sync::Arc;

mod eos;
mod state_changed;

pub fn connect_events(player: Arc<GStreamerPlayer>) {
    let bus = player.playbin.bus().unwrap();
    bus.add_signal_watch();

    bus.connect("message::state-changed", false, {
        let player = player.clone();
        move |msg| state_changed(msg, player.clone())
    });

    bus.connect("message::eos", false, {
        let player = player.clone();
        move |msg| eos(msg, player.clone())
    });
}
