use crate::player::{
    backends::gstreamer::GStreamerPlayer, commons::subtitle_controls::SubtitleControls,
};
use gtk4::prelude::ObjectExt;

impl SubtitleControls for GStreamerPlayer {
    fn set_subtitle(&self, subtitle: i32) {
        println!("{}", subtitle);
        //for the subtitles to change, the video must be playing, and I want it to also change when it is paused. There must be a hack for this
        self.playbin.set_property("current-text", subtitle);
        // self.set_state(MediaStates::Playing);
    }
}
