use crate::streamer::utils::macros::make;
use async_channel::{unbounded, Receiver, Sender};
use gstreamer::{prelude::ElementExt, Element, State};
use gtk4::prelude::ObjectExt;

mod audio;
mod video;

pub struct HibikiPipeline {
    pub playbin: Element,
    pub widget: Element,
    pub receiver: Receiver<String>,
    pub sender: Sender<String>,
}

impl HibikiPipeline {
    pub fn play(&self, uri: &str) {
        self.playbin.set_state(State::Null).unwrap();
        self.playbin.set_property("uri", uri);
        self.playbin.set_state(State::Playing).unwrap();
    }
}

pub(super) fn create_pipeline() -> HibikiPipeline {
    let (sender, receiver) = unbounded::<String>();

    let pipeline = HibikiPipeline {
        //maybe use playbin3 in the future?
        playbin: make!("playbin").unwrap(),
        widget: make!("gtk4paintablesink").unwrap(),
        receiver,
        sender,
    };

    pipeline
        .playbin
        .set_property("video-sink", &pipeline.widget);
    pipeline.playbin.set_property("connection-speed", 56u64);

    pipeline
}
