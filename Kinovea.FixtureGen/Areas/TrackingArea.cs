using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Reflection;
using Kinovea.ScreenManager;
using Kinovea.Services;
using Newtonsoft.Json.Linq;
using OpenCvSharp;
using OpenCvSharp.Extensions;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Runs the original trackers on frames rendered by formula (see TrackingArea.Frames.cs).
    /// inputs: frame description (width, height, background, gx, gy, color, shape, size), "path": one
    /// [x, y] center per frame in quarter pixels, "tsPerFrame" (default 1), "tracker": "template"|"circle",
    /// "searchWindow" [w, h], "blockWindow" [w, h], "similarity", "templateUpdate", "useMask", "hsv"
    /// [hueMin, hueMax, satMin, satMax, valMin, valMax], "dilate", "erode". The reference point is the
    /// first path point, rounded to the pixel; every later frame runs TrackStep on the growing timeline.
    /// track.csv: frame, t, x, y, r, score (template similarity of the last template; -1 for circles), matched.
    /// truth.csv: frame, x, y (the rendered center in pixels).
    /// </summary>
    public partial class TrackingArea : IFixtureArea
    {
        public string Name { get { return "tracking"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            SetupFrames(fixtureCase);
            JArray path = (JArray)fixtureCase.Inputs["path"];
            long tsPerFrame = fixtureCase.Input("tsPerFrame", 1L);
            bool circle = fixtureCase.Input("tracker", "template") == "circle";
            TrackingParameters parameters = Parameters(fixtureCase);
            AbstractTracker tracker = circle ? (AbstractTracker)new TrackerCircle(parameters) : new TrackerTemplateMatching(parameters);

            List<IList<string>> track = new List<IList<string>>();
            List<IList<string>> truth = new List<IList<string>>();
            List<TimedPoint> timeline = new List<TimedPoint>();
            for (int frame = 0; frame < path.Count; frame++)
            {
                JArray center = (JArray)path[frame];
                truth.Add(CsvRows.S(frame, (double)(int)center[0] / 4, (double)(int)center[1] / 4));
                long t = frame * tsPerFrame;
                using (Bitmap bitmap = Render(center))
                using (Mat image = BitmapConverter.ToMat(bitmap))
                {
                    TimedPoint point;
                    bool matched = true;
                    if (frame == 0)
                    {
                        point = new TimedPoint((int)center[0] / 4, (int)center[1] / 4, t);
                        tracker.CreateReferenceTrackPoint(point, image);
                    }
                    else
                    {
                        matched = tracker.TrackStep(timeline, t, image, out point);
                    }

                    timeline.Add(point);
                    track.Add(CsvRows.S(frame, t, point.X, point.Y, point.R, circle ? -1f : LastScore(tracker), matched ? 1 : 0));
                }
            }

            Oracle.WriteCsv(Path.Combine(outDir, "track.csv"), new[] { "frame", "t", "x", "y", "r", "score", "matched" }, track);
            Oracle.WriteCsv(Path.Combine(outDir, "truth.csv"), new[] { "frame", "x", "y" }, truth);
            if (parameters.UseMask && !circle)
                WriteMask(tracker, Path.Combine(outDir, "mask.csv"));
        }

        /// <summary>
        /// mask.csv: the elliptic mask the template tracker painted with GDI+ (row, then the gray values).
        /// </summary>
        private static void WriteMask(AbstractTracker tracker, string path)
        {
            FieldInfo field = typeof(TrackerTemplateMatching).GetField("cvMaskGray", BindingFlags.NonPublic | BindingFlags.Instance);
            Mat mask = (Mat)field.GetValue(tracker);
            List<IList<string>> rows = new List<IList<string>>();
            for (int y = 0; y < mask.Rows; y++)
            {
                List<string> values = new List<string>();
                for (int x = 0; x < mask.Cols; x++)
                    values.Add(mask.At<byte>(y, x).ToString(Oracle.Culture));
                rows.Add(new[] { y.ToString(Oracle.Culture), string.Join(" ", values) });
            }

            Oracle.WriteCsv(path, new[] { "row", "values" }, rows);
        }

        private static TrackingParameters Parameters(FixtureCase fixtureCase)
        {
            int[] search = fixtureCase.Input("searchWindow", new[] { 100, 100 });
            int[] block = fixtureCase.Input("blockWindow", new[] { 20, 20 });
            TrackingParameters parameters = new TrackingParameters();
            parameters.SearchWindow = new System.Drawing.Size(search[0], search[1]);
            parameters.BlockWindow = new System.Drawing.Size(block[0], block[1]);
            parameters.SimilarityThreshold = fixtureCase.Input("similarity", parameters.SimilarityThreshold);
            parameters.TemplateUpdateThreshold = fixtureCase.Input("templateUpdate", parameters.TemplateUpdateThreshold);
            parameters.UseMask = fixtureCase.Input("useMask", false);
            float[] hsv = fixtureCase.Input<float[]>("hsv", null);
            if (hsv != null)
                parameters.HSVRange = new HSVRange(hsv[0], hsv[1], hsv[2], hsv[3], hsv[4], hsv[5]);
            parameters.Dilate = fixtureCase.Input("dilate", 0);
            parameters.Erode = fixtureCase.Input("erode", 0);
            return parameters;
        }

        /// <summary>
        /// The similarity of the last stored template (the template timeline is private).
        /// </summary>
        private static float LastScore(AbstractTracker tracker)
        {
            FieldInfo field = typeof(TrackerTemplateMatching).GetField("trackedTemplates", BindingFlags.NonPublic | BindingFlags.Instance);
            Timeline<TrackingTemplate> templates = (Timeline<TrackingTemplate>)field.GetValue(tracker);
            return templates.Last().Score;
        }
    }
}
