using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using Kinovea.ScreenManager;
using Kinovea.Services;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Runs the six spreadsheet exporters and the Markdown exporter (no images) on a loaded KVA.
    /// inputs: "kva", "context" (as the kva area), "exportSpace" (WorldSpace|ImageSpace),
    /// "csvDecimalSeparator" (System|Point|Comma), "timecodeFormat", "selection" [start, end] (default: all).
    /// The xlsx and ods containers embed their creation time, so only the sheet XML is kept:
    /// measured.xlsx.sheet1.xml, measured.xlsx.strings.xml and measured.ods.content.xml.
    /// </summary>
    public class ExportArea : IFixtureArea
    {
        public string Name { get { return "export"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            PlayerPreferences prefs = PreferencesManager.PlayerPreferences;
            ExportSpace savedSpace = prefs.ExportSpace;
            CSVDecimalSeparator savedSeparator = prefs.CSVDecimalSeparator;
            TimecodeFormat savedFormat = prefs.TimecodeFormat;
            try
            {
                prefs.ExportSpace = Parse(fixtureCase.Input("exportSpace", "WorldSpace"), ExportSpace.WorldSpace);
                prefs.CSVDecimalSeparator = Parse(fixtureCase.Input("csvDecimalSeparator", "Point"), CSVDecimalSeparator.Point);
                prefs.TimecodeFormat = Parse(fixtureCase.Input("timecodeFormat", "ClassicTime"), TimecodeFormat.ClassicTime);
                Run(fixtureCase, outDir);
            }
            finally
            {
                prefs.ExportSpace = savedSpace;
                prefs.CSVDecimalSeparator = savedSeparator;
                prefs.TimecodeFormat = savedFormat;
            }
        }

        private static void Run(FixtureCase fixtureCase, string outDir)
        {
            string repoRoot = Oracle.RepoRoot(Path.GetDirectoryName(Path.GetDirectoryName(fixtureCase.Path)));
            string source = Path.Combine(repoRoot, fixtureCase.Input("kva", "").Replace('/', Path.DirectorySeparatorChar));
            Metadata metadata = KvaArea.Load(source, (JObject)fixtureCase.Inputs["context"], fixtureCase.Name + ".mp4");
            long[] selection = fixtureCase.Input("selection", new[] { long.MinValue, long.MaxValue });
            metadata.SelectionStart = selection[0];
            metadata.SelectionEnd = selection[1];

            MeasuredData md = metadata.CollectMeasuredData();
            Directory.CreateDirectory(outDir);
            new ExporterJSON().Export(Path.Combine(outDir, "measured.json"), md);
            new ExporterCSVTrajectory().Export(Path.Combine(outDir, "trajectory.csv"), md);
            new ExporterCSVChrono().Export(Path.Combine(outDir, "chrono.csv"), md);
            new ExporterTXTTrajectory().Export(Path.Combine(outDir, "trajectory.txt"), md);
            new ExporterMarkdown().Export(Path.Combine(outDir, "report.md"), new List<string>(), metadata);

            string xlsx = Path.Combine(outDir, "measured.xlsx");
            new ExporterXLSX().Export(xlsx, md);
            Extract(xlsx, "xl/worksheets/sheet1.xml", Path.Combine(outDir, "measured.xlsx.sheet1.xml"));
            Extract(xlsx, "xl/sharedStrings.xml", Path.Combine(outDir, "measured.xlsx.strings.xml"));
            File.Delete(xlsx);

            string ods = Path.Combine(outDir, "measured.ods");
            new ExporterODS().Export(ods, md);
            Extract(ods, "content.xml", Path.Combine(outDir, "measured.ods.content.xml"));
            File.Delete(ods);
        }

        private static void Extract(string archive, string entryName, string target)
        {
            using (ZipArchive zip = ZipFile.OpenRead(archive))
            {
                ZipArchiveEntry entry = zip.GetEntry(entryName);
                if (entry != null)
                    entry.ExtractToFile(target, true);
            }
        }

        private static T Parse<T>(string text, T fallback) where T : struct
        {
            T value;
            return System.Enum.TryParse(text, out value) ? value : fallback;
        }
    }
}
