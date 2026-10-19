using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using System.Xml.Serialization;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    public partial class KvaArea
    {
        /// <summary>
        /// summary.json (counts) and drawings.json (element name, id and name of every drawing, per
        /// container, in storage order). `ContentHash` is left out: it mixes per-process hashes
        /// of strings and GDI+ objects, so it differs between two runs.
        /// </summary>
        private static void WriteSummary(Metadata metadata, string outDir)
        {
            int drawings = 0;
            foreach (Keyframe kf in metadata.Keyframes)
                drawings += kf.Drawings.Count;
            Oracle.WriteText(Path.Combine(outDir, "summary.json"),
                "{\n" +
                "  \"attached_drawings\": " + drawings + ",\n" +
                "  \"chronos\": " + metadata.ChronoManager.Drawings.Count + ",\n" +
                "  \"image_height\": " + metadata.ImageSize.Height + ",\n" +
                "  \"image_width\": " + metadata.ImageSize.Width + ",\n" +
                "  \"keyframes\": " + metadata.Count + ",\n" +
                "  \"time_origin\": " + metadata.TimeOrigin + ",\n" +
                "  \"tracks\": " + metadata.Tracks().Count + "\n" +
                "}\n");

            StringBuilder sb = new StringBuilder("{\n  \"keyframes\": [\n");
            for (int i = 0; i < metadata.Count; i++)
            {
                Keyframe kf = metadata[i];
                sb.Append("    { \"id\": \"").Append(kf.Id).Append("\", \"timestamp\": ").Append(kf.Timestamp).Append(", \"drawings\": [\n");
                AppendDrawings(sb, kf.Drawings, "      ");
                sb.Append("    ] }").Append(i + 1 < metadata.Count ? ",\n" : "\n");
            }
            sb.Append("  ],\n  \"chronos\": [\n");
            AppendDrawings(sb, metadata.ChronoManager.Drawings, "    ");
            sb.Append("  ],\n  \"tracks\": [\n");
            AppendDrawings(sb, metadata.TrackManager.Drawings, "    ");
            sb.Append("  ]\n}\n");
            string path = Path.Combine(outDir, "drawings.json");
            Oracle.WriteText(path, sb.ToString());
            Oracle.NormalizeIds(path);
        }

        private static void AppendDrawings(StringBuilder sb, IList<AbstractDrawing> drawings, string indent)
        {
            for (int i = 0; i < drawings.Count; i++)
            {
                AbstractDrawing d = drawings[i];
                sb.Append(indent).Append("{ \"element\": \"").Append(ElementName(d)).Append("\", \"id\": \"").Append(d.Id)
                  .Append("\", \"name\": \"").Append(d.Name.Replace("\"", "\\\"")).Append("\" }")
                  .Append(i + 1 < drawings.Count ? ",\n" : "\n");
            }
        }

        private static string ElementName(AbstractDrawing drawing)
        {
            object[] attributes = drawing.GetType().GetCustomAttributes(typeof(XmlTypeAttribute), false);
            return attributes.Length == 0 ? drawing.GetType().Name : ((XmlTypeAttribute)attributes[0]).TypeName;
        }
    }
}
