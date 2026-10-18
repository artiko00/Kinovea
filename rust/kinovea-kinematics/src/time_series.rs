//! Sample times plus one `f64` series per kinematic component (`TimeSeriesCollection`), and
//! the NaN padder used around central differences.

use crate::component::Kinematics;
use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TimeSeriesCollection {
    length: usize,
    pub times: Vec<i64>,
    components: BTreeMap<Kinematics, Vec<f64>>,
}

impl TimeSeriesCollection {
    pub fn new(length: usize) -> Self {
        TimeSeriesCollection {
            length,
            times: vec![0; length],
            components: BTreeMap::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn set_times(&mut self, times: Vec<i64>) {
        self.times = times;
    }

    /// Adds or replaces a series.
    pub fn add_component(&mut self, key: Kinematics, series: Vec<f64>) {
        self.components.insert(key, series);
    }

    /// Creates zero-filled series for `keys`.
    pub fn initialize_components(&mut self, keys: &[Kinematics]) {
        for k in keys {
            self.components.insert(*k, vec![0.0; self.length]);
        }
    }

    pub fn get(&self, key: Kinematics) -> Option<&[f64]> {
        self.components.get(&key).map(Vec::as_slice)
    }

    pub fn contains(&self, key: Kinematics) -> bool {
        self.components.contains_key(&key)
    }

    pub fn keys(&self) -> impl Iterator<Item = Kinematics> + '_ {
        self.components.keys().copied()
    }
}

impl Index<Kinematics> for TimeSeriesCollection {
    type Output = Vec<f64>;
    /// Panics when the component is absent, as the original throws.
    fn index(&self, key: Kinematics) -> &Vec<f64> {
        self.components.get(&key).unwrap_or_else(|| panic!("component {key} not present"))
    }
}

impl IndexMut<Kinematics> for TimeSeriesCollection {
    fn index_mut(&mut self, key: Kinematics) -> &mut Vec<f64> {
        self.components.get_mut(&key).unwrap_or_else(|| panic!("component {key} not present"))
    }
}

/// Sets `padding` values at each end to NaN; the whole series when it is not longer than
/// twice the padding (`TimeSeriesPadder.Pad`).
pub fn pad(values: &mut [f64], padding: usize) {
    let n = values.len();
    if n <= padding * 2 {
        values.fill(f64::NAN);
        return;
    }
    for i in 0..padding {
        values[i] = f64::NAN;
        values[n - 1 - i] = f64::NAN;
    }
}
