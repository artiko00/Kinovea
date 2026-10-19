using System.IO;
using Kinovea.ScreenManager;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Imports an external file into an empty document with the original importers and saves the KVA.
    /// inputs: "kind" (openpose|trc|srt), "file" (repository path; for OpenPose, one keypoints file of the
    /// folder), "context" (as the kva area). Output: imported.kva, ids normalized as in the kva area.
    /// </summary>
    public class ImportArea : IFixtureArea
    {
        public string Name { get { return "import"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            string repoRoot = Oracle.RepoRoot(Path.GetDirectoryName(Path.GetDirectoryName(fixtureCase.Path)));
            string source = Path.Combine(repoRoot, fixtureCase.Input("file", "").Replace('/', Path.DirectorySeparatorChar));
            Metadata metadata = KvaArea.NewMetadata();
            KvaArea.ApplyContext(metadata, (JObject)fixtureCase.Inputs["context"], fixtureCase.Name + ".mp4");

            switch (fixtureCase.Input("kind", ""))
            {
                case "openpose": MetadataImporterOpenPose.Import(metadata, source, true); break;
                case "trc": MetadataImporterSports2D.Import(metadata, source, true); break;
                case "srt": MetadataImporterSRT.Import(metadata, source, true); break;
                default: throw new System.ArgumentException("unknown import kind in " + fixtureCase.Path);
            }

            Directory.CreateDirectory(outDir);
            KvaArea.Save(metadata, Path.Combine(outDir, "imported.kva"));
        }
    }
}
