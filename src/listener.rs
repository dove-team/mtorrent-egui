use mtorrent::utils::listener::{StateListener, StateSnapshot};
use std::{future::Future, time::Duration};
use tokio::sync::oneshot;

struct DumpSnapshot {
    level: log::Level,
    ticks: usize,
}

pub struct Listener {
    callback: Box<dyn Fn(&StateSnapshot<'_>) + Send>,
    dump_cfg: Option<DumpSnapshot>,
}

pub type Canceller = oneshot::Sender<()>;

pub fn listener_with_canceller<F>(
    callback: F,
    dump_level: log::Level,
) -> (Listener, Canceller, impl Future<Output = ()>)
where
    F: Fn(&StateSnapshot<'_>) + Send + 'static,
{
    let (canceller, cancelled) = oneshot::channel();
    let listener = Listener {
        callback: Box::new(callback),
        dump_cfg: log::log_enabled!(dump_level).then_some(DumpSnapshot {
            level: dump_level,
            ticks: 0,
        }),
    };
    (listener, canceller, async {
        let _ = cancelled.await;
    })
}

impl StateListener for Listener {
    const INTERVAL: Duration = Duration::from_secs(1);

    fn on_snapshot(&mut self, snapshot: StateSnapshot<'_>) {
        if let Some(dump) = &mut self.dump_cfg {
            // dump snapshot every 10s
            dump.ticks = dump.ticks.wrapping_add(1);
            if dump.ticks.is_multiple_of(10) {
                log::log!(dump.level, "{snapshot}");
            }
        }
        (self.callback)(&snapshot);
    }
}
