//! Replays the `videocache` op script of `Kinovea.FixtureGen/Areas/VideoCacheArea.Ops.cs` on the
//! Rust containers and records the five `steps.csv` columns: op, result, current, snapshot, frames.

#[cfg(test)]
mod mod_test;
mod ops;
#[cfg(test)]
mod ops_test;

use kinovea_video::cache::Cache;
use kinovea_video::container::FramesContainer;
use kinovea_video::prebuffer::{PreBuffer, PreBufferConfig};
use serde_json::Value;

/// One `steps.csv` row.
pub type Row = [String; 5];

enum Container {
    Dense(Cache),
    Pre(PreBuffer),
}

/// Container built from the case inputs, plus the timestamp step of the generated frames.
struct Session {
    container: Container,
    ts_per_frame: i64,
}

/// Replays every op of the case and returns one row per op.
pub fn replay(inputs: &Value) -> Vec<Row> {
    let mut session = Session::new(inputs);
    inputs["ops"]
        .as_array()
        .expect("ops")
        .iter()
        .map(|op| {
            let op = op.as_str().expect("op").to_owned();
            let result = session.run(&op);
            [
                op,
                result,
                session.current(),
                session.snapshot(),
                session.frames_column(),
            ]
        })
        .collect()
}

impl Session {
    fn new(inputs: &Value) -> Self {
        let container = if inputs["container"].as_str().unwrap_or("cache") == "cache" {
            Container::Dense(Cache::new())
        } else {
            Container::Pre(PreBuffer::new(PreBufferConfig {
                capacity: inputs["capacity"].as_u64().unwrap_or(32) as usize,
                retention: inputs["retention"].as_u64().unwrap_or(8) as usize,
                tolerance: inputs["tolerance"].as_f64().unwrap_or(0.0),
                far_ahead: inputs["farAhead"].as_f64().unwrap_or(0.0),
            }))
        };
        Self {
            container,
            ts_per_frame: inputs["tsPerFrame"].as_i64().unwrap_or(1),
        }
    }

    /// Runs one op. A failing op yields `exception:<name>`, as the C# `Run` records it.
    fn run(&mut self, op: &str) -> String {
        let args: Vec<&str> = op.split(' ').collect();
        self.dispatch(&args)
            .unwrap_or_else(|exception| format!("exception:{exception}"))
    }

    fn store(&self) -> &dyn FramesContainer {
        match &self.container {
            Container::Dense(cache) => cache,
            Container::Pre(pb) => pb,
        }
    }

    fn store_mut(&mut self) -> &mut dyn FramesContainer {
        match &mut self.container {
            Container::Dense(cache) => cache,
            Container::Pre(pb) => pb,
        }
    }

    fn current(&self) -> String {
        match self.store().current() {
            Some(frame) => frame.timestamp.to_string(),
            None => "-1".to_owned(),
        }
    }

    /// `version|[a;b] [c;d]`, with the trailing space trimmed as in the C# output.
    fn snapshot(&self) -> String {
        let snapshot = self.store().snapshot();
        format!("{}|{}", snapshot.version, snapshot.to_string().trim_end())
    }

    fn frames_column(&self) -> String {
        let stamps: Vec<String> = self
            .store()
            .timestamps()
            .iter()
            .map(i64::to_string)
            .collect();
        stamps.join(" ")
    }
}
