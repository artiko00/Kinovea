using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    public partial class GeometryArea
    {
        /// <summary>
        /// GeometryHelper calls over seeded points. angles.csv: oriented angles; points.csv: one
        /// row per operation with inputs a..d, scalar parameters p1, p2 and the result r.
        /// </summary>
        private void Helpers(string outDir, int count)
        {
            List<IList<string>> angles = new List<IList<string>>();
            List<IList<string>> pts = new List<IList<string>>();
            for (int i = 0; i < count; i++)
            {
                PointF a = NextPoint(-500, 500), b = NextPoint(-500, 500), c = NextPoint(-500, 500), d = NextPoint(-500, 500);
                angles.Add(Row(Pt(a), Pt(b), Pt(c), Pt(d), S(GeometryHelper.GetAngle(a, b, c, d), GeometryHelper.GetAngle(a, b, c))));

                float distance = NextF(0, 300), radians = NextF(-4, 4), alpha = NextF(-0.5, 1.5), degrees = NextF(-360, 360);
                int subdivisions = 2 + i % 11, margin = i % 7, position = i % 6;
                Op(pts, "distance", a, b, c, d, 0, 0, new PointF(GeometryHelper.GetDistance(a, b), 0));
                Op(pts, "middle", a, b, c, d, 0, 0, GeometryHelper.GetMiddlePoint(a, b));
                Op(pts, "at_distance", a, b, c, d, distance, 0, GeometryHelper.GetPointAtDistance(a, b, distance));
                Op(pts, "rotate", a, b, c, d, radians, 0, GeometryHelper.Rotate(a, b, radians));
                Op(pts, "closest_step", a, b, c, d, subdivisions, 0, GeometryHelper.GetPointAtClosestRotationStep(a, b, c, subdivisions));
                Op(pts, "closest_step_cardinal", a, b, c, d, subdivisions, 0, GeometryHelper.GetPointAtClosestRotationStepCardinal(a, c, subdivisions));
                Op(pts, "at_angle", a, b, c, d, degrees, 0, GeometryHelper.GetPointAtAngle(a, b, c, degrees));
                Op(pts, "at_angle_distance", a, b, c, d, radians, distance, GeometryHelper.GetPointAtAngleAndDistance(a, b, radians, distance));
                Op(pts, "on_parallel", a, b, c, d, 0, 0, GeometryHelper.GetPointOnParallel(a, b, c, d));
                Op(pts, "mix", a, b, c, d, alpha, 0, GeometryHelper.Mix(a, b, alpha));
                Op(pts, "closest_point", a, b, c, d, position, margin, GeometryHelper.GetClosestPoint(a, b, c, (PointLinePosition)position, margin));
                RectangleF rect = new RectangleF(a.X, a.Y, Math.Abs(b.X) + 1, Math.Abs(b.Y) + 1);
                PointF end = rect.Center().Translate(c.X * 10, c.Y * 10);
                Op(pts, "intersection", a, new PointF(rect.Width, rect.Height), end, d, 0, 0, GeometryHelper.IntersectionRectangleCenter(rect, end));
            }
            Oracle.WriteCsv(Path.Combine(outDir, "angles.csv"), new[] { "ax", "ay", "bx", "by", "cx", "cy", "dx", "dy", "angle4", "angle3" }, angles);
            Oracle.WriteCsv(Path.Combine(outDir, "points.csv"), new[] { "op", "ax", "ay", "bx", "by", "cx", "cy", "dx", "dy", "p1", "p2", "rx", "ry" }, pts);
        }

        private static void Op(List<IList<string>> rows, string op, PointF a, PointF b, PointF c, PointF d, float p1, float p2, PointF r)
        {
            rows.Add(Row(S(op), Pt(a), Pt(b), Pt(c), Pt(d), S(p1, p2), Pt(r)));
        }

        /// <summary>
        /// QuadrilateralF operations: inputs (8 coords), parameters, outputs (8 coords or fewer),
        /// plus convexity and axis alignment of the input.
        /// </summary>
        private void Quads(string outDir, int count)
        {
            List<IList<string>> rows = new List<IList<string>>();
            for (int i = 0; i < count; i++)
            {
                QuadrilateralF free = new QuadrilateralF(NextPoint(0, 1000), NextPoint(0, 1000), NextPoint(0, 1000), NextPoint(0, 1000));
                QuadrilateralF rect = new QuadrilateralF(new RectangleF(NextF(0, 500), NextF(0, 500), NextF(1, 500), NextF(1, 500)));
                int anchor = i % 4;
                float p1 = NextF(-50, 50), p2 = NextF(-50, 50);
                QuadOp(rows, "make_rectangle", free, anchor, 0, 0, q => q.MakeRectangle(anchor));
                QuadOp(rows, "make_square", rect, anchor, 0, 0, q => q.MakeSquare(anchor));
                QuadOp(rows, "flip_horizontal", free, 0, 0, 0, q => q.FlipHorizontal());
                QuadOp(rows, "flip_vertical", free, 0, 0, 0, q => q.FlipVertical());
                QuadOp(rows, "rotate90", free, 0, 0, 0, q => q.Rotate90());
                QuadOp(rows, "expand", free, 0, p1, p2, q => q.Expand(p1, p2));
                QuadOp(rows, "translate", free, 0, p1, p2, q => q.Translate(p1, p2));
                QuadOp(rows, "scale", free, 0, p1 / 10, p2 / 10, q => q.Scale(p1 / 10, p2 / 10));
                RectangleF box = free.GetBoundingBox();
                rows.Add(Row(S("bounding_box", 0, 0f, 0f), Quad(free), S(box.X, box.Y, box.Width, box.Height, 0f, 0f, 0f, 0f), Flags(free)));
                PointF probe = new PointF(NextF(0, 1000), NextF(0, 1000));
                rows.Add(Row(S("contains", 0, probe.X, probe.Y), Quad(free), S(free.Contains(probe) ? 1f : 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f), Flags(free)));
            }
            Oracle.WriteCsv(Path.Combine(outDir, "ops.csv"),
                new[] { "op", "anchor", "p1", "p2", "ax", "ay", "bx", "by", "cx", "cy", "dx", "dy", "oax", "oay", "obx", "oby", "ocx", "ocy", "odx", "ody", "convex", "axis_aligned" }, rows);
        }

        private static void QuadOp(List<IList<string>> rows, string op, QuadrilateralF input, int anchor, float p1, float p2, Action<QuadrilateralF> apply)
        {
            QuadrilateralF q = input.Clone();
            apply(q);
            rows.Add(Row(S(op, anchor, p1, p2), Quad(input), Quad(q), Flags(input)));
        }

        private static string[] Flags(QuadrilateralF q)
        {
            return S(q.IsConvex ? 1 : 0, q.IsAxisAlignedRectangle ? 1 : 0);
        }
    }
}
