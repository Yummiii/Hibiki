#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaStates {
    Stopped,
    Playing,
    Paused,
}

//maybe I'll split this further into audio and video controls
pub trait MediaControls {
    fn load(&self, uri: &str);
    fn play(&self, uri: &str);

    fn set_state(&self, state: MediaStates);
    fn state(&self) -> MediaStates;

    fn set_volume(&self, volume: f64);
    fn volume(&self) -> f64;

    fn position(&self) -> Option<u64>;
    fn seek(&self, position: u64) -> Result<(), ()>;
    fn duration(&self) -> Option<u64>;
}
