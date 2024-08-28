macro_rules! on_message {
    ($messenger:expr, $msg_type:expr, $data_type:ty, $handler:expr) => {
        $messenger.on_message::<$data_type, _>($msg_type, $handler);
    };
}

pub(crate) use on_message;
