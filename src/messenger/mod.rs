use async_channel::{unbounded, Receiver, Sender};
use gstreamer::Fraction;

pub struct Messenger {
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}

impl Messenger {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Messenger { sender, receiver }
    }

    pub fn send(&self, message: Message) {
        self.sender.send_blocking(message).unwrap();
    }
}

pub enum Message {
    FpsFound(Fraction),
    DurationFound(u64),
}
