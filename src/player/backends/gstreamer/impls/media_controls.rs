use crate::player::{
    backends::gstreamer::GStreamerPlayer,
    commons::media_controls::{MediaControls, MediaStates},
};
use gstreamer::{
    prelude::{ElementExt, ElementExtManual},
    ClockTime, SeekFlags, State,
};
use gtk4::prelude::ObjectExt;

impl MediaControls for GStreamerPlayer {
    fn load(&self, uri: &str) {
        self.playbin.set_property("uri", uri);
    }

    fn play(&self, uri: &str) {
        self.set_state(MediaStates::Stopped);
        self.load(uri);
        self.set_state(MediaStates::Playing);
    }

    fn set_state(&self, state: MediaStates) {
        match state {
            MediaStates::Playing => self.playbin.set_state(State::Playing),
            MediaStates::Paused => self.playbin.set_state(State::Paused),
            MediaStates::Stopped => self.playbin.set_state(State::Null),
        }
        .unwrap();
    }

    fn state(&self) -> MediaStates {
        match self.playbin.current_state() {
            State::Playing => MediaStates::Playing,
            State::Paused => MediaStates::Paused,
            State::Null => MediaStates::Stopped,
            _ => MediaStates::Stopped,
        }
    }

    fn set_volume(&self, volume: f64) {
        self.playbin.set_property("volume", volume);
    }

    fn volume(&self) -> f64 {
        self.playbin.property::<f64>("volume")
    }

    fn position(&self) -> Option<u64> {
        let pos = self.playbin.query_position::<ClockTime>();
        pos.map(|pos| pos.mseconds())
    }

    fn duration(&self) -> Option<u64> {
        let dur = self.playbin.query_duration::<ClockTime>();
        dur.map(|dur| dur.mseconds())
    }

    fn seek(&self, position: u64) -> Result<(), ()> {
        //todo make error more descriptive
        self.playbin
            .seek_simple(
                SeekFlags::TRICKMODE | SeekFlags::FLUSH,
                ClockTime::from_mseconds(position),
            )
            .map_err(|_| ())
    }
}
