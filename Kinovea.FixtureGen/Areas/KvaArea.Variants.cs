using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using Kinovea.ScreenManager;
using Kinovea.Services;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen.Areas
{
    public partial class KvaArea
    {
        /// <summary>
        /// Legacy files: `converted.kva` is the XSLT output (what `MetadataConverter` feeds to the
        /// reader), `roundtrip.kva` the file saved after loading it (relative track times fixed).
        /// </summary>
        private static void Migration(FixtureCase fixtureCase, string repoRoot, string outDir)
        {
            string source = Source(fixtureCase, repoRoot, "kva");
            bool relativeTrajectories;
            string converted = MetadataConverter.Convert(source, true, out relativeTrajectories);
            string target = Path.Combine(outDir, "converted.kva");
            File.Copy(converted, target, true);
            Oracle.NormalizeTextFile(target);
            Metadata metadata = Load(source, null, fixtureCase.Path);
            Save(metadata, Path.Combine(outDir, "roundtrip.kva"));
            WriteSummary(metadata, outDir);
            Oracle.WriteText(Path.Combine(outDir, "migration.json"), "{\n  \"relative_trajectories\": " + (relativeTrajectories ? "true" : "false") + "\n}\n");
        }

        /// <summary>
        /// The same metadata exported as a capture recording and with the drawings-only flags.
        /// </summary>
        private static void Capture(FixtureCase fixtureCase, string repoRoot, string outDir)
        {
            Metadata metadata = Load(Source(fixtureCase, repoRoot, "kva"), null, fixtureCase.Path);
            Save(metadata, Path.Combine(outDir, "capture.kva"), true, KVAExportFlags.DefaultCaptureRecording);
            Save(metadata, Path.Combine(outDir, "drawings.kva"), false, KVAExportFlags.Drawings);
            Save(metadata, Path.Combine(outDir, "calibration.kva"), false, KVAExportFlags.Calibration);
            Save(metadata, Path.Combine(outDir, "video.kva"), false, KVAExportFlags.VideoSpecific);
        }

        /// <summary>
        /// Files written by the Rust port are loaded by the original and saved again; the output
        /// must be identical to the input for the port to be a fixed point of the original.
        /// </summary>
        private static void CrossLoad(FixtureCase fixtureCase, string repoRoot, string outDir)
        {
            StringBuilder sb = new StringBuilder("[\n");
            JArray files = (JArray)fixtureCase.Inputs["files"];
            for (int i = 0; i < files.Count; i++)
            {
                string relative = (string)files[i];
                string source = Path.Combine(repoRoot, relative.Replace('/', Path.DirectorySeparatorChar));
                if (!File.Exists(source))
                    throw new FileNotFoundException("cross-load input not found", source);
                string name = Path.GetFileNameWithoutExtension(source);
                Metadata metadata = Load(source, null, fixtureCase.Path);
                string target = Path.Combine(outDir, name + ".kva");
                Save(metadata, target);
                int drawings = 0;
                foreach (Keyframe kf in metadata.Keyframes)
                    drawings += kf.Drawings.Count;
                bool identical = File.ReadAllBytes(source).Length == File.ReadAllBytes(target).Length && File.ReadAllText(source) == File.ReadAllText(target);
                sb.Append("  { \"file\": \"").Append(relative).Append("\", \"keyframes\": ").Append(metadata.Count)
                  .Append(", \"attached_drawings\": ").Append(drawings).Append(", \"chronos\": ").Append(metadata.ChronoManager.Drawings.Count)
                  .Append(", \"tracks\": ").Append(metadata.Tracks().Count).Append(", \"identical\": ").Append(identical ? "true" : "false").Append(" }")
                  .Append(i + 1 < files.Count ? ",\n" : "\n");
            }
            sb.Append("]\n");
            Oracle.WriteText(Path.Combine(outDir, "summary.json"), sb.ToString());
        }

        /// <summary>
        /// `InfosFading.GetOpacityFactor` over a grid of configurations and query timestamps.
        /// </summary>
        private static void Fading(string outDir)
        {
            string[] header = { "always_visible", "opaque_frames", "fading_frames", "master_factor", "atpf", "reference", "query", "opacity" };
            List<IList<string>> rows = new List<IList<string>>();
            bool[] always = { false, true };
            int[] opaque = { 1, 5 };
            int[] fading = { 1, 20 };
            float[] master = { 1.0f, 0.4f };
            double[] atpf = { 1.0, 40.0, 33.3667 };
            long[] queries = { -50, -1, 0, 1, 3, 4, 5, 20, 160, 161, 800, 801, 5000 };
            foreach (bool a in always) foreach (int o in opaque) foreach (int f in fading) foreach (float m in master) foreach (double t in atpf)
            {
                InfosFading info = new InfosFading(1000, t);
                info.UseDefault = false;
                info.Enabled = true;
                info.AlwaysVisible = a;
                info.OpaqueFrames = o;
                info.FadingFrames = f;
                info.MasterFactor = m;
                foreach (long q in queries)
                    rows.Add(CsvRows.Row(CsvRows.S(a ? 1 : 0, o, f, m, t, 1000L, 1000 + q, info.GetOpacityFactor(1000 + q))));
            }
            Oracle.WriteCsv(Path.Combine(outDir, "opacity.csv"), header, rows);
        }
    }
}
