using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Generic posture tools: the parsed description (`summary.json`) and the constraint engine
    /// moving every handle to seeded targets (`moves.csv`).
    /// </summary>
    public partial class PostureArea : IFixtureArea
    {
        public string Name { get { return "posture"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            string repoRoot = Oracle.RepoRoot(Path.GetDirectoryName(Path.GetDirectoryName(fixtureCase.Path)));
            string file = Path.Combine(repoRoot, fixtureCase.Input("file", "").Replace('/', Path.DirectorySeparatorChar));
            if (!File.Exists(file))
                throw new FileNotFoundException("posture file not found", file);
            GenericPosture posture = new GenericPosture(file, false, false);
            WriteSummary(posture, outDir);
            WriteMoves(file, posture, fixtureCase, outDir);
        }

        private static void WriteSummary(GenericPosture p, string outDir)
        {
            StringBuilder sb = new StringBuilder("{\n");
            sb.Append("  \"id\": \"").Append(p.Id).Append("\",\n");
            sb.Append("  \"name\": \"").Append(p.Name).Append("\",\n");
            sb.Append("  \"display_name\": \"").Append(p.DisplayName).Append("\",\n");
            sb.Append("  \"points\": ").Append(p.PointList.Count).Append(",\n");
            sb.Append("  \"segments\": ").Append(p.Segments.Count).Append(",\n");
            sb.Append("  \"circles\": ").Append(p.Circles.Count).Append(",\n");
            sb.Append("  \"angles\": ").Append(p.Angles.Count).Append(",\n");
            sb.Append("  \"polylines\": ").Append(p.Polylines.Count).Append(",\n");
            sb.Append("  \"handles\": ").Append(p.Handles.Count).Append(",\n");
            sb.Append("  \"hit_zones\": ").Append(p.HitZones.Count).Append(",\n");
            sb.Append("  \"distances\": ").Append(p.Distances.Count).Append(",\n");
            sb.Append("  \"positions\": ").Append(p.Positions.Count).Append(",\n");
            sb.Append("  \"computed_points\": ").Append(p.ComputedPoints.Count).Append(",\n");
            sb.Append("  \"options\": ").Append(p.Options.Count).Append(",\n");
            sb.Append("  \"has_non_hidden_options\": ").Append(p.HasNonHiddenOptions ? "true" : "false").Append(",\n");
            sb.Append("  \"trackable\": ").Append(p.Trackable ? "true" : "false").Append(",\n");
            sb.Append("  \"flip_horizontal\": ").Append((p.Capabilities & GenericPostureCapabilities.FlipHorizontal) != 0 ? "true" : "false").Append(",\n");
            sb.Append("  \"flip_vertical\": ").Append((p.Capabilities & GenericPostureCapabilities.FlipVertical) != 0 ? "true" : "false").Append(",\n");
            sb.Append("  \"trackable_points\": [").Append(string.Join(", ", p.GetTrackablePoints().Keys.Select(k => "\"" + k + "\""))).Append("],\n");
            sb.Append("  \"point_names\": [").Append(string.Join(", ", p.Points.Select(pt => "\"" + pt.Name + "\""))).Append("],\n");
            sb.Append("  \"point_list\": [").Append(string.Join(", ", p.PointList.Select(pt => "\"" + Oracle.R(pt.X) + ";" + Oracle.R(pt.Y) + "\""))).Append("],\n");
            sb.Append("  \"computed\": [").Append(string.Join(", ", p.ComputedPoints.Select(c => { PointF l = c.ComputeLocation(p); return "\"" + Oracle.R(l.X) + ";" + Oracle.R(l.Y) + "\""; }))).Append("],\n");
            sb.Append("  \"option_keys\": [").Append(string.Join(", ", p.Options.Keys.Select(k => "\"" + k + "\""))).Append("]\n");
            sb.Append("}\n");
            Oracle.WriteText(Path.Combine(outDir, "summary.json"), sb.ToString());
        }
    }
}
