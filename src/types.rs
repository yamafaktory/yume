use crate::io::Line;
use async_std::sync::{Receiver, Sender};
use std::sync::Arc;

pub type SenderReceiver = Arc<(Sender<Option<Line>>, Receiver<Option<Line>>)>;
