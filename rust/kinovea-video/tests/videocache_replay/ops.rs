//! The op interpreter of `VideoCacheArea.Ops.cs`: one arm per op name. Ops that the C# container
//! would reject with an exception return the exception name, as the C# `Run` records it.

use super::{Container, Session};
use kinovea_video::cache::Cache;
use kinovea_video::container::{CacheAddResult, FramesContainer};
use kinovea_video::frame::VideoFrame;
use kinovea_video::prebuffer::PreBuffer;
use kinovea_video::state::{PlayerAction, PlayerState};

impl Session {
    pub(super) fn dispatch(&mut self, op: &[&str]) -> Result<String, &'static str> {
        let ts_per_frame = self.ts_per_frame;
        match op[0] {
            "add" => self.add(frame_from(op, ts_per_frame), false),
            "forceAdd" => self.add(frame_from(op, ts_per_frame), true),
            "acquireClosest" => {
                self.store_mut().acquire_closest(num(op[1]));
                Ok(String::new())
            }
            "acquireNext" => Ok(csharp_bool(self.store_mut().acquire_next())),
            "clear" => {
                self.store_mut().clear();
                Ok(String::new())
            }
            // The C# methods throw. The Rust dense cache clears on shutdown (design D3), so the
            // replay reports the original's row and leaves the state untouched.
            "shutdown" => match &mut self.container {
                Container::Dense(_) => Err("NotImplementedException"),
                Container::Pre(pb) => {
                    pb.shutdown();
                    Ok(String::new())
                }
            },
            "trim" => {
                self.dense()?.trim(num(op[1]), num(op[2]));
                Ok(String::new())
            }
            "workingZone" => {
                let zone = self.dense()?.working_zone();
                Ok(format!("{};{}", zone.start, zone.end))
            }
            "tryAcquire" => {
                let action: PlayerAction = op[1].parse().map_err(|_| "ArgumentException")?;
                let request = PlayerState::new(0, action, false, num(op[2]), 0.0, 0.0);
                let result = self.pre()?.try_acquire(&request);
                Ok(format!(
                    "{};{};{}",
                    csharp_bool(result.target_acquired),
                    result.acquired_timestamp,
                    csharp_bool(result.is_contiguous)
                ))
            }
            "relate" => Ok(self.pre()?.relate_timestamp(num(op[1])).name().to_owned()),
            "contiguous" => Ok(csharp_bool(self.pre()?.is_contiguous())),
            "purge" => {
                self.pre()?.purge();
                Ok(String::new())
            }
            "interrupt" => {
                self.pre()?.interrupt_add();
                Ok(String::new())
            }
            "resetInterrupt" => {
                self.pre()?.reset_interrupt_add();
                Ok(String::new())
            }
            _ => Err("ArgumentException"),
        }
    }

    /// `add` blocks in C#. Here the non-blocking form reports `Blocked` when the add would wait.
    /// That path changes nothing, so the outcome matches without a thread or a timeout.
    fn add(&mut self, frame: VideoFrame, forced: bool) -> Result<String, &'static str> {
        let result = match &mut self.container {
            Container::Dense(cache) => Some(cache.add(frame)),
            Container::Pre(pb) if forced => Some(pb.force_add(frame)),
            Container::Pre(pb) => pb.try_add(frame),
        };
        Ok(result.map_or("Blocked", CacheAddResult::name).to_owned())
    }

    /// The dense cache's own operations. The prebuffer case throws NullReferenceException in C#.
    fn dense(&mut self) -> Result<&mut Cache, &'static str> {
        match &mut self.container {
            Container::Dense(cache) => Ok(cache),
            Container::Pre(_) => Err("NullReferenceException"),
        }
    }

    /// The prebuffer's own operations. The dense case throws NullReferenceException in C#.
    fn pre(&mut self) -> Result<&mut PreBuffer, &'static str> {
        match &mut self.container {
            Container::Pre(pb) => Ok(pb),
            Container::Dense(_) => Err("NullReferenceException"),
        }
    }
}

/// `add t [previous]`: the previous timestamp defaults to `t - tsPerFrame`.
fn frame_from(op: &[&str], ts_per_frame: i64) -> VideoFrame {
    let t = num(op[1]);
    let previous = op.get(2).map_or(t - ts_per_frame, |p| num(p));
    VideoFrame::new(None, t, previous)
}

fn num(text: &str) -> i64 {
    text.parse().expect("integer argument")
}

fn csharp_bool(value: bool) -> String {
    if value { "True" } else { "False" }.to_owned()
}
