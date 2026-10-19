using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Kinovea.FixtureGen.Areas;

namespace Kinovea.FixtureGen
{
    /// <summary>
    /// Kinovea.FixtureGen.exe --cases &lt;dir&gt; --out &lt;dir&gt; [--verify]
    /// Exit codes: 0 ok, 1 generation error, 2 usage, 3 unsupported area, 4 non-deterministic output.
    /// </summary>
    public static class Program
    {
        private static readonly Dictionary<string, IFixtureArea> areas = new IFixtureArea[]
        {
            new KvaArea(),
            new KinematicsArea(),
            new TimecodeArea(),
            new UnitsArea(),
            new PreferencesArea(),
            new GeometryArea(),
            new CalibrationArea(),
            new PostureArea(),
            new VideoCacheArea(),
            new TrackingArea(),
            new ExportArea(),
            new ImportArea(),
        }.ToDictionary(a => a.Name);

        public static int Main(string[] args)
        {
            string casesDir = null, outDir = null;
            bool verify = false;
            for (int i = 0; i < args.Length; i++)
            {
                if (args[i] == "--cases" && i + 1 < args.Length) casesDir = args[++i];
                else if (args[i] == "--out" && i + 1 < args.Length) outDir = args[++i];
                else if (args[i] == "--verify") verify = true;
                else { Console.Error.WriteLine("unknown argument: " + args[i]); return Usage(); }
            }

            if (casesDir == null || outDir == null)
                return Usage();

            casesDir = Path.GetFullPath(casesDir);
            outDir = Path.GetFullPath(outDir);

            List<FixtureCase> cases = FixtureCase.LoadAll(casesDir);
            foreach (FixtureCase c in cases.Where(c => !areas.ContainsKey(c.Area)))
            {
                Console.Error.WriteLine("unsupported area '" + c.Area + "' in " + c.Path + ". Supported: " + string.Join(", ", areas.Keys));
                return 3;
            }

            Oracle.Initialize();
            string repoRoot = Oracle.RepoRoot(casesDir);
            string commit = Oracle.GitCommit(repoRoot);
            string ffmpeg = Oracle.FFmpegVersion(repoRoot);

            try
            {
                Generate(cases, repoRoot, outDir, commit, ffmpeg);
                if (!verify)
                    return 0;

                string second = outDir + "-verify";
                Generate(cases, repoRoot, second, commit, ffmpeg);
                List<string> diffs = Compare(outDir, second);
                Directory.Delete(second, true);
                if (diffs.Count == 0)
                {
                    Console.WriteLine("deterministic: identical trees");
                    return 0;
                }

                Console.Error.WriteLine("non-deterministic output:");
                diffs.ForEach(d => Console.Error.WriteLine("  " + d));
                return 4;
            }
            catch (Exception e)
            {
                Console.Error.WriteLine(e);
                return 1;
            }
        }

        private static void Generate(List<FixtureCase> cases, string repoRoot, string outDir, string commit, string ffmpeg)
        {
            if (Directory.Exists(outDir))
                Directory.Delete(outDir, true);

            foreach (FixtureCase c in cases)
            {
                string caseOut = Path.Combine(outDir, c.Area, c.Name);
                Directory.CreateDirectory(caseOut);
                Console.WriteLine("[" + c.Area + "] " + c.Name);
                areas[c.Area].Generate(c, caseOut);
                Oracle.WriteManifest(caseOut, commit, ffmpeg, c);
            }
        }

        private static List<string> Compare(string a, string b)
        {
            Func<string, Dictionary<string, string>> index = root =>
                Directory.GetFiles(root, "*", SearchOption.AllDirectories)
                    .ToDictionary(f => f.Substring(root.Length).TrimStart('\\', '/'), f => f);

            Dictionary<string, string> fa = index(a), fb = index(b);
            List<string> diffs = new List<string>();
            foreach (string rel in fa.Keys.Union(fb.Keys).OrderBy(k => k, StringComparer.Ordinal))
            {
                if (!fa.ContainsKey(rel) || !fb.ContainsKey(rel))
                    diffs.Add(rel + " (missing in one run)");
                else if (!File.ReadAllBytes(fa[rel]).SequenceEqual(File.ReadAllBytes(fb[rel])))
                    diffs.Add(rel + " (content differs)");
            }

            return diffs;
        }

        private static int Usage()
        {
            Console.Error.WriteLine("usage: Kinovea.FixtureGen --cases <dir> --out <dir> [--verify]");
            return 2;
        }
    }
}
