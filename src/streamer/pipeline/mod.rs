use crate::streamer::utils::macros::make;
use audio::Audio;
use gstreamer::{prelude::GstBinExtManual, Element, Pipeline};
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

pub(super) fn create_pipeline() -> HibikiPipeline {
    let pipeline = HibikiPipeline {
        pipeline: Pipeline::with_name("hibiki-pipeline"),
        source: make!("uridecodebin").unwrap(),
        audio: audio::create_elements(),
        video: video::create_elements(),
    };

    // there is probably a better way to do this

    let mut elements = [pipeline.audio.to_vec(), pipeline.video.to_vec()].concat();
    elements.push(&pipeline.source);
    pipeline.pipeline.add_many(&elements).unwrap();
    debug!("{} elements in pipeline", elements.len());

    Element::link_many(pipeline.audio.to_vec()).unwrap();
    Element::link_many(pipeline.video.to_vec()).unwrap();

    pipeline
}
