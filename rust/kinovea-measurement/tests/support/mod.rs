//! Fixture readers shared by the calibration parity tests.

// Each integration test compiles this module separately and uses a subset of it.
#![allow(dead_code)]

pub mod calibrations;
#[cfg(test)]
mod calibrations_test;
mod mod_test;

use kinovea_fixtures::fixtures_root;
use kinovea_geometry::{PointF, Quadrilateral};
use std::path::PathBuf;

pub fn fixture(path: &str) -> PathBuf {
    fixtures_root().join("calibration").join(path)
}

pub fn case(path: &str) -> PathBuf {
    fixtures_root().join("../cases/calibration").join(path)
}

/// Header names and rows of a fixture CSV.
pub fn table(path: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let full = fixture(path);
    let text = std::fs::read_to_string(&full).unwrap_or_else(|e| panic!("{}: {e}", full.display()));
    let mut lines = text.lines().filter(|l| !l.is_empty());
    let header = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::to_owned)
        .collect();
    let rows = lines
        .map(|l| l.split(',').map(str::to_owned).collect())
        .collect();
    (header, rows)
}

/// Column accessor by header name.
pub struct Row<'a> {
    pub header: &'a [String],
    pub cells: &'a [String],
}

impl Row<'_> {
    pub fn text(&self, name: &str) -> &str {
        let i = self
            .header
            .iter()
            .position(|h| h == name)
            .unwrap_or_else(|| panic!("{name} missing"));
        &self.cells[i]
    }

    pub fn f(&self, name: &str) -> f64 {
        self.text(name)
            .parse()
            .unwrap_or_else(|e| panic!("{name}: {e}"))
    }

    pub fn point(&self, x: &str, y: &str) -> PointF {
        PointF::new(self.f(x) as f32, self.f(y) as f32)
    }

    pub fn quad(&self, prefix: &str) -> Quadrilateral {
        let v = |i: usize| self.point(&format!("{prefix}{i}x"), &format!("{prefix}{i}y"));
        Quadrilateral::new(v(0), v(1), v(2), v(3))
    }
}

pub fn assert_f(got: f64, want: f64, tol: f64, ctx: &str) {
    kinovea_fixtures::assert_close(got, want, tol, ctx);
}

pub fn assert_pt(got: PointF, want: PointF, tol: f64, ctx: &str) {
    assert_f(
        f64::from(got.x),
        f64::from(want.x),
        tol,
        &format!("{ctx} x"),
    );
    assert_f(
        f64::from(got.y),
        f64::from(want.y),
        tol,
        &format!("{ctx} y"),
    );
}
