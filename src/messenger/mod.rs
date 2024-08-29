use gtk4::glib::spawn_future_local;
use log::error;
use std::{
    any::{type_name, Any},
    process,
    sync::Arc,
};
use tokio::sync::broadcast::{self, Sender};
use types::MessageType;

pub mod macros;
pub mod types;

#[derive(Debug, Clone)]
pub struct Message {
    pub msg_type: MessageType,
    pub data: Arc<dyn Any + Send + Sync>,
}

pub struct Messenger {
    pub sender: Sender<Message>,
}

impl Messenger {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel::<Message>(2);

        Messenger { sender }
    }

    pub fn send<T: Send + Sync + 'static>(&self, msg_type: MessageType, data: T) {
        self.sender
            .send(Message {
                msg_type,
                data: Arc::new(data),
            })
            .unwrap();
    }

    pub fn on_message<T: Send + Sync + 'static, F: Fn(Arc<T>) + 'static>(
        &self,
        msg_type: MessageType,
        f: F,
    ) {
        let mut rx = self.sender.subscribe();

        spawn_future_local({
            async move {
                while let Ok(msg) = rx.recv().await {
                    let data = msg.data;
                    if msg.msg_type == msg_type {
                        let data = data.downcast::<T>().unwrap_or_else(|_| {
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
