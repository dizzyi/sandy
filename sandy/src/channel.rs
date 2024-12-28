use std::sync::{mpsc, Arc, LazyLock, Mutex};

pub type ChannelRx<T> = Arc<Mutex<mpsc::Receiver<T>>>;
pub type ChannelTx<T> = Arc<mpsc::Sender<T>>;

#[derive(Debug, Clone)]
pub struct Channel<T> {
    rx: ChannelRx<T>,
    tx: ChannelTx<T>,
}

pub type LazyChannel<T> = LazyLock<Channel<T>>;

impl<T> Channel<T> {
    pub fn read(&self) -> Option<T> {
        match self.rx.lock().ok()?.try_recv() {
            Ok(t) => Some(t),
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => {
                unreachable!("Unexpected closure of static mpsc channel");
            }
        }
    }
    pub fn send(&self, msg: impl Into<T>) {
        match self.tx.send(msg.into()) {
            Ok(()) => {}
            Err(mpsc::SendError(_t)) => {
                unreachable!("Unexpected closure of static mpsc channel")
            }
        }
    }
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let tx = Arc::new(tx);
        Channel { rx, tx }
    }
}

macro_rules! lazy_channel {
    () => {
        std::sync::LazyLock::new(|| Default::default())
    };
}

use bevy::time::TrySendError;
pub(crate) use lazy_channel;
