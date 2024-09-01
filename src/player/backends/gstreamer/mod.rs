use crate::{
    messenger::Messenger,
    player::commons::{Player, PlayerFactory},
};
use events::connect_events;
use gstreamer::Element;
use gtk4::prelude::ObjectExt;
use std::sync::Arc;
use utils::macros::make;

mod events;
pub mod impls;
mod utils;

pub struct GStreamerPlayer {
    pub messenger: Arc<Messenger>,
    pub playbin: Element,
    pub widget: Element,
}

impl PlayerFactory for GStreamerPlayer {
    type Player = GStreamerPlayer;

    fn new(messenger: Arc<Messenger>) -> Arc<Self> {
        let player = Arc::new(GStreamerPlayer {
            messenger,
            playbin: make!("playbin").unwrap(),
            widget: make!("gtk4paintablesink").unwrap(),
        });

        player.playbin.set_property("video-sink", &player.widget);
        player.playbin.set_property("connection-speed", 56u64);
        player.playbin.set_property("message-forward", true);

        connect_events(player.clone());

        player
    }
}

impl Player for GStreamerPlayer {}