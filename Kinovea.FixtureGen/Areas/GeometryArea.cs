using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Reflection;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Reference outputs of the geometry toolbox (Kinovea.ScreenManager/Measurement/Geometry).
    /// inputs: { "kind": "clipping"|"homography"|"helpers"|"quads"|"svd", "count": n, ... }
    /// </summary>
    public partial class GeometryArea : IFixtureArea
    {
        private Random random;

        public string Name { get { return "geometry"; } }

        public void Generate(FixtureCase c, string outDir)
        {
            random = new Random(c.Seed);
            int count = c.Input("count", 20);
            switch (c.Input("kind", ""))
            {
                case "clipping": Clipping(c, outDir, count); break;
                case "homography": Homography(outDir, count, c.Input("points", 10)); break;
                case "helpers": Helpers(outDir, count); break;
                case "quads": Quads(outDir, count); break;
                case "svd": Svd(outDir, count); break;
                default: throw new ArgumentException("unknown geometry kind in " + c.Path);
            }
        }

        private float NextF(double min, double max)
        {
            return (float)(min + random.NextDouble() * (max - min));
        }

        private PointF NextPoint(double min, double max)
        {
            return new PointF(NextF(min, max), NextF(min, max));
        }

        private static string[] Pt(PointF p)
        {
            return new[] { Oracle.R(p.X), Oracle.R(p.Y) };
        }

        private static List<string> Row(params string[][] parts)
        {
            List<string> row = new List<string>();
            foreach (string[] part in parts)
                row.AddRange(part);
            return row;
        }

        private static string[] S(params object[] values)
        {
            string[] out_ = new string[values.Length];
            for (int i = 0; i < values.Length; i++)
                out_[i] = values[i] is double ? Oracle.R((double)values[i]) : values[i] is float ? Oracle.R((float)values[i]) : Convert.ToString(values[i], Oracle.Culture);
            return out_;
        }

        private void Clipping(FixtureCase c, string outDir, int count)
        {
            List<float> w = c.Input("window", new List<float> { 500, 500, 1000, 1000 });
            RectangleF window = new RectangleF(w[0], w[1], w[2], w[3]);
            List<PointF[]> lines = new List<PointF[]>
            {
                new[] { new PointF(250, 1750), new PointF(1750, 250) },
                new[] { new PointF(250, 1500), new PointF(1750, 0) },
                new[] { new PointF(500, 250), new PointF(1750, 1250) },
                new[] { new PointF(250, 250), new PointF(500, 1750) },
                new[] { new PointF(250, 250), new PointF(250, 1750) },
                new[] { new PointF(600, 250), new PointF(600, 1750) },
                new[] { new PointF(500, 250), new PointF(500, 1750) },
                new[] { new PointF(250, 750), new PointF(1750, 750) },
                new[] { new PointF(750, 750), new PointF(1250, 1250) },
                new[] { new PointF(1750, 250), new PointF(750, 750) },
            };
            for (int i = 0; i < count; i++)
                lines.Add(new[] { NextPoint(0, 2000), NextPoint(0, 2000) });

            List<IList<string>> rows = new List<IList<string>>();
            foreach (PointF[] line in lines)
            {
                Add(rows, line, "segment", LiangBarsky.ClipSegment(window, line[0], line[1]));
                Add(rows, line, "line", LiangBarsky.ClipLine(window, line[0], line[1]));
                Add(rows, line, "ray_forward", LiangBarsky.ClipLine(window, line[0], line[1], 0, double.MaxValue));
                Add(rows, line, "ray_backward", LiangBarsky.ClipLine(window, line[0], line[1], double.MinValue, 1));
            }
            Oracle.WriteCsv(Path.Combine(outDir, "lines.csv"), new[] { "ax", "ay", "bx", "by", "mode", "visible", "ex", "ey", "xx", "xy" }, rows);
        }

        private static void Add(List<IList<string>> rows, PointF[] line, string mode, ClipResult r)
        {
            rows.Add(Row(Pt(line[0]), Pt(line[1]), S(mode, r.Visible ? 1 : 0), Pt(r.A), Pt(r.B)));
        }

        /// <summary>
        /// Calls the internal Numerical Recipes routine through reflection so the fixture keeps
        /// double precision (Matrix3x3.SVD would narrow to float).
        /// </summary>
        private void Svd(string outDir, int count)
        {
            Type svd = typeof(Matrix3x3).Assembly.GetType("Kinovea.ScreenManager.svd");
            MethodInfo svdcmp = svd.GetMethod("svdcmp", BindingFlags.Public | BindingFlags.Static);
            List<IList<string>> rows = new List<IList<string>>();
            for (int i = 0; i < count; i++)
            {
                int n = 2 + i % 4;
                int m = n + i % 3;
                double[,] a = new double[m, n];
                List<string> values = new List<string>();
                for (int r = 0; r < m; r++)
                    for (int col = 0; col < n; col++)
                    {
                        a[r, col] = NextF(-10, 10);
                        values.Add(Oracle.R(a[r, col]));
                    }
                object[] args = { a, null, null };
                svdcmp.Invoke(null, args);
                rows.Add(Row(S(i, m, n, string.Join(" ", values)), S(Flatten((double[])args[1]), Flatten(a), Flatten((double[,])args[2]))));
            }
            Oracle.WriteCsv(Path.Combine(outDir, "cases.csv"), new[] { "case", "m", "n", "values", "w", "u", "v" }, rows);
        }

        private static string Flatten(Array values)
        {
            List<string> parts = new List<string>();
            foreach (double v in values)
                parts.Add(Oracle.R(v));
            return string.Join(" ", parts);
        }
    }
}
