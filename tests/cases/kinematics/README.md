# Kinematics cases

Each case synthesizes a trajectory and runs the original `FilteredTrajectory`, `LinearKinematics`
and `AngularKinematics` (C#) over it. Outputs per case:

- `truth.csv`: `frame,t,x,y` noiseless ground truth (not written for `winter`).
- `linear.csv`: `time` (frame index) then one column per `Kinematics` component:
  `XRaw,YRaw,X,Y` (raw and filtered coordinates, pixels), `LinearDistance`,
  `LinearHorizontalDisplacement`, `LinearVerticalDisplacement` (pixels), `LinearSpeed`,
  `LinearHorizontalVelocity`, `LinearVerticalVelocity` (pixels/s), `LinearAcceleration`,
  `LinearHorizontalAcceleration`, `LinearVerticalAcceleration` (pixels/s²).
- `angular.csv` (`circular` only): angle `o-a-b` with `o` at the image center, `a` the moving
  point, `b` fixed on the circle at angle 0. Columns: `AngularPosition` (degrees), `AngularDisplacement`,
  `TotalAngularDisplacement`, `AngularVelocity` (deg/s), `TangentialVelocity`, `AngularAcceleration`,
  `TangentialAcceleration`, `CentripetalAcceleration`, `ResultantLinearAcceleration`.
- `filtering.json`: Butterworth best cutoff indexes, whether filtering was enabled, best-fit circle.

Units follow the default preferences (pixels, degrees). Values are written with .NET round-trip
("R") formatting; compare with the tolerances declared in the Rust tests.

Kinds: `linear` (constant velocity plus uniform noise from `seed`), `circular` (constant angular
speed), `winter` (published gait data from Winter, "Biomechanics of Human Movement", 69.9 Hz, cm).
