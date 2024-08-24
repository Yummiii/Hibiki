use crate::streamer::ArcPipe;
use gstreamer::{prelude::ElementExt, State};
use gtk4::glib::Value;
use log::debug;

pub fn eof(_: &[Value], pipeline: ArcPipe) -> Option<Value> {
    pipeline.pipeline.set_state(State::Null).unwrap();
    debug!("Reached EOF");
    None
}
