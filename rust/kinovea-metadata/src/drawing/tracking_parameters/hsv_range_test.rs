use crate::drawing::tracking_parameters::HsvRange;

#[test]
fn text_form_round_trips() {
    let range = HsvRange {
        hue: (10.5, 170.0),
        saturation: (0.0, 255.0),
        value: (30.0, 200.0),
    };
    assert_eq!(HsvRange::parse(&range.write()), range);
    assert_eq!(range.write(), "10.5;170;0;255;30;200");
}

#[test]
fn content_hash_depends_on_every_bound() {
    let hash = HsvRange::default().content_hash();
    let edits: [fn(&mut HsvRange); 6] = [
        |r| r.hue.0 = 1.0,
        |r| r.hue.1 = 1.0,
        |r| r.saturation.0 = 1.0,
        |r| r.saturation.1 = 1.0,
        |r| r.value.0 = 1.0,
        |r| r.value.1 = 1.0,
    ];
    for edit in edits {
        let mut range = HsvRange::default();
        edit(&mut range);
        assert_ne!(range.content_hash(), hash);
    }
}

#[test]
fn an_unreadable_part_keeps_the_bounds_read_before_it() {
    // HSVRange.cs:83-88: the hue min is set, then the hue max fails and the rest keep defaults.
    assert_eq!(
        HsvRange::parse("1;x;3;4;5;6"),
        HsvRange {
            hue: (1.0, 360.0),
            saturation: (0.0, 255.0),
            value: (0.0, 255.0)
        }
    );
}

#[test]
fn parts_after_the_sixth_are_ignored() {
    assert_eq!(HsvRange::parse("1;2;3;4;5;6;7").value, (5.0, 6.0));
}
