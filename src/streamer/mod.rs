use events::end::eof;
use gstreamer::{prelude::ElementExt, Message, State};
use gtk4::prelude::ObjectExt;
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
            println!("{:#?}", msg);

            if msg.src().map(|s| *s == pipeline.playbin).unwrap_or(false) {
                let a = msg.structure().unwrap();
                let state = a.get::<State>("new-state");

                println!("{:?}", state);

                pipeline.sender.send_blocking(format!("{:?}", state)).unwrap();


                if let Ok(State::Playing) = state {
                    // pipeline.sender.send_blocking("aa".to_string()).unwrap();
                //     let n_video = pipeline.playbin.property::<i32>("n-video");
                //     let n_audio = pipeline.playbin.property::<i32>("n-audio");
                //     let n_text = pipeline.playbin.property::<i32>("n-text");
                //     println!("{n_video} video stream(s), {n_audio} audio stream(s), {n_text} text stream(s)");
                }
            }

            None
        }
    });

    // let b = pipeline.clone();
    pipeline.playbin.connect("audio-tags-changed", false, move |a| {
        // println!("{:#?}", a);
        // println!("{:#?}", b)
        // b.sender.send_blocking(format!("{:#?}", a)).unwrap();

        None
    });

    pipeline
}
