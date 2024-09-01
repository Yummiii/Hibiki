use crate::{
    messenger::Messenger,
    player::commons::{Player, PlayerFactory},
};
use std::sync::Arc;

//idk, I just liked the name
pub type ArcPipe<T> = Arc<AppState<T>>;

pub struct AppState<T: Player> {
    pub messenger: Arc<Messenger>,
    pub player: Arc<T>,
}

impl<T: Player> AppState<T> {
    pub fn new<F: PlayerFactory<Player = T>>() -> Arc<Self> {
        let messenger = Arc::new(Messenger::new());
        Arc::new(Self {
            messenger: messenger.clone(),
            player: F::new(messenger),
        })
    }
}
