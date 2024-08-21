use crate::streamer::utils::macros::make;
use gstreamer::Element;
use gtk4::prelude::ObjectExt;

pub struct Audio {
    pub convert: Element,
    pub resample: Element,
    pub volume: Element,
    pub sink: Element,
}

impl Audio {
    pub fn to_vec(&self) -> Vec<&Element> {
        vec![&self.convert, &self.resample, &self.volume, &self.sink]
    }
}

pub fn create_elements() -> Audio {
    //todo add error handling
    let audio = Audio {
        convert: make!("audioconvert").unwrap(),
        resample: make!("audioresample").unwrap(),
        volume: make!("volume").unwrap(),
        sink: make!("autoaudiosink").unwrap(),
    };

    audio.volume.set_property("volume", 0.03);

    audio
}
