//! Seeded generators for the time-based drawings: stopwatches, multi-stopwatches, counters and tracks.

use super::gen::{drawing, leaf, node, Rng, MEASURES, TEXTS};
use super::parts::style;
use kinovea_core::preferences::dom::Element;

pub fn chrono(r: &mut Rng) -> Element {
    let values = node(
        "Values",
        vec![
            leaf("Visible", r.int(0, 9000)),
            leaf("Invisible", r.int(-1, 9000)),
            leaf("StartCounting", r.int(0, 9000)),
            leaf("StopCounting", r.int(0, 9000)),
            leaf("ClockOrigin", r.int(-1, 9000)),
        ],
    );
    let label = node(
        "Label",
        vec![leaf("Text", r.pick(&TEXTS)), leaf("Show", r.flag())],
    );
    let c = vec![leaf("Position", r.point()), values, label, style(r)];
    drawing("Chrono", r, c)
}

fn section(r: &mut Rng, name: &str, tag: &str) -> Element {
    let mut e = leaf("Section", format!("{};{}", r.int(0, 9000), r.int(-1, 9000)));
    e.attributes = vec![("name".into(), name.into()), ("tag".into(), tag.into())];
    e
}

pub fn chrono_multi(r: &mut Rng) -> Element {
    let sections = node(
        "Sections",
        vec![section(r, "first", "a"), section(r, "second", "b")],
    );
    let values = node(
        "Values",
        vec![
            leaf("Visible", r.int(0, 9000)),
            leaf("Invisible", r.int(-1, 9000)),
            sections,
            leaf("Locked", r.flag()),
            leaf("VisibleColumns", "0;1"),
        ],
    );
    let label = node("Label", vec![leaf("Show", r.flag())]);
    let c = vec![leaf("Position", r.point()), values, label, style(r)];
    drawing("ChronoMulti", r, c)
}

pub fn counter(r: &mut Rng) -> Element {
    let beats = node(
        "Beats",
        (0..3).map(|_| leaf("Beat", r.int(0, 9000))).collect(),
    );
    let values = node(
        "Values",
        vec![
            leaf("Visible", r.int(0, 9000)),
            leaf("Invisible", r.int(-1, 9000)),
            beats,
            leaf("Locked", r.flag()),
            leaf("ZeroBased", r.flag()),
            leaf("HalfCadence", r.flag()),
            leaf("DoubleCadence", r.flag()),
            leaf("ExtraData", r.pick(&MEASURES)),
        ],
    );
    let c = vec![leaf("Position", r.point()), values, style(r)];
    drawing("Counter", r, c)
}

pub fn track(r: &mut Rng) -> Element {
    let points: Vec<Element> = (0..3i64)
        .map(|i| {
            leaf(
                "TrackPoint",
                format!("{};{};{}", r.coord(), r.coord(), 400 + 40 * i),
            )
        })
        .collect();
    let mut main = node(
        "MainLabel",
        vec![
            leaf("SpacePosition", r.point()),
            leaf("TimePosition", r.int(0, 9000)),
        ],
    );
    main.attributes = vec![("Text".into(), "Ball".into())];
    let c = vec![
        leaf("TimePosition", r.int(0, 9000)),
        leaf("IsVisible", r.flag()),
        leaf("Visible", r.int(0, 9000)),
        leaf("Invisible", r.int(-1, 9000)),
        leaf("ExtraData", r.pick(&MEASURES)),
        node("TrackPointList", points),
        style(r),
        main,
        leaf("SeeFuture", r.flag()),
        leaf("ShowTrackLabel", r.flag()),
        leaf("ShowKeyframeLabels", r.flag()),
        leaf("UseKeyframeColors", r.flag()),
        leaf("IsInteractiveTrack", r.flag()),
    ];
    drawing("Track", r, c)
}
