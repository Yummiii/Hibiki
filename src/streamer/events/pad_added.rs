use crate::streamer::ArcPipe;
use gstreamer::{
    prelude::{ElementExt, PadExt},
    Element, Pad,
};
use log::{error, info};

pub fn pad_added(src: &Element, pad: &Pad, pipeline: ArcPipe) {
    let new_pad_caps = pad.current_caps().unwrap();
    let new_pad_struct = new_pad_caps.structure(0).unwrap();
    let new_pad_type = new_pad_struct.name();

    let sink_pad = match new_pad_type.to_string().as_ref() {
        "audio/x-raw" => pipeline.audio.convert.static_pad("sink"),
        "video/x-raw" => pipeline.video.convert.static_pad("sink"),
        _ => {
            error!("It has type {}", new_pad_type);
            None
        }
    };

    if let Some(sink_pad) = sink_pad {
        if !sink_pad.is_linked() {
            if let Err(err) = pad.link(&sink_pad) {
                error!("Type is {}, but link failed: {}", new_pad_type, err);
            } else {
                info!("Link succeeded {}", new_pad_type);
            }
        }
    }
}
