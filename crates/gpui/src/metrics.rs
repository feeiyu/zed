//! metrics
use core::fmt;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

thread_local! {
    pub(crate) static METRICS: RefCell<Metrics> = RefCell::new(Metrics::new());
}

const METRICS_COUNT: usize = 32;

#[derive(Clone, Copy)]
struct Item {
    count: u64,
    cost: u128,
}

impl Default for Item {
    fn default() -> Self {
        Self { count: 0, cost: 0 }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "count: {}, cost: {}", self.count, self.cost)
    }
}

struct Metrics {
    items: [Item; METRICS_COUNT],
}

impl Metrics {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            items: [Default::default(); METRICS_COUNT],
        }
    }

    #[allow(missing_docs)]
    pub fn clear_all(&mut self) {
        self.items = [Default::default(); METRICS_COUNT];
    }

    #[allow(missing_docs)]
    pub fn record(&mut self, index: usize, cost: u128) {
        let item = &mut self.items[index];
        item.count += 1;
        item.cost += cost;
    }

    pub fn print_all(&self) {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.count > 0)
            .for_each(|(index, item)| log::error!("index: {}, {}", index, item));
    }
}

#[allow(missing_docs)]
pub struct MetricsRecorder {
    index: usize,
    start: Instant,
}

impl Drop for MetricsRecorder {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        let index = self.index;
        let cost = elapsed.as_nanos();
        METRICS.with_borrow_mut(|metrics| metrics.record(index, cost));
    }
}

impl MetricsRecorder {
    #[allow(missing_docs)]
    pub fn new(index: usize) -> Self {
        Self {
            index,
            start: Instant::now(),
        }
    }
}


#[allow(missing_docs)]
pub fn clear_all_metrics() {
    METRICS.with_borrow_mut(|metrics| metrics.clear_all());
}

#[allow(missing_docs)]
pub fn print_all_metrics() {
    METRICS.with_borrow(|metrics| metrics.print_all())
}
