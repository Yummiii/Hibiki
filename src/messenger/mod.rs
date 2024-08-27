use async_channel::{unbounded, Receiver, Sender};
use gtk4::glib::spawn_future_local;
use log::error;
use std::{
    any::{type_name, Any},
    process,
};

pub struct Message {
    pub msg_type: MessageType,
    pub data: Box<dyn Any + Send + Sync>,
}

pub struct Messenger {
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}

impl Messenger {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Messenger { sender, receiver }
    }

    pub fn send<T: Send + Sync + 'static>(&self, msg_type: MessageType, data: T) {
        self.sender
            .send_blocking(Message {
                msg_type,
                data: Box::new(data),
            })
            .unwrap();
    }

    pub fn on_message<T: 'static, F: Fn(Box<T>) + 'static>(&self, msg_type: MessageType, f: F) {
        spawn_future_local({
            let receiver = self.receiver.clone();
            async move {
                while let Ok(msg) = receiver.recv().await {
                    if msg.msg_type == msg_type {
                        let data = msg.data.downcast::<T>().unwrap_or_else(|_| {
                            error!("Error parsing message data to type: {}", type_name::<T>());
                            process::exit(1);
                        });

                        f(data)
                    }
                }
            }
        });
    }
}

#[derive(PartialEq)]
pub enum MessageType {
    DurationFound,
}
