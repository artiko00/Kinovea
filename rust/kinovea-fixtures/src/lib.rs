//! Access to the parity fixtures generated from the original C# Kinovea.
//!
//! Fixtures live in `<repo>/tests/fixtures/<area>/<case>/` and each directory
//! carries a `manifest.json` describing its provenance.

#[cfg(test)]
mod lib_test;

use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Errors raised while locating or reading fixtures.
#[derive(Debug, thiserror::Error)]
pub enum FixtureError {
    /// The fixture file does not exist. Run `tools/oracle/fixtures.sh` to generate it.
    #[error("fixture not found: {0} (run tools/oracle/fixtures.sh)")]
    Missing(PathBuf),
    /// The file exists but could not be read.
    #[error("cannot read {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    /// The manifest is not valid JSON for the expected schema.
    #[error("invalid manifest {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
}

/// Provenance of a fixture directory, written by `Kinovea.FixtureGen`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Manifest {
    /// Git commit of the C# generator that produced the fixture.
    pub commit: String,
    /// FFmpeg version bundled with that build.
    pub ffmpeg_version: String,
    /// SHA-256 (lowercase hex) of the input case file.
    pub case_sha256: String,
}

impl Manifest {
    /// Loads a `manifest.json`.
    pub fn load(path: &Path) -> Result<Self, FixtureError> {
        if !path.exists() {
            return Err(FixtureError::Missing(path.to_path_buf()));
        }
        let text = std::fs::read_to_string(path).map_err(|source| FixtureError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        serde_json::from_str(&text).map_err(|source| FixtureError::Parse {
            path: path.to_path_buf(),
            source,
        })
    }
}

/// Root of the versioned fixtures directory (`<repo>/tests/fixtures`).
pub fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .components()
        .fold(PathBuf::new(), |mut acc, c| {
            if c.as_os_str() == ".." {
                acc.pop();
            } else {
                acc.push(c);
            }
            acc
        })
}

/// Lists every `manifest.json` under the fixtures root, sorted by path.
pub fn manifests(root: &Path) -> Result<Vec<(PathBuf, Manifest)>, FixtureError> {
    let mut out = Vec::new();
    if !root.exists() {
        return Ok(out);
    }
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = std::fs::read_dir(&dir).map_err(|source| FixtureError::Io {
            path: dir.clone(),
            source,
        })?;
        for entry in entries {
            let path = entry
                .map_err(|source| FixtureError::Io {
                    path: dir.clone(),
                    source,
                })?
                .path();
            if path.is_dir() {
                stack.push(path);
            } else if path.file_name().is_some_and(|n| n == "manifest.json") {
                let manifest = Manifest::load(&path)?;
                out.push((path, manifest));
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Absolute-difference comparison. `NaN` never compares equal.
pub fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

/// Panics unless `got` is within `tol` of `want`; a NaN expectation requires a NaN result.
/// `ctx` names the fixture row in the failure message.
pub fn assert_close(got: f64, want: f64, tol: f64, ctx: &str) {
    if want.is_nan() {
        assert!(got.is_nan(), "{ctx}: got {got}, want NaN");
        return;
    }
    assert!(approx_eq(got, want, tol), "{ctx}: got {got}, want {want}");
}

/// Lowercase hex SHA-256 of a byte slice, matching the generator's `case_sha256`.
pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::Digest;
    let digest = sha2::Sha256::digest(bytes);
    digest.iter().fold(String::with_capacity(64), |mut s, b| {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
        s
    })
}
