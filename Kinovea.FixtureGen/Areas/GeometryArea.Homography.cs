using System.Collections.Generic;
using System.Drawing;
using System.IO;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    public partial class GeometryArea
    {
        /// <summary>
        /// Seeded plane rectangles mapped to perturbed image quadrilaterals: homography corners,
        /// mapped ellipse, point mappings and homogeneous vectors (including the Backward typo).
        /// </summary>
        private void Homography(string outDir, int count, int points)
        {
            List<IList<string>> cases = new List<IList<string>>();
            List<IList<string>> pts = new List<IList<string>>();
            List<IList<string>> vectors = new List<IList<string>>();
            for (int i = 0; i < count; i++)
            {
                QuadrilateralF plane = new QuadrilateralF(NextF(1, 10), NextF(1, 10));
                RectangleF box = new RectangleF(NextF(100, 600), NextF(100, 400), NextF(400, 1200), NextF(300, 600));
                QuadrilateralF image = new QuadrilateralF(box);
                for (int v = 0; v < 4; v++)
                    image[v] = image[v].Translate(NextF(-0.15, 0.15) * box.Width, NextF(-0.15, 0.15) * box.Height);

                ProjectiveMapper mapper = new ProjectiveMapper(plane, image);
                Ellipse e = mapper.Ellipse();
                cases.Add(CsvRows.Row(CsvRows.S(i), CsvRows.Quad(plane), CsvRows.Quad(image), CsvRows.Pt(e.Center), CsvRows.S(e.SemiMajorAxis, e.SemiMinorAxis, e.Rotation)));

                for (int j = 0; j < points; j++)
                {
                    PointF p = new PointF(NextF(0, plane.B.X), NextF(0, plane.D.Y));
                    PointF q = new PointF(NextF(box.Left, box.Right), NextF(box.Top, box.Bottom));
                    pts.Add(CsvRows.Row(CsvRows.S(i), CsvRows.Pt(p), CsvRows.Pt(mapper.Forward(p)), CsvRows.Pt(q), CsvRows.Pt(mapper.Backward(q))));

                    Vector3 h = new Vector3(NextF(-5, 5), NextF(-5, 5), NextF(0.5, 2));
                    vectors.Add(CsvRows.Row(CsvRows.S(i), CsvRows.V3(h), CsvRows.V3(mapper.Forward(h)), CsvRows.V3(mapper.Backward(h))));
                }
            }
            Save(outDir, "cases.csv", cases, "case", "p0x", "p0y", "p1x", "p1y", "p2x", "p2y", "p3x", "p3y", "i0x", "i0y", "i1x", "i1y", "i2x", "i2y", "i3x", "i3y", "ecx", "ecy", "ea", "eb", "erot");
            Save(outDir, "points.csv", pts, "case", "px", "py", "fx", "fy", "qx", "qy", "bx", "by");
            Save(outDir, "vectors.csv", vectors, "case", "x", "y", "z", "fx", "fy", "fz", "bx", "by", "bz");
        }
    }
}
