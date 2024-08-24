use events::{end::eof, pad_added::pad_added};
use gstreamer::prelude::ElementExt;
use gtk4::prelude::ObjectExt;
use pipeline::{create_pipeline, HibikiPipeline};
use std::sync::Arc;

mod events;
pub mod pipeline;
mod utils;

pub type ArcPipe = Arc<HibikiPipeline>;

pub fn init_pipeline() -> ArcPipe {
    let pipeline = Arc::new(create_pipeline());

    // pipeline.source.connect_pad_added({
    //     let pipeline = pipeline.clone();
    //     move |src, pad| pad_added(src, pad, pipeline.clone())
    // });

    // pipeline.video.convert.connect("video-tags-changed", after, callback);

    let bus = pipeline.pipeline.bus().unwrap();
    bus.add_signal_watch();

    bus.connect("message::eos", false, {
        let pipeline = pipeline.clone();
        move |value| eof(value, pipeline.clone())
    });

    // bus.connect("message::state-changed", false, |a| {
    //     let msg = &a[1].get::<Message>().unwrap();

    //     println!("{:#?}", msg);

    //     None
    // });

    bus.connect_message(None, |_, b| {
        let a = b.view();

        println!("{:#?}", a);
    });

    pipeline
}
