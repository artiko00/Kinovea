//! The `Drawing` enum: one variant per attached drawing kind with uniform dispatch.

use super::*;

/// Declares the enum, the reader and the dispatch macro from one list of
/// `Variant => "tool display name"` pairs.
macro_rules! kinds {
    ($($v:ident => $display:literal),+ $(,)?) => {
        /// Every attached drawing kind.
        #[derive(Debug, Clone, PartialEq)]
        #[allow(clippy::large_enum_variant)]
        pub enum Drawing {
            $($v($v),)+
        }

        macro_rules! dispatch {
            ($self:expr, |$d:ident| $body:expr) => {
                match $self {
                    $(Drawing::$v($d) => $body,)+
                }
            };
        }

        impl Drawing {
            /// Reads a drawing element by its name; `None` for unknown kinds.
            pub fn read_kva(e: &Element, scale: Scale, map: TimestampMapper) -> Option<Drawing> {
                Some(match e.name.as_str() {
                    $($v::KVA_NAME => Drawing::$v($v::read_kva(e, scale, map)),)+
                    _ => return None,
                })
            }

            fn fixed_display_name(&self) -> &'static str {
                match self {
                    $(Drawing::$v(_) => $display,)+
                }
            }
        }
    };
}

kinds!(
    Line => "Line",
    Angle => "Angle",
    CrossMark => "Marker",
    Circle => "Circle",
    Rectangle => "Rectangle",
    Pencil => "Pencil",
    Polyline => "Polyline",
    TimePolyline => "Trajectory curve",
    Label => "Text",
    Plane => "Perspective grid",
    DistortionGrid => "Distortion grid",
    TimeSegment => "Time segment",
    Chrono => "Stopwatch",
    ChronoMulti => "Advanced stopwatch",
    Counter => "Counter",
    Track => "Trajectory",
    Bitmap => "Bitmap Image",
    Svg => "SVG Image",
    GenericPostureDrawing => "",
);

impl Drawing {
    pub fn kva_name(&self) -> &'static str {
        dispatch!(self, |d| kva_name_of(d))
    }

    pub fn header(&self) -> &Header {
        dispatch!(self, |d| d.header())
    }

    pub fn header_mut(&mut self) -> &mut Header {
        dispatch!(self, |d| d.header_mut())
    }

    pub fn id(&self) -> Uuid {
        self.header().id
    }

    pub fn to_element(&self, filter: Filter, refs: RefValue) -> Element {
        dispatch!(self, |d| element(d, filter, refs))
    }

    pub fn trackable_points(&self) -> Vec<(String, PointF)> {
        dispatch!(self, |d| d.trackable_points())
    }

    pub fn set_trackable_point(&mut self, key: &str, value: PointF) {
        dispatch!(self, |d| d.set_trackable_point(key, value))
    }

    pub fn content_hash(&self) -> i32 {
        dispatch!(self, |d| d.content_hash())
    }

    pub fn is_valid(&self) -> bool {
        dispatch!(self, |d| d.is_valid())
    }

    /// English tool name used for automatic drawing names (`ToolDisplayName`); postures use
    /// the tool's own name.
    pub fn tool_display_name(&self) -> String {
        match self {
            Drawing::GenericPostureDrawing(p) => p.tool_name(),
            other => other.fixed_display_name().to_owned(),
        }
    }

    /// Planes are appended to a keyframe while every other kind is inserted first.
    pub fn is_plane(&self) -> bool {
        matches!(self, Drawing::Plane(_))
    }

    /// Chronos, multi-chronos and counters live in the detached chrono list.
    pub fn is_chrono_like(&self) -> bool {
        matches!(
            self,
            Drawing::Chrono(_) | Drawing::ChronoMulti(_) | Drawing::Counter(_)
        )
    }

    /// `ITrackable` in the original: only these kinds register their points for tracking.
    pub fn is_trackable(&self) -> bool {
        matches!(
            self,
            Drawing::Line(_)
                | Drawing::Angle(_)
                | Drawing::CrossMark(_)
                | Drawing::Plane(_)
                | Drawing::Polyline(_)
                | Drawing::GenericPostureDrawing(_)
        )
    }
}

fn kva_name_of<D: KvaDrawing>(_: &D) -> &'static str {
    D::KVA_NAME
}
