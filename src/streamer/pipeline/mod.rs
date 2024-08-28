use crate::{messenger::Messenger, streamer::utils::macros::make};
use gstreamer::{prelude::{ElementExt, ElementExtManual}, Element, State};
use gtk4::prelude::ObjectExt;

pub mod audio;

pub struct HibikiPipeline {
    pub playbin: Element,
    pub widget: Element,
    pub messenger: Messenger,
}

impl HibikiPipeline {
    pub fn play(&self, uri: &str) {
        self.playbin.set_state(State::Null).unwrap();
        self.playbin.set_property("uri", uri);
        self.playbin.set_state(State::Playing).unwrap();
    }

    pub fn toggle(&self) {
        let state = self.playbin.current_state();
        match state {
            State::Playing => {
                self.playbin.set_state(State::Paused).unwrap();
            }
            State::Paused => {
                self.playbin.set_state(State::Playing).unwrap();
            }
            _ => {}
        }
    }
}

pub(super) fn create_pipeline() -> HibikiPipeline {
    let pipeline = HibikiPipeline {
        //maybe use playbin3 in the future?
        playbin: make!("playbin").unwrap(),
        widget: make!("gtk4paintablesink").unwrap(),
        messenger: Messenger::new(),
    };

    pipeline
        .playbin
        .set_property("video-sink", &pipeline.widget);
    pipeline.playbin.set_property("connection-speed", 56u64);
    pipeline.playbin.set_property("message-forward", true);

    pipeline
}
