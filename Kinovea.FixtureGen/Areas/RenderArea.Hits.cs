using System.Collections.Generic;
using System.Drawing;
using System.IO;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    public partial class RenderArea
    {
        /// <summary>
        /// `hits.csv`: every probe of a `hitStep` grid over the image whose `HitTest` result is not
        /// -1, for each viewport scale. Keyframe drawings are probed at their keyframe time, detached
        /// ones (chronos, tracks, then the singletons: spotlights, number sequence, coordinate system,
        /// test grid, in `SingletonDrawingsManager` order) at `hitTimestamp`; a frame is painted first at that time and scale because several
        /// drawings compute their screen boxes while drawing. Probes not listed are misses; a drawing
        /// whose `HitTest` throws gets a single `exception` row at the failing probe.
        /// </summary>
        private static void WriteHits(FixtureCase c, Metadata m, Color frame, bool fadingEnabled, string outDir)
        {
            double[] scales = c.Input("hitScales", new double[0]);
            if (scales.Length == 0)
                return;
            int step = c.Input("hitStep", 4);
            long detachedTime = c.Input("hitTimestamp", 0L);
            DistortionHelper distorter = m.CalibrationHelper.DistortionHelper;
            List<IList<string>> rows = new List<IList<string>>();
            foreach (double scale in scales)
            {
                ImageToViewportTransformer transformer = new ImageToViewportTransformer(Point.Empty, scale);
                for (int k = 0; k < m.Count; k++)
                {
                    long t = m[k].Timestamp;
                    Render(m, scale, frame, t, fadingEnabled).Dispose();
                    for (int i = 0; i < m[k].Drawings.Count; i++)
                        Probe(rows, "k" + k, i, m[k].Drawings[i], t, scale, step, m.ImageSize, distorter, transformer);
                }
                Render(m, scale, frame, detachedTime, fadingEnabled).Dispose();
                for (int i = 0; i < m.ChronoManager.Drawings.Count; i++)
                    Probe(rows, "chronos", i, m.ChronoManager.Drawings[i], detachedTime, scale, step, m.ImageSize, distorter, transformer);
                for (int i = 0; i < m.TrackManager.Drawings.Count; i++)
                    Probe(rows, "tracks", i, m.TrackManager.Drawings[i], detachedTime, scale, step, m.ImageSize, distorter, transformer);
                for (int i = 0; i < m.SingletonDrawingsManager.Drawings.Count; i++)
                    Probe(rows, "singletons", i, m.SingletonDrawingsManager.Drawings[i], detachedTime, scale, step, m.ImageSize, distorter, transformer);
            }
            string[] header = { "container", "index", "element", "timestamp", "scale", "x", "y", "result" };
            Oracle.WriteCsv(Path.Combine(outDir, "hits.csv"), header, rows);
        }

        private static void Probe(List<IList<string>> rows, string container, int index, AbstractDrawing d, long t, double scale, int step, Size image, DistortionHelper distorter, IImageToViewportTransformer transformer)
        {
            string element = KvaArea.ElementName(d);
            for (int y = 0; y < image.Height; y += step)
            {
                for (int x = 0; x < image.Width; x += step)
                {
                    int result;
                    try
                    {
                        result = d.HitTest(new PointF(x, y), t, distorter, transformer);
                    }
                    catch (System.Exception)
                    {
                        // The original throws for some states (e.g. a non-perspective plane whose
                        // projective mapping is only built while drawing a perspective grid).
                        // One `exception` row marks the drawing; it is not probed further.
                        rows.Add(CsvRows.Row(CsvRows.S(container, index, element, t, scale, x, y, "exception")));
                        return;
                    }
                    if (result >= 0)
                        rows.Add(CsvRows.Row(CsvRows.S(container, index, element, t, scale, x, y, result)));
                }
            }
        }
    }
}
