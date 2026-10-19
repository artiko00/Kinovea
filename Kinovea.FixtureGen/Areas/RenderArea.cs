using System;
using System.Collections.Generic;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Drawing.Imaging;
using System.Drawing.Text;
using System.IO;
using Kinovea.ScreenManager;
using Kinovea.Services;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// GDI+ rasterizations of a case KVA (`<timestamp>.png`) and hit-test tables (`hits.csv`).
    /// The frame is a flat colour; drawings are painted with the original `Draw` methods in the
    /// order of `PlayerScreenUserInterface2.FlushOnGraphics` / `FlushDrawingsOnGraphics`.
    /// </summary>
    public partial class RenderArea : IFixtureArea
    {
        public string Name { get { return "render"; } }

        public void Generate(FixtureCase c, string outDir)
        {
            string repoRoot = Oracle.RepoRoot(Path.GetDirectoryName(Path.GetDirectoryName(c.Path)));
            string source = Path.Combine(repoRoot, c.Input("kva", "").Replace('/', Path.DirectorySeparatorChar));
            InfosFading saved = PreferencesManager.PlayerPreferences.DefaultFading;
            JObject f = (JObject)c.Inputs["defaultFading"];
            InfosFading fading = new InfosFading();
            fading.FromInfosFading(saved);
            fading.Enabled = (bool)f["enabled"];
            fading.FadingFrames = (int)f["frames"];
            fading.OpaqueFrames = (int)f["opaque"];
            fading.AlwaysVisible = (bool)f["alwaysVisible"];
            PreferencesManager.PlayerPreferences.DefaultFading = fading;
            try
            {
                Metadata metadata = KvaArea.Load(source, null, c.Path);
                double scale = c.Input("scale", 1.0);
                Color frame = ParseColor(c.Input("frame", "0;0;0"));
                foreach (long t in c.Input("timestamps", new long[0]))
                {
                    using (Bitmap bmp = Render(metadata, scale, frame, t, fading.Enabled))
                        bmp.Save(Path.Combine(outDir, t.ToString(Oracle.Culture) + ".png"), ImageFormat.Png);
                }
                WriteHits(c, metadata, frame, fading.Enabled, outDir);
            }
            finally
            {
                PreferencesManager.PlayerPreferences.DefaultFading = saved;
            }
        }

        /// <summary>
        /// One frame at `scale` (viewport = image size × scale) and `timestamp`, 96 DPI, 32 bpp.
        /// </summary>
        private static Bitmap Render(Metadata m, double scale, Color frame, long timestamp, bool fadingEnabled)
        {
            Size size = new Size((int)Math.Round(m.ImageSize.Width * scale), (int)Math.Round(m.ImageSize.Height * scale));
            Bitmap bmp = new Bitmap(size.Width, size.Height, PixelFormat.Format32bppArgb);
            bmp.SetResolution(96, 96);
            using (Graphics g = Graphics.FromImage(bmp))
            {
                // FlushOnGraphics: the decoded frame (flat here), pixel offset and background.
                g.InterpolationMode = InterpolationMode.Bilinear;
                g.PixelOffsetMode = PixelOffsetMode.Half;
                using (SolidBrush brush = new SolidBrush(frame))
                    g.FillRectangle(brush, new Rectangle(Point.Empty, size));
                if (m.BackgroundColor.A != 0)
                {
                    using (SolidBrush brush = new SolidBrush(m.BackgroundColor))
                        g.FillRectangle(brush, new Rectangle(Point.Empty, size));
                }
                FlushDrawings(g, m, new ImageToViewportTransformer(Point.Empty, scale), KeyframeIndex(m, timestamp), timestamp, fadingEnabled);
            }
            return bmp;
        }

        /// <summary>
        /// Port of `FlushDrawingsOnGraphics` with nothing selected.
        /// </summary>
        private static void FlushDrawings(Graphics canvas, Metadata m, IImageToViewportTransformer transformer, int keyFrameIndex, long timestamp, bool fadingEnabled)
        {
            DistortionHelper distorter = m.CalibrationHelper.DistortionHelper;
            CameraTransformer camera = m.CameraTransformer;
            canvas.SmoothingMode = SmoothingMode.AntiAlias;
            canvas.TextRenderingHint = TextRenderingHint.AntiAlias;
            bool drawAttached = true;
            bool drawDetached = true;
            if (m.ActiveVideoFilter != null)
            {
                m.ActiveVideoFilter.DrawExtra(canvas, distorter, transformer, timestamp, false);
                drawAttached = m.ActiveVideoFilter.DrawAttachedDrawings;
                drawDetached = m.ActiveVideoFilter.DrawDetachedDrawings;
            }
            if (drawDetached)
            {
                foreach (AbstractDrawing d in m.ChronoManager.Drawings)
                    d.Draw(canvas, distorter, camera, transformer, false, timestamp);
                foreach (DrawingTrack d in m.TrackManager.Drawings)
                    d.Draw(canvas, distorter, camera, transformer, false, timestamp);
                foreach (AbstractDrawing d in m.SingletonDrawingsManager.Drawings)
                    d.Draw(canvas, distorter, camera, transformer, false, timestamp);
            }
            if (!drawAttached)
                return;
            if (fadingEnabled)
            {
                int[] zOrder = m.GetKeyframesZOrder(timestamp);
                for (int k = zOrder.Length - 1; k >= 0; k--)
                    DrawKeyframe(canvas, m.Keyframes[zOrder[k]], distorter, camera, transformer, timestamp);
            }
            else if (keyFrameIndex >= 0)
            {
                DrawKeyframe(canvas, m.Keyframes[keyFrameIndex], distorter, camera, transformer, timestamp);
            }
        }

        private static void DrawKeyframe(Graphics canvas, Keyframe kf, DistortionHelper distorter, CameraTransformer camera, IImageToViewportTransformer transformer, long timestamp)
        {
            for (int i = kf.Drawings.Count - 1; i >= 0; i--)
                kf.Drawings[i].Draw(canvas, distorter, camera, transformer, false, timestamp);
        }

        /// <summary>
        /// The player's active keyframe: the one exactly at the current time, -1 otherwise.
        /// </summary>
        private static int KeyframeIndex(Metadata m, long timestamp)
        {
            for (int i = 0; i < m.Count; i++)
                if (m[i].Timestamp == timestamp)
                    return i;
            return -1;
        }

        private static Color ParseColor(string text)
        {
            string[] p = text.Split(';');
            return Color.FromArgb(255, int.Parse(p[0], Oracle.Culture), int.Parse(p[1], Oracle.Culture), int.Parse(p[2], Oracle.Culture));
        }
    }
}
