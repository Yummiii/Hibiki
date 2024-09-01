use crate::messenger::Messenger;
use media_controls::MediaControls;
use std::sync::Arc;
use video_controls::VideoControls;

pub mod media_controls;
pub mod video_controls;

pub trait Player: VideoControls + MediaControls + Send + Sync + 'static {}

pub trait PlayerFactory {
    type Player: Player;
    fn new(messenger: Arc<Messenger>) -> Arc<Self::Player>;
}
