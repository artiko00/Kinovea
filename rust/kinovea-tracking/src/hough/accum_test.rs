use super::{centers, vote, Accumulator};
use crate::hough::gradient::Gradient;

#[test]
fn each_radius_step_votes_once_in_both_gradient_directions() {
    let mut g = Gradient {
        width: 9,
        height: 9,
        dx: vec![0; 81],
        dy: vec![0; 81],
    };
    g.dx[4 * 9 + 4] = 3;
    let mut edges = vec![false; 81];
    edges[4 * 9 + 4] = true;
    let acc = vote(&g, &edges, 1.0, 2, 3);
    assert_eq!(acc.votes.iter().sum::<i32>(), 4);
    for (x, y) in [(6, 4), (7, 4), (2, 4), (1, 4)] {
        assert_eq!(acc.at(x, y), 1);
    }
}

#[test]
fn walk_stops_at_the_accumulator_border() {
    let mut g = Gradient {
        width: 9,
        height: 9,
        dx: vec![0; 81],
        dy: vec![0; 81],
    };
    g.dx[4 * 9] = 3;
    let mut edges = vec![false; 81];
    edges[4 * 9] = true;
    let acc = vote(&g, &edges, 1.0, 1, 3);
    assert_eq!(acc.votes.iter().sum::<i32>(), 3);
}

#[test]
fn centers_are_strict_maxima_above_threshold_most_voted_first() {
    let mut acc = Accumulator {
        cols: 6,
        rows: 4,
        votes: vec![0; 24],
    };
    acc.votes[6 + 1] = 5;
    acc.votes[6 + 2] = 5;
    acc.votes[2 * 6 + 4] = 9;
    assert_eq!(centers(&acc, 5), vec![(4, 2)]);
    assert_eq!(centers(&acc, 4), vec![(4, 2), (1, 1)]);
}
