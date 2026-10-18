using System;
using System.Collections.Generic;
using System.Drawing;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen
{
    /// <summary>
    /// Builders for CSV cells shared by the fixture areas.
    /// </summary>
    public static class CsvRows
    {
        public static string[] Pt(PointF p)
        {
            return new[] { Oracle.R(p.X), Oracle.R(p.Y) };
        }

        public static string[] Quad(QuadrilateralF q)
        {
            return Row(Pt(q.A), Pt(q.B), Pt(q.C), Pt(q.D)).ToArray();
        }

        public static string[] V3(Vector3 v)
        {
            return S(v.X, v.Y, v.Z);
        }

        public static List<string> Row(params string[][] parts)
        {
            List<string> row = new List<string>();
            foreach (string[] part in parts)
                row.AddRange(part);
            return row;
        }

        /// <summary>
        /// Formats scalars: doubles and floats with round-trip precision, anything else invariant.
        /// </summary>
        public static string[] S(params object[] values)
        {
            string[] cells = new string[values.Length];
            for (int i = 0; i < values.Length; i++)
                cells[i] = values[i] is double ? Oracle.R((double)values[i]) : values[i] is float ? Oracle.R((float)values[i]) : Convert.ToString(values[i], Oracle.Culture);
            return cells;
        }
    }
}
