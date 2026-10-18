using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using Kinovea.ScreenManager;
using Kinovea.Services;
using Kinovea.Tests.Kinematics;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Synthesizes a trajectory with known ground truth, runs the original filtering and
    /// linear/angular kinematics, and writes the resulting time series.
    /// inputs: { "kind": "linear"|"circular"|"winter", "fps", "frames", "imageWidth", "imageHeight",
    ///           "speedX", "speedY" (px/frame), "radius", "angularSpeed" (rad/s), "noise" (px, std-dev-ish),
    ///           "filtering" and "smoothing" (player preferences, default true) }
    /// </summary>
    public partial class KinematicsArea : IFixtureArea
    {
        public string Name { get { return "kinematics"; } }

        public void Generate(FixtureCase c, string outDir)
        {
            string kind = c.Input("kind", "linear");
            double fps = c.Input("fps", 100.0);
            int frames = c.Input("frames", 120);
            Size imageSize = new Size(c.Input("imageWidth", 1920), c.Input("imageHeight", 1080));
            Random random = new Random(c.Seed);
            bool filtering = c.Input("filtering", true);
            bool smoothing = c.Input("smoothing", true);
            bool savedFiltering = PreferencesManager.PlayerPreferences.EnableFiltering;
            bool savedSmoothing = PreferencesManager.PlayerPreferences.EnableHighSpeedDerivativesSmoothing;
            PreferencesManager.PlayerPreferences.EnableFiltering = filtering;
            PreferencesManager.PlayerPreferences.EnableHighSpeedDerivativesSmoothing = smoothing;
            try
            {
                GenerateWithPreferences(c, outDir, kind, fps, frames, imageSize, random);
            }
            finally
            {
                PreferencesManager.PlayerPreferences.EnableFiltering = savedFiltering;
                PreferencesManager.PlayerPreferences.EnableHighSpeedDerivativesSmoothing = savedSmoothing;
            }
        }

        private static void GenerateWithPreferences(FixtureCase c, string outDir, string kind, double fps, int frames, Size imageSize, Random random)
        {

            CalibrationHelper calibration = new CalibrationHelper();
            calibration.Initialize(imageSize, t => PointF.Empty, (t, type, id) => new QuadrilateralF(), id => false);
            calibration.CaptureFramesPerSecond = fps;

            List<TimedPoint> moving = new List<TimedPoint>();
            List<TimedPoint> origin = new List<TimedPoint>();
            List<TimedPoint> fixedLeg = new List<TimedPoint>();
            List<IList<string>> truth = new List<IList<string>>();
            PointF center = new PointF(imageSize.Width / 2f, imageSize.Height / 2f);

            if (kind == "winter")
            {
                // Published gait data (Winter), sampled at 69.9 Hz, in cm; used as-is.
                fps = 69.9;
                calibration.CaptureFramesPerSecond = fps;
                frames = KinematicsTestData.RightToeX.Length;
                for (int i = 0; i < frames; i++)
                    moving.Add(new TimedPoint((float)KinematicsTestData.RightToeX[i], (float)KinematicsTestData.RightToeY[i], i));
            }
            else
            {
                MovingObject o = new MovingObject { SpeedX = c.Input("speedX", 3.0), SpeedY = c.Input("speedY", -1.0), NoiseX = c.Input("noise", 0.0), NoiseY = c.Input("noise", 0.0) };
                double radius = c.Input("radius", 200.0);
                double angularSpeed = c.Input("angularSpeed", 1.5);
                for (int i = 0; i < frames; i++)
                {
                    double t = i / fps;
                    double x, y;
                    if (kind == "circular")
                    {
                        x = center.X + radius * Math.Cos(angularSpeed * t);
                        y = center.Y + radius * Math.Sin(angularSpeed * t);
                    }
                    else
                    {
                        x = center.X + o.SpeedX * i;
                        y = center.Y + o.SpeedY * i;
                    }
                    truth.Add(new[] { i.ToString(Oracle.Culture), Oracle.R(t), Oracle.R(x), Oracle.R(y) });
                    x += (random.NextDouble() * 2 - 1) * o.NoiseX;
                    y += (random.NextDouble() * 2 - 1) * o.NoiseY;
                    moving.Add(new TimedPoint((float)x, (float)y, i));
                    origin.Add(new TimedPoint(center.X, center.Y, i));
                    fixedLeg.Add(new TimedPoint(center.X + (float)radius, center.Y, i));
                }
                Oracle.WriteCsv(Path.Combine(outDir, "truth.csv"), new[] { "frame", "t", "x", "y" }, truth);
            }

            FilteredTrajectory traj = new FilteredTrajectory();
            traj.Initialize(moving, calibration);
            TimeSeriesCollection linear = new LinearKinematics().BuildKinematics(traj, calibration);
            WriteSeries(Path.Combine(outDir, "linear.csv"), linear, linearComponents);

            Oracle.WriteText(Path.Combine(outDir, "filtering.json"),
                "{\n" +
                "  \"best_fit_circle\": { \"cx\": " + Oracle.R(traj.BestFitCircle.Center.X) + ", \"cy\": " + Oracle.R(traj.BestFitCircle.Center.Y) + ", \"radius\": " + Oracle.R(traj.BestFitCircle.Radius) + " },\n" +
                "  \"can_filter\": " + (traj.CanFilter ? "true" : "false") + ",\n" +
                "  \"fps\": " + Oracle.R(fps) + ",\n" +
                "  \"x_cutoff_index\": " + traj.XCutoffIndex + ",\n" +
                "  \"y_cutoff_index\": " + traj.YCutoffIndex + "\n" +
                "}\n");

            if (kind != "circular")
                return;

            Dictionary<string, FilteredTrajectory> legs = new Dictionary<string, FilteredTrajectory>();
            legs["o"] = Trajectory(origin, calibration);
            legs["a"] = traj;
            legs["b"] = Trajectory(fixedLeg, calibration);
            TimeSeriesCollection angular = new AngularKinematics().BuildKinematics(legs, new AngleOptions(true, true, false), calibration);
            WriteSeries(Path.Combine(outDir, "angular.csv"), angular, angularComponents);
        }
    }
}
