use super::HibikiPipeline;
use gtk4::prelude::ObjectExt;

impl HibikiPipeline {
    pub fn _set_volume(&self, volume: f64) {
        self.playbin.set_property("volume", volume);
    }
}
