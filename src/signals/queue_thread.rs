use std::{
    collections::{HashMap, VecDeque},
    sync::{atomic::AtomicU32, Arc},
    thread::Thread,
    time::Duration,
};

use flume::{Receiver, Sender};
use frclib_core::time::Instant;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::devices::DeviceIdentifier;

use super::{native, SignalSource, SignalValueRaw};

static QUEUE_THREAD_MANAGER: Lazy<Mutex<QueueThreadManager>> = Lazy::new(|| {
    Mutex::new(QueueThreadManager {
        threads: Vec::new(),
        signal_cache: HashMap::new(),
    })
});

struct SignalCacheEntry {
    count: u8,
    thread_index: usize,
    receiver: Receiver<SignalValueRaw>,
}

struct QueueThreadManager {
    threads: Vec<QueueThread>,
    signal_cache: HashMap<SignalSource, SignalCacheEntry>,
}

struct QueueThread {
    thread: Thread,
    sender: Sender<QueueThreadMessage>,
    signal_count: Arc<AtomicU32>,
}

#[derive(Debug)]
enum QueueThreadMessage {
    NewSignal {
        source: SignalSource,
        channel: Sender<SignalValueRaw>,
        delay: f64,
    },
    DropSignal {
        source: SignalSource,
    },
    UpdateDelay {
        source: SignalSource,
        delay: f64,
    },
}

impl QueueThread {
    #[allow(clippy::comparison_chain)]
    fn new() -> Self {
        let (sender, receiver) = flume::unbounded();
        let signal_count_out = Arc::new(AtomicU32::new(0));
        let signal_count = signal_count_out.clone();
        let thread = std::thread::spawn(move || {
            let mut signals: HashMap<
                SignalSource,
                (Sender<SignalValueRaw>, f64, DeviceIdentifier),
            > = HashMap::new();
            let mut checks: VecDeque<(SignalSource, Instant)> = VecDeque::new();
            loop {
                if signals.is_empty() {
                    std::thread::park();
                }
                if checks.len() > 1 {
                    let poll = checks.pop_front().unwrap();
                    let next = checks.front().unwrap();
                    let wait_duration = next
                        .1
                        .checked_duration_since(Instant::now())
                        .unwrap_or_else(|| Duration::from_secs(0));
                    if let Ok(ret) = native::request_signal_value_single(
                        native::SignalMeta {
                            can_bus: signals[&poll.0].2.canbus.clone(),
                            timeout: wait_duration.as_secs_f64(),
                        },
                        poll.0,
                    ) {
                        signals[&poll.0].0.send(ret).unwrap();
                        checks.push_back((
                            poll.0,
                            Instant::now()
                                .checked_add(Duration::from_secs_f64(signals[&poll.0].1))
                                .unwrap(),
                        ));
                    }
                } else if checks.len() == 1 {
                    let poll = checks.pop_front().unwrap();
                    if let Ok(ret) = native::request_signal_value_single(
                        native::SignalMeta {
                            can_bus: signals[&poll.0].2.canbus.clone(),
                            timeout: Duration::from_secs(0).as_secs_f64(),
                        },
                        poll.0,
                    ) {
                        signals[&poll.0].0.send(ret).unwrap();
                        checks.push_back((
                            poll.0,
                            Instant::now()
                                .checked_add(Duration::from_secs_f64(signals[&poll.0].1))
                                .unwrap(),
                        ));
                    }
                }
                if !checks.is_empty() && receiver.is_empty() {
                    continue;
                }
                match receiver.recv() {
                    Ok(QueueThreadMessage::NewSignal {
                        source,
                        channel,
                        delay,
                    }) => {
                        if let Some(id) = DeviceIdentifier::from_hash(source.hash) {
                            signals.insert(source, (channel, delay, id));
                            signal_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                            //add to checks
                            let now = Instant::now();
                            checks.push_back((
                                source,
                                now.checked_add(Duration::from_secs_f64(delay)).unwrap(),
                            ));
                        }
                    }
                    Ok(QueueThreadMessage::DropSignal { source }) => {
                        if (signals.remove(&source)).is_some() {
                            signal_count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);

                            //remove from checks
                            checks.retain(|(s, _)| s != &source);
                        }
                    }
                    Ok(QueueThreadMessage::UpdateDelay { source, delay }) => {
                        if let Some((_, d, _)) = signals.get_mut(&source) {
                            *d = delay;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        Self {
            thread: thread.thread().clone(),
            sender,
            signal_count: signal_count_out,
        }
    }

    fn new_signal(&self, source: SignalSource, channel: Sender<SignalValueRaw>, delay: f64) {
        self.sender
            .send(QueueThreadMessage::NewSignal {
                source,
                channel,
                delay,
            })
            .unwrap();
        if self.signal_count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
            self.thread.unpark();
        }
    }

    fn drop_signal(&self, source: SignalSource) {
        self.sender
            .send(QueueThreadMessage::DropSignal { source })
            .unwrap();
    }

    fn update_delay(&self, source: SignalSource, delay: f64) {
        self.sender
            .send(QueueThreadMessage::UpdateDelay { source, delay })
            .unwrap();
    }
}
