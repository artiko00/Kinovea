//! Parity of every per-drawing hit test with the oracle probe tables (`tests/fixtures/render/<case>/`).
//!
//! Rule, per drawing and scale, over the box of the drawing (`hits/grid.rs`):
//! - at least 99 percent of the probes in the box agree with the oracle (a probe not listed is a miss);
//! - every disagreement lies within 2 px of a change in the Rust answer.
//!
//! The exemptions are listed with their cause in `hits/rule.rs`, and `hits/check.rs` checks that none of
//! them is stale. A drawing with no compared scale fails unless it is an expected exception.

#[path = "hits/cases.rs"]
mod cases;
#[path = "hits/check.rs"]
mod check;
#[path = "hits/grid.rs"]
mod grid;
#[path = "hits/load.rs"]
mod load;
#[path = "hits/rule.rs"]
mod rule;
#[path = "hits/run.rs"]
mod run;
#[path = "hits/table.rs"]
mod table;
#[path = "hits/targets.rs"]
mod targets;

#[test]
fn hit_tables_match_the_oracle_probe_grid() {
    let evaluation = run::evaluate();
    let mut failures = evaluation.problems.clone();
    failures.extend(check::violations(&evaluation.judged));
    assert!(
        failures.is_empty(),
        "hit rule violated:\n{}",
        failures.join("\n")
    );
}
