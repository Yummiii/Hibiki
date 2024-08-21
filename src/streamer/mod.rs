use events::pad_added::pad_added;
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
    let pipeline_clone = pipeline.clone();
    let pipeline_clone2 = pipeline.clone();

    pipeline
        .source
        .connect_pad_added(move |src, pad| pad_added(src, pad, pipeline_clone.clone()));

    let bus = pipeline.pipeline.bus().unwrap();
    bus.add_signal_watch();

    //todo add handiling

    // bus.connect("message::error", false, |a| {
    //     println!("erro");
    //     None
    // });
    bus.connect("message::eos", false, move |_| {
        pipeline_clone2.pipeline.set_state(State::Null).unwrap();
        println!("aa");
        None
    });
    bus.connect("message::state-changed", false, |a| {
        let msg = &a[1].get::<Message>().unwrap();

        println!("{:#?}", msg);

        None
    });
    // bus.connect("message::application", false, |a| {
    //     println!("app");
    //     None
    // });


    pipeline
}
