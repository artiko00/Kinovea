# Spec Delta

## Purpose

Lets the application discover, configure and grab frames from cameras on Linux through interchangeable backends (deterministic generator, HTTP, V4L2, GenICam), presenting them uniformly to the capture screen and to tests.

## ADDED Requirements

### Requirement: Uniform camera summary and discovery
Every backend SHALL report cameras as summaries carrying alias, name, stable identifier, icon, aspect ratio, rotation, mirror flag and backend-specific settings, and SHALL be polled for discovery on a fixed interval while the camera list is visible.

#### Scenario: Discovery merges remembered settings
- **WHEN** a camera previously configured with alias "Side" and rotation 90 is rediscovered
- **THEN** the summary presents alias "Side" and rotation 90 instead of the defaults

#### Scenario: Forgetting a camera
- **WHEN** the user chooses "Forget custom settings" on a camera in the camera list of the file browser panel
- **THEN** the next discovery presents that camera with default alias, aspect ratio, rotation and mirror

### Requirement: Deterministic frame generator backend
A frame generator backend SHALL produce synthetic frames at a configured resolution and frame rate with a frame counter drawn into the image and a fixed pattern seeded by configuration, and it SHALL be available in every build so tests run without hardware.

#### Scenario: Frame counter advances
- **WHEN** the generator runs at 60 fps for one second
- **THEN** 60 ± 1 frames are produced with consecutive identifiers

#### Scenario: Configuration
- **WHEN** the user configures the generator camera (Configure button on the capture screen) with 1280x720 and 30 fps
- **THEN** the next grabbing session produces frames with that descriptor

### Requirement: HTTP camera backend
The HTTP backend SHALL grab frames from an MJPEG stream or from repeated JPEG snapshots at a URL with optional basic authentication, as the original Kinovea HTTP camera did.

#### Scenario: MJPEG stream
- **WHEN** the configured URL serves multipart MJPEG at 15 fps
- **THEN** decoded frames are produced at the stream rate and the reported live data rate matches the received bytes

#### Scenario: Unreachable URL
- **WHEN** the URL does not answer within the timeout
- **THEN** grabbing stops, the capture screen shows the connection error, and the backend retries at the discovery interval

### Requirement: V4L2 backend replaces DirectShow
The V4L2 backend SHALL enumerate `/dev/video*` capture devices, expose their formats, resolutions and frame rates, and grab frames in MJPEG, YUYV or RGB with conversion to the pipeline format.

#### Scenario: Enumeration
- **WHEN** two capture devices are present
- **THEN** both appear in the camera list with the kernel-reported name and a stable identifier derived from the bus path

#### Scenario: Format selection persists
- **WHEN** the user selects 1920x1080 MJPEG 30 fps in the configuration dialog
- **THEN** the choice is stored with the camera settings and applied on the next connection

### Requirement: GenICam backend through Aravis
When built with the GenICam feature, the backend SHALL discover GenICam cameras through Aravis, expose exposure, gain, frame rate and region of interest, and convert Mono8 and Bayer8 to the pipeline format.

#### Scenario: Feature disabled
- **WHEN** the build does not enable the GenICam feature
- **THEN** no GenICam entry appears in the camera list and no Aravis library is required at run time

#### Scenario: Bayer conversion
- **WHEN** a camera delivers BayerRG8 frames
- **THEN** frames reach the pipeline as RGB with the same dimensions

### Requirement: Camera thumbnails
Each backend SHALL produce a single snapshot thumbnail on request without starting a full grabbing session.

#### Scenario: Thumbnail for the camera list
- **WHEN** the camera list is shown
- **THEN** each camera receives one thumbnail within the discovery interval or a placeholder when the camera does not answer
