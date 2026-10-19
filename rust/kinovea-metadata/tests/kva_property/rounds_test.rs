//! The idempotence check must reject a read/write pair that is not idempotent.

use super::drawings_a::line;
use super::rounds::check;

#[test]
#[should_panic(expected = "seed 0")]
fn check_rejects_a_round_that_changes_on_every_pass() {
    check(line, &|xml| format!("{xml}x"));
}
