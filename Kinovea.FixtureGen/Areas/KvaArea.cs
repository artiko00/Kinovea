using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Text.RegularExpressions;
using System.Xml.Linq;
using Kinovea.ScreenManager;
using Kinovea.Services;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Loads a KVA through the original deserializer, saves it back and records a summary.
    /// inputs: { "kva": "<path relative to repo root>" }
    /// </summary>
    public class KvaArea : IFixtureArea
    {
        public string Name { get { return "kva"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            string repoRoot = Oracle.RepoRoot(Path.GetDirectoryName(Path.GetDirectoryName(fixtureCase.Path)));
            string source = Path.Combine(repoRoot, fixtureCase.Input("kva", "").Replace('/', Path.DirectorySeparatorChar));
            if (!File.Exists(source))
                throw new FileNotFoundException("KVA input not found", source);

            Metadata metadata = new Metadata(new HistoryStack(), TimecodeBuilder);
            ApplyVideoContext(metadata, source);

            MetadataSerializer serializer = new MetadataSerializer();
            serializer.Load(metadata, source, true);

            string roundtrip = Path.Combine(outDir, "roundtrip.kva");
            serializer.SaveToFile(metadata, roundtrip);
            Oracle.NormalizeTextFile(roundtrip);
            Oracle.NormalizeIds(roundtrip);

            int drawings = 0;
            foreach (Keyframe kf in metadata.Keyframes)
                drawings += kf.Drawings.Count;

            Oracle.WriteText(Path.Combine(outDir, "summary.json"),
                "{\n" +
                "  \"attached_drawings\": " + drawings + ",\n" +
                "  \"image_height\": " + metadata.ImageSize.Height + ",\n" +
                "  \"image_width\": " + metadata.ImageSize.Width + ",\n" +
                "  \"keyframes\": " + metadata.Count + ",\n" +
                "  \"tracks\": " + metadata.Tracks().Count + "\n" +
                "}\n");
        }

        /// <summary>
        /// The player normally sets these from the opened video. Take them from the KVA header
        /// so the import behaves as "same video" and no coordinate rescaling is applied.
        /// </summary>
        private static void ApplyVideoContext(Metadata metadata, string source)
        {
            XElement root = XDocument.Load(source).Root;
            XElement size = root.Element("ImageSize");
            if (size != null)
            {
                string[] parts = size.Value.Split(';');
                metadata.ImageSize = new Size(int.Parse(parts[0], Oracle.Culture), int.Parse(parts[1], Oracle.Culture));
            }

            XElement atpf = root.Element("AverageTimeStampsPerFrame");
            metadata.AverageTimeStampsPerFrame = atpf == null ? 1 : double.Parse(atpf.Value, Oracle.Culture);
        }

        /// <summary>
        /// The original importer assigns a fresh Guid to every keyframe or drawing that has no id in
        /// the source file (KVA 2.0 and older). Replace each distinct non-empty Guid, in order of first
        /// appearance, by a stable stand-in so the fixture is deterministic while cross-references hold.
        /// </summary>

        private static string TimecodeBuilder(long timestamp, TimeType type, TimecodeFormat format, bool symbol)
        {
            return timestamp.ToString(Oracle.Culture);
        }
    }
}
