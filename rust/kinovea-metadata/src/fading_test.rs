use crate::fading::Fading;
use kinovea_core::preferences::dom::Element;

fn fading(reference: i64, atpf: f64) -> Fading {
    Fading {
        use_default: false,
        reference_timestamp: reference,
        average_timestamps_per_frame: atpf,
        fading_frames: 20,
        opaque_frames: 1,
        ..Fading::default()
    }
}

#[test]
fn opacity_is_one_in_the_opaque_window_and_fades_linearly() {
    let f = fading(1000, 10.0);
    let d = Fading::default();
    assert_eq!(f.opacity_factor(1000, &d), 1.0);
    assert!((f.opacity_factor(1100, &d) - 0.5).abs() < 1e-9);
    assert!((f.opacity_factor(900, &d) - 0.5).abs() < 1e-9);
    assert_eq!(f.opacity_factor(1200, &d), 0.0);
    assert_eq!(f.opacity_factor(5000, &d), 0.0);
    let mut wide = fading(1000, 10.0);
    wide.opaque_frames = 5;
    assert_eq!(wide.opacity_factor(1040, &d), 1.0);
    assert!((wide.opacity_factor(1140, &d) - 0.5).abs() < 1e-9);
    let mut half = fading(1000, 10.0);
    half.master_factor = 0.5;
    assert_eq!(half.opacity_factor(1000, &d), 0.5);
}

#[test]
fn always_visible_and_use_default_override() {
    let mut f = fading(1000, 10.0);
    f.always_visible = true;
    assert_eq!(f.opacity_factor(99_999, &Fading::default()), 1.0);
    let d = Fading {
        always_visible: true,
        ..Fading::default()
    };
    let mut g = fading(1000, 10.0);
    g.use_default = true;
    assert_eq!(g.opacity_factor(99_999, &d), 1.0);
    assert!((g.opacity_trackable(100, 99_999, &d) - 1.0).abs() < 1e-9);
    assert!((fading(1000, 10.0).opacity_trackable(100, 5000, &Fading::default()) - 0.5).abs() < 1e-9);
}

#[test]
fn kva_round_trip_and_minimum_frames() {
    let f = Fading {
        enabled: true,
        use_default: false,
        always_visible: false,
        fading_frames: 80,
        opaque_frames: 1,
        master_factor: 1.0,
        ..Fading::default()
    };
    let mut e = Element::new("InfosFading");
    e.children = f.write_kva();
    assert_eq!(e.children.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(), ["Enabled", "UseDefault", "AlwaysVisible", "MasterFactor", "OpaqueFrames", "Frames"]);
    assert_eq!(e.child_text("Frames"), Some("80"));
    assert_eq!(e.child_text("MasterFactor"), Some("1"));
    let mut back = Fading::default();
    back.read_kva(&e);
    assert_eq!(back, f);
    let mut zero = Element::new("InfosFading");
    zero.push_leaf("Frames", "0").push_leaf("Unknown", "x");
    back.read_kva(&zero);
    assert_eq!(back.fading_frames, 1);
    assert_eq!(f.content_hash(), f.clone().content_hash());
    assert_ne!(f.content_hash(), Fading::default().content_hash());
}

/// Oracle parity of `GetOpacityFactor` over the fixture grid.
#[test]
fn opacity_matches_the_oracle_table() {
    let csv = std::fs::read_to_string(kinovea_fixtures::fixtures_root().join("kva/fading/opacity.csv")).unwrap();
    let mut rows = 0;
    for line in csv.lines().skip(1) {
        let f: Vec<&str> = line.split(',').collect();
        let fading = Fading {
            enabled: true,
            use_default: false,
            always_visible: f[0] == "1",
            opaque_frames: f[1].parse().unwrap(),
            fading_frames: f[2].parse().unwrap(),
            master_factor: f[3].parse().unwrap(),
            average_timestamps_per_frame: f[4].parse().unwrap(),
            reference_timestamp: f[5].parse().unwrap(),
        };
        let got = fading.opacity_factor(f[6].parse().unwrap(), &Fading::default());
        kinovea_fixtures::assert_close(got, f[7].parse().unwrap(), 1e-9, line);
        rows += 1;
    }
    assert!(rows > 600, "{rows}");
}
