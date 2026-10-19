using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Windows.Forms;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    public partial class PostureArea
    {
        private static readonly Size imageSize = new Size(640, 480);

        /// <summary>
        /// For every handle, `moves` seeded targets, with options all off then all on; one row per
        /// resulting point. A fresh posture is parsed for each move so moves are independent.
        /// Segment handles grab their segment's middle point.
        /// </summary>
        private static void WriteMoves(string file, GenericPosture reference, FixtureCase fixtureCase, string outDir)
        {
            int moves = fixtureCase.Input("moves", 3);
            Random random = new Random(fixtureCase.Seed);
            CalibrationHelper calibration = new CalibrationHelper();
            calibration.Initialize(imageSize, t => PointF.Empty, (t, type, drawing) => new QuadrilateralF(), drawing => false);
            string[] header = { "handle", "options_on", "move", "target_x", "target_y", "point", "x", "y" };
            List<IList<string>> rows = new List<IList<string>>();
            for (int handle = 0; handle < reference.Handles.Count; handle++)
            {
                for (int move = 0; move < moves; move++)
                {
                    PointF target = new PointF((float)(random.NextDouble() * imageSize.Width), (float)(random.NextDouble() * imageSize.Height));
                    for (int optionsOn = 0; optionsOn < 2; optionsOn++)
                    {
                        GenericPosture posture = new GenericPosture(file, false, false);
                        foreach (GenericPostureOption option in posture.Options.Values)
                            option.Value = optionsOn == 1;
                        GenericPostureHandle h = posture.Handles[handle];
                        if (h.Type == HandleType.Segment)
                        {
                            GenericPostureSegment s = posture.Segments[h.Reference];
                            h.GrabPoint = GeometryHelper.GetMiddlePoint(posture.PointList[s.Start], posture.PointList[s.End]);
                        }
                        GenericPostureConstraintEngine.MoveHandle(posture, calibration, handle, target, Keys.None);
                        for (int i = 0; i < posture.PointList.Count; i++)
                            rows.Add(CsvRows.Row(CsvRows.S(handle, optionsOn, move, target.X, target.Y, i, posture.PointList[i].X, posture.PointList[i].Y)));
                        for (int i = 0; i < posture.Circles.Count; i++)
                            rows.Add(CsvRows.Row(CsvRows.S(handle, optionsOn, move, target.X, target.Y, "circle" + i, posture.Circles[i].Radius, 0)));
                    }
                }
            }
            Oracle.WriteCsv(Path.Combine(outDir, "moves.csv"), header, rows);
        }
    }
}
