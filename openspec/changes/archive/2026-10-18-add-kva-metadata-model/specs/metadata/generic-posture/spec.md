# Spec Delta

## Purpose

Loads posture tool descriptions (joint models such as human, bike fit, OpenPose Body25) and applies their handle constraints and impacts, so skeleton annotations can be created from data by the MCP server and edited consistently.

## ADDED Requirements

### Requirement: Posture descriptions load from XML
The model SHALL load `KinoveaPostureTool` XML (format 1.0 and 1.1): points with names, default positions and colors, segments, polylines, circles, angles, handles, hit zones, distances, positions, computed points, options and capabilities.

#### Scenario: Every shipped posture loads
- **WHEN** the 16 custom posture files under `Tools/DrawingTools/Custom` are loaded
- **THEN** each loads without error and the counts of points, segments, angles, handles and options equal the `posture/<name>/summary.json` fixture

### Requirement: OpenPose Body25 mapping
The `OpenPoseBody25` posture SHALL expose its 25 named points in OpenPose index order so that keypoint arrays can be mapped onto it.

#### Scenario: Index to name mapping
- **WHEN** the Body25 posture is loaded
- **THEN** point index 0 is `Nose`, index 1 is `Neck`, index 8 is `MidHip` and index 24 is the last point declared in the file

### Requirement: Constraint engine moves handles like the original
Moving a handle SHALL apply its constraint (locked, horizontal, vertical, line or parallel or perpendicular slide, distance to point, rotation steps) and its impacts (pivot, align, keep angle, symmetry, segment center) producing the same point set as the original application.

#### Scenario: Handle move parity
- **WHEN** the handle moves listed in the `posture/moves` fixture are applied to the named posture
- **THEN** every resulting point equals the fixture value within 1e-3 pixels

### Requirement: Postures serialize in KVA
A GenericPosture drawing SHALL be written to and read from KVA with its tool id, point positions, option values and style, loadable by the original application.

#### Scenario: Posture round trip
- **WHEN** a KVA containing a GenericPosture drawing from the fixture set is loaded and written back
- **THEN** the output equals the fixture byte for byte after id normalization
