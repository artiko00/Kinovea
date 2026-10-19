#[test]
fn style_module_is_declared() {
    assert!(module_path!().contains("style"));
}

mod resolving {
    use crate::renderer::{FontFamily, LineCap, LineJoin};
    use crate::style::*;
    use kinovea_metadata::style::{DashStyle, LineShape, PenShape, TrackShape};
    use kinovea_metadata::{Color, StyleElements, StyleValue};

    fn style_with_line(size: i32) -> StyleData {
        let elements = StyleElements::new(vec![
            ("line color", StyleValue::Color(Color::rgb(10, 20, 30))),
            ("line size", StyleValue::LineSize(size)),
        ]);
        StyleData::from_elements(&elements)
    }

    // StyleData.cs:126 LineSize and DrawingTrack.cs:2377-2378: `line size` feeds LineSize, while the
    // track's `TrackPointSize` pen is bound to its own property, whatever the element order.
    #[test]
    fn track_point_pen_does_not_override_line_size() {
        for order in [0, 1] {
            let line = ("line size", StyleValue::LineSize(6));
            let point = ("TrackPointSize", StyleValue::PenSize(3));
            let items = if order == 0 {
                vec![line.clone(), point.clone()]
            } else {
                vec![point, line]
            };
            let style = StyleData::from_elements(&StyleElements::new(items));
            assert_eq!(style.line_size, 6);
        }
    }

    // DrawingCircle.cs:549 and DrawingPencil.cs:300 bind LineSize to `pen size`, DrawingText.cs:537
    // to `LineSize`: those keys feed the line size.
    #[test]
    fn pen_size_and_legacy_line_size_keys_feed_line_size() {
        let pen = StyleElements::new(vec![("pen size", StyleValue::PenSize(4))]);
        assert_eq!(StyleData::from_elements(&pen).line_size, 4);
        let legacy = StyleElements::new(vec![("LineSize", StyleValue::LineSize(5))]);
        assert_eq!(StyleData::from_elements(&legacy).line_size, 5);
    }

    #[test]
    fn keys_pick_main_and_back_colours_and_kinds_pick_the_rest() {
        let elements = StyleElements::new(vec![
            ("back color", StyleValue::Color(Color::rgb(1, 1, 1))),
            ("color", StyleValue::Color(Color::rgb(2, 2, 2))),
            ("font size", StyleValue::FontSize(18)),
            ("pen size", StyleValue::PenSize(6)),
            ("line shape", StyleValue::LineShape(LineShape::Squiggle)),
            ("pen shape", StyleValue::PenShape(PenShape::Dash)),
            (
                "track shape",
                StyleValue::TrackShape(TrackShape {
                    dash: DashStyle::Dot,
                    steps: false,
                }),
            ),
        ]);
        let style = StyleData::from_elements(&elements);
        assert_eq!(style.color, Color::rgb(2, 2, 2));
        assert_eq!(style.background_color, Color::rgb(1, 1, 1));
        assert_eq!(style.font_size, 18);
        assert_eq!(style.line_size, 6);
        assert_eq!(style.line_shape, LineShape::Squiggle);
        assert_eq!(style.pen_shape, PenShape::Dash);
        assert_eq!(style.track_shape.dash, DashStyle::Dot);
    }

    #[test]
    fn missing_elements_fall_back_to_the_original_defaults() {
        let style = StyleData::from_elements(&StyleElements::default());
        assert_eq!(style.color, Color::BLACK);
        assert_eq!(style.font_size, 12);
        assert_eq!(style.line_size, 1);
        assert_eq!(style.font_family, FontFamily::Arial);
    }

    #[test]
    fn pen_width_is_line_size_scaled_and_clamped_to_one() {
        // DrawingLine: GetPen(alpha, stretch): width = max(lineSize * stretch, 1).
        assert_eq!(style_with_line(4).pen(255, 0.5).width, 2.0);
        assert_eq!(style_with_line(1).pen(255, 0.5).width, 1.0);
    }

    #[test]
    fn pen_takes_alpha_and_round_joins_with_flat_caps() {
        let pen = style_with_line(4).pen(128, 1.0);
        assert_eq!(pen.color, Color::argb(128, 10, 20, 30));
        assert_eq!(
            (pen.start_cap, pen.end_cap, pen.line_join),
            (LineCap::Flat, LineCap::Flat, LineJoin::Round)
        );
        assert_eq!(style_with_line(4).pen(-1, 1.0).color.a, 255);
    }

    #[test]
    fn pen_dash_comes_from_track_shape() {
        let mut style = style_with_line(4);
        style.track_shape = TrackShape {
            dash: DashStyle::Dash,
            steps: false,
        };
        assert_eq!(style.pen(255, 1.0).dash, DashStyle::Dash);
    }

    #[test]
    fn marker_pen_is_one_pixel_with_round_caps() {
        // StyleData.GetPen(int alpha) -> NormalPen: width 1, round caps and joins.
        let pen = style_with_line(9).marker_pen(255);
        assert_eq!(pen.width, 1.0);
        assert_eq!(
            (pen.start_cap, pen.end_cap, pen.line_join),
            (LineCap::Round, LineCap::Round, LineJoin::Round)
        );
    }

    #[test]
    fn font_size_is_points_scaled_then_converted_to_pixels() {
        // Arial 12 pt at 96 DPI is 16 px; the 8 pt floor applies after scaling.
        let style = StyleData::from_elements(&StyleElements::new(vec![(
            "font size",
            StyleValue::FontSize(12),
        )]));
        assert_eq!(style.font(1.0).em_px, 16.0);
        assert_eq!(style.font(2.0).em_px, 32.0);
        assert!((style.font(0.5).em_px - 8.0 * 96.0 / 72.0).abs() < 1e-4);
        assert_eq!(style.font(1.0).family, FontFamily::Arial);
    }

    #[test]
    fn brushes_and_background_colours_follow_the_alpha() {
        let elements = StyleElements::new(vec![
            ("color", StyleValue::Color(Color::rgb(10, 20, 30))),
            ("back color", StyleValue::Color(Color::rgb(250, 250, 250))),
        ]);
        let style = StyleData::from_elements(&elements);
        assert_eq!(style.brush(64).color, Color::argb(64, 10, 20, 30));
        assert_eq!(
            style.background_brush(64).color,
            Color::argb(64, 250, 250, 250)
        );
        assert_eq!(style.foreground_brush(255).color, Color::BLACK);
        assert_eq!(
            style.background_pen(128).color,
            Color::argb(128, 250, 250, 250)
        );
        assert_eq!(style.foreground_pen(255).color, Color::BLACK);
    }
}
