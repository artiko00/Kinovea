//! Custom posture drawing (`DrawingGenericPosture`, KVA `GenericPosture`): a shipped tool
//! description plus the current position of its points.

use super::{Header, KvaDrawing, RefValue};
use crate::color::Color;
use crate::ids::id_text;
use crate::kva_io::*;
use crate::posture::engine::{move_handle, Space};
use crate::posture::{posture_by_id, GenericPosture};
use crate::style::{StyleElements, StyleValue};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericPostureDrawing {
    pub header: Header,
    pub tool_id: Uuid,
    /// `None` when the tool is not shipped: the drawing then serializes to nothing.
    pub posture: Option<GenericPosture>,
}

impl GenericPostureDrawing {
    pub fn new(tool_id: Uuid, posture: Option<GenericPosture>) -> Self {
        let style = StyleElements::new(vec![(
            "line color",
            StyleValue::Color(Color::rgb(85, 107, 47)),
        )]);
        GenericPostureDrawing {
            header: Header::new("", style),
            tool_id,
            posture,
        }
    }

    /// The posture's name is the tool display name used for automatic drawing names.
    pub fn tool_name(&self) -> String {
        self.posture
            .as_ref()
            .map(|p| p.name.clone())
            .unwrap_or_default()
    }
}

impl KvaDrawing for GenericPostureDrawing {
    const KVA_NAME: &'static str = "GenericPosture";

    fn header(&self) -> &Header {
        &self.header
    }

    fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    fn read_kva(e: &Element, scale: Scale, _map: TimestampMapper) -> Self {
        let mut d = GenericPostureDrawing::new(Uuid::nil(), None);
        d.header.read_attributes(e);
        for c in &e.children {
            match c.name.as_str() {
                "ToolId" => {
                    d.tool_id = Uuid::parse_str(c.text.trim()).unwrap_or_default();
                    d.posture = posture_by_id(d.tool_id, true);
                }
                "Positions" => {
                    let points: Vec<PointF> = c
                        .children
                        .iter()
                        .filter(|p| p.name == "Point")
                        .map(|p| read_point(p, scale))
                        .collect();
                    if let Some(posture) = d
                        .posture
                        .as_mut()
                        .filter(|p| p.point_list.len() == points.len())
                    {
                        posture.point_list = points;
                    }
                }
                _ => {
                    read_style_or_fading(c, &mut d.header.style, &mut d.header.fading);
                }
            }
        }
        d
    }

    fn write_kva(&self, filter: Filter, _refs: RefValue) -> Vec<Element> {
        let mut out = Vec::new();
        let Some(posture) = self.posture.as_ref().filter(|p| !p.id.is_nil()) else {
            return out;
        };
        out.push(Element::leaf("ToolId", &id_text(posture.id)));
        if filter.core {
            let mut positions = Element::new("Positions");
            positions.children = posture
                .point_list
                .iter()
                .map(|p| point_leaf("Point", *p))
                .collect();
            out.push(positions);
        }
        push_style_and_fading(&mut out, filter, &self.header.style, &self.header.fading);
        out
    }

    fn trackable_points(&self) -> Vec<(String, PointF)> {
        let Some(p) = &self.posture else {
            return Vec::new();
        };
        p.trackable_indices
            .iter()
            .map(|i| {
                (
                    i.to_string(),
                    p.point_list
                        .get(*i as usize)
                        .copied()
                        .unwrap_or(PointF::EMPTY),
                )
            })
            .collect()
    }

    /// `SetTrackablePointValue`: the point moves through its handle so constraints apply.
    fn set_trackable_point(&mut self, key: &str, value: PointF) {
        let Some(p) = &mut self.posture else {
            return;
        };
        let Ok(index) = key.parse::<i32>() else {
            return;
        };
        if let Some(handle) = p.handles.iter().position(|h| h.reference == index) {
            move_handle(p, Space(None), handle, value, false);
        }
    }

    fn content_hash(&self) -> i32 {
        let points = self.posture.as_ref().map_or(0, |p| {
            p.point_list.iter().fold(0, |h, pt| h ^ pt.hash_code())
        });
        points ^ self.header.style.content_hash() ^ self.header.fading.content_hash()
    }
}
