use crate::streamer::utils::macros::make;
use audio::Audio;
use gstreamer::{prelude::{GstBinExtManual, ElementExt}, Element, Pipeline, State};
use gtk4::prelude::ObjectExt;
use log::debug;
use video::Video;

mod audio;
mod video;

pub struct HibikiPipeline {
    pub pipeline: Pipeline,
    // is this the best way to store these?
    pub source: Element,
    pub audio: Audio,
    pub video: Video,
}

impl HibikiPipeline {
    pub fn play(&self, uri: &str) {
        self.pipeline.set_state(State::Null).unwrap();
        self.source.set_property("uri", uri);
        self.pipeline.set_state(State::Playing).unwrap();
    }
}

pub(super) fn create_pipeline() -> HibikiPipeline {
    let pipeline = HibikiPipeline {
        pipeline: Pipeline::with_name("hibiki-pipeline"),
        source: make!("uridecodebin").unwrap(),
        audio: audio::create_elements(),
        video: video::create_elements(),
    };

    pipeline.source.set_property("message-forward", true);

    let mut elements = vec![];
    elements.push(&pipeline.source);
    elements.extend(pipeline.audio.to_vec());
    elements.extend(pipeline.video.to_vec());

    pipeline.pipeline.add_many(&elements).unwrap();
    debug!("{} elements in pipeline", elements.len());

    // Link the audio and video elements
    Element::link_many(pipeline.audio.to_vec()).unwrap();
    Element::link_many(pipeline.video.to_vec()).unwrap();

    pipeline
}
