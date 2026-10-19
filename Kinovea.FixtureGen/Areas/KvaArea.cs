using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Xml.Linq;
using Kinovea.ScreenManager;
using Kinovea.Services;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// KVA round trips through the original serializer, plus the fuzz, fading, migration,
    /// capture-recording and cross-load variants selected by the `kind` input.
    /// </summary>
    public partial class KvaArea : IFixtureArea
    {
        public string Name { get { return "kva"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            string repoRoot = Oracle.RepoRoot(Path.GetDirectoryName(Path.GetDirectoryName(fixtureCase.Path)));
            switch (fixtureCase.Input("kind", "roundtrip"))
            {
                case "fuzz": Fuzz(fixtureCase, outDir); break;
                case "fading": Fading(outDir); break;
                case "migration": Migration(fixtureCase, repoRoot, outDir); break;
                case "capture": Capture(fixtureCase, repoRoot, outDir); break;
                case "crossload": CrossLoad(fixtureCase, repoRoot, outDir); break;
                default: RoundTrip(fixtureCase, repoRoot, outDir); break;
            }
        }

        private static string Source(FixtureCase fixtureCase, string repoRoot, string key)
        {
            string source = Path.Combine(repoRoot, fixtureCase.Input(key, "").Replace('/', Path.DirectorySeparatorChar));
            if (!File.Exists(source))
                throw new FileNotFoundException("KVA input not found", source);
            return source;
        }

        private static void RoundTrip(FixtureCase fixtureCase, string repoRoot, string outDir)
        {
            string source = Source(fixtureCase, repoRoot, "kva");
            Metadata metadata = Load(source, fixtureCase.Inputs["context"] as JObject, fixtureCase.Path);
            Save(metadata, Path.Combine(outDir, "roundtrip.kva"));
            WriteSummary(metadata, outDir);
        }

        /// <summary>
        /// Loads a KVA into a metadata whose video context either mirrors the file header or
        /// comes from the case (`context`: image size, timing, and a real file as the video path so
        /// the original decides the file is a different video and remaps everything).
        /// </summary>
        public static Metadata Load(string source, JObject context, string videoPathForContext)
        {
            Metadata metadata = NewMetadata();
            if (context == null)
                ApplyVideoContext(metadata, source);
            else
                ApplyContext(metadata, context, videoPathForContext);
            // What the player does once the video is open (FrameServerPlayer.Load): singletons go
            // through AfterDrawingCreation (test grid scaled to the image, coordinate system
            // registered for trackability). KVA files are loaded after that.
            metadata.PostSetupVideo(true);
            new MetadataSerializer().Load(metadata, source, true);
            return metadata;
        }

        public static Metadata NewMetadata()
        {
            Metadata metadata = new Metadata(new HistoryStack(), TimecodeBuilder);
            metadata.TrackableDrawingAdded += (s, e) => metadata.TrackabilityManager.Add(e.Value, e.Value.ReferenceTimestamp);
            // The original never assigns these two; importing a KVA with Spotlights or a
            // NumberSequence then throws and aborts the import at that element.
            metadata.DrawingSpotlight.ParentMetadata = metadata;
            metadata.DrawingNumberSequence.ParentMetadata = metadata;
            return metadata;
        }

        public static void Save(Metadata metadata, string path, bool isCaptureRecording = false, KVAExportFlags flags = KVAExportFlags.Full)
        {
            new MetadataSerializer().SaveToFile(metadata, path, isCaptureRecording, flags);
            Oracle.NormalizeTextFile(path);
            Oracle.NormalizeIds(path);
        }

        private static void ApplyVideoContext(Metadata metadata, string source)
        {
            XElement root = XDocument.Load(source).Root;
            XElement size = root.Element("ImageSize");
            if (size != null)
                metadata.ImageSize = ParseSize(size.Value);
            XElement atpf = root.Element("AverageTimeStampsPerFrame");
            metadata.AverageTimeStampsPerFrame = atpf == null ? 1 : double.Parse(atpf.Value, Oracle.Culture);
        }

        private static void ApplyContext(Metadata metadata, JObject context, string videoPath)
        {
            metadata.ImageSize = ParseSize((string)context["imageSize"]);
            metadata.AverageTimeStampsPerFrame = (double)context["averageTimeStampsPerFrame"];
            metadata.AverageTimeStampsPerSecond = (double)context["averageTimeStampsPerSecond"];
            metadata.BaselineFrameInterval = (double)context["baselineFrameInterval"];
            metadata.FirstTimeStamp = (long)context["firstTimeStamp"];
            metadata.TimeOrigin = (long)context["timeOrigin"];
            metadata.VideoPath = videoPath;
        }

        private static Size ParseSize(string text)
        {
            string[] parts = text.Split(';');
            return new Size(int.Parse(parts[0], Oracle.Culture), int.Parse(parts[1], Oracle.Culture));
        }

        private static string TimecodeBuilder(long timestamp, TimeType type, TimecodeFormat format, bool symbol)
        {
            return timestamp.ToString(Oracle.Culture);
        }
    }
}
