use crate::messenger::types::MessageType;
use events::end::eof;
use gstreamer::{
    prelude::{ElementExt, ElementExtManual},
    tags::{AudioCodec, Bitrate, LanguageCode, VideoCodec},
    ClockTime, Element, Message, State, TagList,
};
use gtk4::prelude::ObjectExt;
use log::info;
use pipeline::{create_pipeline, HibikiPipeline};
use std::sync::Arc;

mod events;
pub mod pipeline;
mod utils;

pub type ArcPipe = Arc<HibikiPipeline>;

pub fn init_pipeline() -> ArcPipe {
    let pipeline = Arc::new(create_pipeline());

    let bus = pipeline.playbin.bus().unwrap();
    bus.add_signal_watch();

    bus.connect("message::eos", false, {
        let pipeline = pipeline.clone();
        move |value| eof(value, pipeline.clone())
    });

    bus.connect("message::state-changed", false, {
        let pipeline = pipeline.clone();
        move |a| {
            let msg = &a[1].get::<Message>().unwrap();

            if msg.src().map(|s| *s == pipeline.playbin).unwrap_or(false) {
                let a = msg.structure().unwrap();
                let state = a.get::<State>("new-state");

                if let Ok(state) = state {
                    pipeline.messenger.send(MessageType::StateChanged, state);

                    if state == State::Playing {
                        analyze_streams(&pipeline.playbin);
                        let duration = pipeline.playbin.query_duration::<ClockTime>().unwrap();
                        pipeline
                            .messenger
                            .send(MessageType::DurationFound, duration.mseconds());
                    }
                }
            }
            None
        }
    });

    pipeline
}

fn analyze_streams(playbin: &Element) {
    let n_video = playbin.property::<i32>("n-video");
    let n_audio = playbin.property::<i32>("n-audio");
    let n_text = playbin.property::<i32>("n-text");
    info!("{n_video} video stream(s), {n_audio} audio stream(s), {n_text} text stream(s)");

    for i in 0..n_video {
        let tags = playbin.emit_by_name::<Option<TagList>>("get-video-tags", &[&i]);

        if let Some(tags) = tags {
            info!("video stream {i}:");
            if let Some(codec) = tags.get::<VideoCodec>() {
                info!("    codec: {}", codec.get());
            }
        }
    }

    for i in 0..n_audio {
        let tags = playbin.emit_by_name::<Option<TagList>>("get-audio-tags", &[&i]);

        if let Some(tags) = tags {
            info!("audio stream {i}:");
            if let Some(codec) = tags.get::<AudioCodec>() {
                info!("    codec: {}", codec.get());
            }
            if let Some(codec) = tags.get::<LanguageCode>() {
                info!("    language: {}", codec.get());
            }
            if let Some(codec) = tags.get::<Bitrate>() {
                info!("    bitrate: {}", codec.get());
            }
        }
    }

    for i in 0..n_text {
        let tags = playbin.emit_by_name::<Option<TagList>>("get-text-tags", &[&i]);

        if let Some(tags) = tags {
            info!("subtitle stream {i}:");
            if let Some(codec) = tags.get::<LanguageCode>() {
                info!("    language: {}", codec.get());
            }
        }
    }

    let current_video = playbin.property::<i32>("current-video");
    let current_audio = playbin.property::<i32>("current-audio");
    let current_text = playbin.property::<i32>("current-text");
    info!(
        "Currently playing video stream {current_video}, audio stream {current_audio}, text stream {current_text}"
    );
}
