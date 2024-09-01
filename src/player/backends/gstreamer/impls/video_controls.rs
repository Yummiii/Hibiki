use crate::player::{backends::gstreamer::GStreamerPlayer, commons::video_controls::VideoControls};
use gtk4::{gdk::Paintable, prelude::ObjectExt, Picture};

impl VideoControls for GStreamerPlayer {
    fn set_widget(&self, widget: &Picture) {
        let paintable = self.widget.property::<Paintable>("paintable");
        widget.set_paintable(Some(&paintable));
    }
}
