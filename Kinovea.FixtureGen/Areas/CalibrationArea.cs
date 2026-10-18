using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using Kinovea.ScreenManager;
using Kinovea.Services;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Reference outputs of CalibrationHelper, CalibratorPlane, DistortionHelper and CameraPoser.
    /// inputs: { "kind": "transforms"|"kva"|"distortion"|"pose", "imageWidth", "imageHeight", ... }
    /// </summary>
    public partial class CalibrationArea : IFixtureArea
    {
        private Random random;
        private Size imageSize;

        public string Name { get { return "calibration"; } }

        public void Generate(FixtureCase c, string outDir)
        {
            random = new Random(c.Seed);
            imageSize = new Size(c.Input("imageWidth", 1920), c.Input("imageHeight", 1080));
            switch (c.Input("kind", ""))
            {
                case "transforms": Transforms(c, outDir); break;
                case "kva": Kva(c, outDir); break;
                case "distortion": Distortion(c, outDir); break;
                case "pose": Pose(c.Input("count", 20), outDir); break;
                default: throw new ArgumentException("unknown calibration kind in " + c.Path);
            }
        }

        private float NextF(double min, double max)
        {
            return (float)(min + random.NextDouble() * (max - min));
        }

        private static PointF Pt(JToken t)
        {
            return new PointF((float)t[0], (float)t[1]);
        }

        private static QuadrilateralF QuadOf(JToken t)
        {
            return new QuadrilateralF(new PointF((float)t[0], (float)t[1]), new PointF((float)t[2], (float)t[3]), new PointF((float)t[4], (float)t[5]), new PointF((float)t[6], (float)t[7]));
        }

        /// <summary>
        /// Builds a helper from a calibration description: default, line (length, a, b, axis) or
        /// plane (size, quad, optional origin and offset); "unit" sets the length unit.
        /// </summary>
        private CalibrationHelper Build(JToken spec, double fps, Guid id)
        {
            CalibrationHelper h = new CalibrationHelper();
            h.Initialize(imageSize, t => PointF.Empty, (t, type, drawing) => new QuadrilateralF(), drawing => false);
            h.CaptureFramesPerSecond = fps;
            if (spec["distortion"] != null)
                h.DistortionHelper.Initialize(Parameters(spec["distortion"]), imageSize);
            if (spec["length"] != null)
            {
                CalibrationAxis axis = (CalibrationAxis)Enum.Parse(typeof(CalibrationAxis), (string)spec["axis"] ?? "LineHorizontal");
                h.CalibrationByLine_Initialize(id, (float)spec["length"], Pt(spec["a"]), Pt(spec["b"]), axis);
            }
            else if (spec["size"] != null)
            {
                h.SetCalibratorFromType(CalibratorType.Plane);
                h.CalibrationByPlane_Initialize(id, new SizeF((float)spec["size"][0], (float)spec["size"][1]), QuadOf(spec["quad"]));
            }
            if (spec["origin"] != null)
                h.SetOrigin(Pt(spec["origin"]));
            if (spec["offset"] != null)
                h.SetOffset(Pt(spec["offset"]));
            if (spec["unit"] != null)
                h.LengthUnit = (LengthUnit)Enum.Parse(typeof(LengthUnit), (string)spec["unit"]);
            return h;
        }

        private DistortionParameters Parameters(JToken d)
        {
            return new DistortionParameters((double)d["k1"], (double)d["k2"], (double)d["k3"], (double)d["p1"], (double)d["p2"], (double)d["fx"], (double)d["fy"], (double)d["cx"], (double)d["cy"], imageSize);
        }

        private void Transforms(FixtureCase c, string outDir)
        {
            int count = c.Input("count", 25);
            double fps = c.Input("fps", 50.0);
            foreach (JToken spec in c.Inputs["calibrations"])
            {
                CalibrationHelper h = Build(spec, fps, Guid.Empty);
                List<IList<string>> rows = new List<IList<string>>();
                for (int i = 0; i < count; i++)
                {
                    PointF p = new PointF(NextF(0, imageSize.Width), NextF(0, imageSize.Height));
                    PointF q = new PointF(NextF(0, imageSize.Width), NextF(0, imageSize.Height));
                    float s = NextF(-500, 500);
                    PointF w = h.GetPoint(p);
                    float gs = h.GetScalar(s);
                    Ellipse e = h.GetEllipseFromCircle(new Circle(w, Math.Abs(gs)));
                    rows.Add(CsvRows.Row(
                        CsvRows.Pt(p), CsvRows.Pt(w), CsvRows.Pt(h.GetImagePoint(w)), CsvRows.Pt(q),
                        CsvRows.S(s, gs, h.GetImageScalar(s), h.GetImageLength(w, h.GetPoint(q)), i, h.GetTime(i)),
                        CsvRows.S(h.ConvertSpeed(s), h.ConvertAcceleration(s), h.ConvertAccelerationFromVelocity(s), h.ConvertAngle(s / 100), h.ConvertAngularVelocity(s / 100), h.ConvertAngularAcceleration(s / 100)),
                        CsvRows.S(Oracle.Csv(h.GetPointText(p, true, 0)), Oracle.Csv(h.GetLengthText(p, q, true)), Oracle.Csv(h.GetCircumferenceText(p, q, true))),
                        CsvRows.Pt(e.Center), CsvRows.S(e.SemiMajorAxis, e.SemiMinorAxis, e.Rotation), CsvRows.Pt(h.GetOrigin())));
                }
                Oracle.WriteCsv(Path.Combine(outDir, (string)spec["name"] + ".csv"),
                    new[] { "ix", "iy", "wx", "wy", "bx", "by", "qx", "qy", "s", "gs", "gis", "ilen", "frames", "time", "cs", "ca", "cav", "cang", "cangv", "cangacc", "point_text", "length_text", "circ_text", "ecx", "ecy", "ea", "eb", "erot", "ox", "oy" }, rows);
            }
        }
    }
}
