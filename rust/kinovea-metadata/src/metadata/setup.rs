//! `Metadata.PostSetupVideo(init)`: what the player does once a video is open, before a KVA is merged.

use super::Metadata;
use crate::singletons::{CoordinateSystem, TestGrid};

impl Metadata {
    /// `PostSetupVideo(true)`. The first setup names the singletons (`AfterDrawingCreation`, through
    /// `SetDrawingName`), and the coordinate system is registered for trackability at reference
    /// timestamp 0, first in the registry. The test grid's `Scale` needs no state here: its hit and
    /// paint read the image size of the metadata. Later setups only register the coordinate system again.
    pub fn post_setup_video(&mut self) {
        if !self.video_set_up {
            self.name_singletons();
            self.video_set_up = true;
        }
        self.register_coordinate_system();
    }

    fn name_singletons(&mut self) {
        if self.coordinate_system.header.name.is_empty() {
            self.coordinate_system.header.name = self.next_name(CoordinateSystem::TOOL_NAME, &[]);
        }
        if self.test_grid.header.name.is_empty() {
            self.test_grid.header.name = self.next_name(TestGrid::TOOL_NAME, &[]);
        }
    }

    /// `TrackableDrawingAdded` of the coordinate system: its origin is the reference value at 0.
    fn register_coordinate_system(&mut self) {
        let cs = &mut self.coordinate_system;
        if cs.header.reference_timestamp < 0 {
            cs.header.reference_timestamp = 0;
        }
        let points = cs.trackable_points();
        self.trackability
            .add(cs.header.id, &cs.header.name, 0, &points);
    }
}

#[cfg(test)]
#[path = "setup_test.rs"]
mod setup_test;
