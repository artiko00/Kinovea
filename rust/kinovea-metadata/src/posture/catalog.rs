//! The 16 custom posture tools shipped in `Tools/DrawingTools/Custom`, embedded at compile
//! time (`GenericPostureManager`).

use super::model::GenericPosture;
use super::parser::parse_posture;
use uuid::Uuid;

macro_rules! custom {
    ($($file:literal),* $(,)?) => {
        &[$(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Tools/DrawingTools/Custom/", $file))),*]
    };
}

pub const CUSTOM_XML: &[&str] = custom!(
    "1 - Goniometer.xml", "2 - Posture.xml", "3 - Bike fit.xml", "5 - Genu flexum.xml",
    "6 - Archery - Top view.xml", "7 - Angle to vertical.xml", "8 - Angle to horizontal.xml",
    "9 - Horizontal distance.xml", "10 - Human model.xml", "11 - Human model 2.xml",
    "12 - Central sacral vertical line.xml", "body_with_feet.xml", "canis.xml", "equus.xml",
    "openpose_body25.xml", "profile.xml",
);

/// Every shipped posture, parsed fresh.
pub fn custom_postures() -> Vec<GenericPosture> {
    CUSTOM_XML.iter().filter_map(|xml| parse_posture(xml, false)).collect()
}

/// A fresh instance of the shipped tool with this id (`GenericPostureManager.Instanciate`).
pub fn posture_by_id(id: Uuid, from_kva: bool) -> Option<GenericPosture> {
    CUSTOM_XML.iter().filter_map(|xml| parse_posture(xml, from_kva)).find(|p| p.id == id)
}
