use crate::streamer::utils::macros::make;
use gstreamer::Element;
use gtk4::prelude::ObjectExt;

pub struct Video {
    // pub convert: Element,
    // pub sink: Element,
    pub widget: Element,
}

impl Video {
    // pub fn to_vec(&self) -> Vec<&Element> {
    //     vec![&self.sink]
    // }
}

pub fn create_elements() -> Video {
    let video = Video {
        // convert: make!("videoconvert").unwrap(),
        // sink: make!("glsinkbin").unwrap(),
        widget: make!("gtk4paintablesink").unwrap(),
    };

    // video.sink.set_property("sink", &video.widget);

    video
}
