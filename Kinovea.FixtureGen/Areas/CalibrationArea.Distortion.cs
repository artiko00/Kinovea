using System.Collections.Generic;
using System.Drawing;
using System.IO;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    public partial class CalibrationArea
    {
        /// <summary>
        /// Distort/undistort of seeded points and lines (Cv2.UndistortPoints for the inverse), the
        /// exported Kinovea file and the parameters imported from the Agisoft sample.
        /// </summary>
        private void Distortion(FixtureCase c, string outDir)
        {
            int count = c.Input("count", 60);
            DistortionParameters parameters = Parameters(c.Inputs["parameters"]);
            DistortionHelper helper = new DistortionHelper();
            helper.Initialize(parameters, imageSize);

            List<IList<string>> points = new List<IList<string>>();
            List<IList<string>> lines = new List<IList<string>>();
            for (int i = 0; i < count; i++)
            {
                PointF p = new PointF(NextF(0, imageSize.Width), NextF(0, imageSize.Height));
                points.Add(CsvRows.Row(CsvRows.Pt(p), CsvRows.Pt(helper.Distort(p)), CsvRows.Pt(helper.Undistort(p))));
                PointF q = new PointF(NextF(0, imageSize.Width), NextF(0, imageSize.Height));
                lines.Add(LineRow("line", p, q, helper.DistortLine(p, q)));
                lines.Add(LineRow("rectified", p, q, helper.DistortRectifiedLine(p, q)));
            }
            Oracle.WriteCsv(Path.Combine(outDir, "points.csv"), new[] { "x", "y", "dx", "dy", "ux", "uy" }, points);
            List<string> header = new List<string> { "kind", "sx", "sy", "ex", "ey" };
            for (int i = 0; i < 7; i++)
                header.AddRange(new[] { "p" + i + "x", "p" + i + "y" });
            Oracle.WriteCsv(Path.Combine(outDir, "lines.csv"), header, lines);

            string kinovea = Path.Combine(outDir, "kinovea.xml");
            DistortionImporterKinovea.Export(kinovea, parameters, imageSize);
            Oracle.NormalizeTextFile(kinovea);

            string repoRoot = Oracle.RepoRoot(Path.GetDirectoryName(Path.GetDirectoryName(c.Path)));
            string agisoft = Path.Combine(repoRoot, c.Input("agisoft", "").Replace('/', Path.DirectorySeparatorChar));
            List<IList<string>> imported = new List<IList<string>>();
            foreach (Size size in new[] { imageSize, new Size(imageSize.Width / 2, imageSize.Height / 2) })
            {
                DistortionParameters a = DistortionImporterAgisoft.Import(agisoft, size);
                imported.Add(CsvRows.S(size.Width, size.Height, a.Fx, a.Fy, a.Cx, a.Cy, a.K1, a.K2, a.K3, a.P1, a.P2, a.PixelsPerMillimeter));
            }
            Oracle.WriteCsv(Path.Combine(outDir, "agisoft.csv"), new[] { "width", "height", "fx", "fy", "cx", "cy", "k1", "k2", "k3", "p1", "p2", "ppmm" }, imported);
        }

        private static IList<string> LineRow(string kind, PointF start, PointF end, List<PointF> points)
        {
            List<string> row = CsvRows.Row(CsvRows.S(kind), CsvRows.Pt(start), CsvRows.Pt(end));
            foreach (PointF p in points)
                row.AddRange(CsvRows.Pt(p));
            return row;
        }

        /// <summary>
        /// CameraPoser on seeded world rectangles seen through perturbed image quadrilaterals,
        /// with the default intrinsics of the image size.
        /// </summary>
        private void Pose(int count, string outDir)
        {
            DistortionParameters lens = new DistortionParameters(imageSize);
            List<IList<string>> rows = new List<IList<string>>();
            for (int i = 0; i < count; i++)
            {
                float w = NextF(1, 10), h = NextF(1, 10);
                QuadrilateralF plane = new QuadrilateralF(w, h);
                RectangleF box = new RectangleF(NextF(100, 600), NextF(100, 400), NextF(400, 1200), NextF(300, 600));
                QuadrilateralF image = new QuadrilateralF(box);
                for (int v = 0; v < 4; v++)
                    image[v] = image[v].Translate(NextF(-0.15, 0.15) * box.Width, NextF(-0.15, 0.15) * box.Height);
                Vector3 position = CameraPoser.Compute(plane, new ProjectiveMapper(plane, image), lens);
                rows.Add(CsvRows.Row(CsvRows.S(w, h), CsvRows.Quad(image), CsvRows.V3(position)));
            }
            Oracle.WriteCsv(Path.Combine(outDir, "cases.csv"), new[] { "w", "h", "i0x", "i0y", "i1x", "i1y", "i2x", "i2y", "i3x", "i3y", "x", "y", "z" }, rows);
        }
    }
}
