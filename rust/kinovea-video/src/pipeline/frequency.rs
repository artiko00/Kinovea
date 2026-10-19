//! Port of `Kinovea.Services/Diagnostics/FrequencyCounter.cs` and `Kinovea.Services/Perfs/Averager.cs`.
//! The pipeline always smooths with the moving average, so the C# flag is not ported.

use std::time::Instant;

/// Frequency of a signal, measured on blocks of `window` intervals once `ignored_intervals`
/// intervals have passed, then smoothed with an exponential moving average (alpha 0.1).
#[derive(Debug)]
pub struct FrequencyCounter {
    window: u64,
    ignored_intervals: u64,
    last_tick: Option<Instant>,
    received: u64,
    running_total: f64,
    frequency: f64,
    averager: Averager,
}

impl FrequencyCounter {
    pub fn new(window: u64, ignored_intervals: u64) -> Self {
        Self {
            window,
            ignored_intervals,
            last_tick: None,
            received: 0,
            running_total: 0.0,
            frequency: 0.0,
            averager: Averager::new(0.1),
        }
    }

    /// Frequency in hertz, zero until the first block is complete.
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Registers one arrival. The first arrival after construction or `reset` only starts the clock.
    pub fn tick_at(&mut self, now: Instant) {
        self.received += 1;
        let Some(last) = self.last_tick.replace(now) else {
            return;
        };

        let received_intervals = self.received - 1;
        if received_intervals <= self.ignored_intervals {
            return;
        }

        let counted_intervals = received_intervals - self.ignored_intervals;
        self.running_total += now.duration_since(last).as_secs_f64();
        if !counted_intervals.is_multiple_of(self.window) {
            return;
        }

        let average = self.running_total / self.window as f64;
        self.averager.post(average);
        self.frequency = 1.0 / self.averager.average();
        self.running_total = 0.0;
    }

    /// Restarts counting. The averager keeps its history, as in C#.
    pub fn reset(&mut self) {
        self.received = 0;
        self.frequency = 0.0;
        self.running_total = 0.0;
        self.last_tick = None;
    }
}

/// Exponential moving average. The first sample becomes the average.
#[derive(Debug)]
struct Averager {
    alpha: f64,
    average: Option<f64>,
}

impl Averager {
    fn new(alpha: f64) -> Self {
        Self {
            alpha,
            average: None,
        }
    }

    fn post(&mut self, value: f64) {
        self.average = Some(match self.average {
            None => value,
            Some(average) => value * self.alpha + average * (1.0 - self.alpha),
        });
    }

    fn average(&self) -> f64 {
        self.average.unwrap_or(0.0)
    }
}
