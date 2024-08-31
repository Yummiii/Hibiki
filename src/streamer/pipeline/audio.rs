use super::HibikiPipeline;
use gtk4::prelude::ObjectExt;

impl HibikiPipeline {
    pub fn set_volume(&self, volume: f64) {
        self.playbin.set_property("volume", volume);
    }

    pub fn get_volume(&self) -> f64 {
        self.playbin.property::<f64>("volume")
    }
}
