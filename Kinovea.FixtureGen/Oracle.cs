using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Security.Cryptography;
using System.Text;
using System.Threading;
using System.Windows.Forms;
using Kinovea.ScreenManager;
using Kinovea.Services;

namespace Kinovea.FixtureGen
{
    /// <summary>
    /// Bootstraps the original Kinovea runtime headlessly and offers deterministic writers.
    /// </summary>
    public static class Oracle
    {
        public static readonly UTF8Encoding Utf8NoBom = new UTF8Encoding(false);
        public static readonly CultureInfo Culture = CultureInfo.InvariantCulture;

        /// <summary>
        /// Mirrors Kinovea/Program.cs and Kernel.cs initialization without UI.
        /// Version is fixed so the KVA "Producer" element never churns between runs.
        /// </summary>
        public static void Initialize()
        {
            CultureInfo.DefaultThreadCurrentCulture = Culture;
            CultureInfo.DefaultThreadCurrentUICulture = Culture;
            Thread.CurrentThread.CurrentCulture = Culture;
            Thread.CurrentThread.CurrentUICulture = Culture;

            Directory.CreateDirectory(Path.Combine(Application.StartupPath, "AppData"));
            Software.Initialize(new Version(0, 0, 0, 0));
            Software.SanityCheckDirectories();
            PreferencesManager.Initialize();
            PreferencesManager.Refresh();
            ToolManager.LoadTools();
        }

        public static string RepoRoot(string casesDir)
        {
            // tests/cases -> repository root.
            return Path.GetFullPath(Path.Combine(casesDir, "..", ".."));
        }

        public static string GitCommit(string repoRoot)
        {
            ProcessStartInfo psi = new ProcessStartInfo("git", "rev-parse HEAD")
            {
                WorkingDirectory = repoRoot,
                RedirectStandardOutput = true,
                UseShellExecute = false,
                CreateNoWindow = true,
            };
            using (Process p = Process.Start(psi))
            {
                string output = p.StandardOutput.ReadToEnd().Trim();
                p.WaitForExit();
                return p.ExitCode == 0 ? output : "unknown";
            }
        }

        public static string FFmpegVersion(string repoRoot)
        {
            string dir = Path.Combine(repoRoot, "Refs", "FFmpeg");
            if (!Directory.Exists(dir))
                return "unknown";

            foreach (string sub in Directory.GetDirectories(dir, "ffmpeg-*"))
            {
                string[] parts = Path.GetFileName(sub).Split('-');
                if (parts.Length > 1)
                    return parts[1];
            }

            return "unknown";
        }

        public static string Sha256Hex(byte[] bytes)
        {
            using (SHA256 sha = SHA256.Create())
            {
                StringBuilder sb = new StringBuilder(64);
                foreach (byte b in sha.ComputeHash(bytes))
                    sb.Append(b.ToString("x2"));
                return sb.ToString();
            }
        }

        public static void WriteManifest(string outDir, string commit, string ffmpegVersion, FixtureCase fixtureCase)
        {
            string json = "{\n" +
                "  \"case_sha256\": \"" + Sha256Hex(fixtureCase.Bytes) + "\",\n" +
                "  \"commit\": \"" + commit + "\",\n" +
                "  \"ffmpeg_version\": \"" + ffmpegVersion + "\"\n" +
                "}\n";
            WriteText(Path.Combine(outDir, "manifest.json"), json);
        }

        /// <summary>
        /// Writes UTF-8 without BOM and LF line endings.
        /// </summary>
        public static void WriteText(string path, string content)
        {
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            File.WriteAllText(path, content.Replace("\r\n", "\n"), Utf8NoBom);
        }

        /// <summary>
        /// Re-writes a file produced by the original code with normalized encoding and line endings.
        /// </summary>
        public static void NormalizeTextFile(string path)
        {
            string text = File.ReadAllText(path, Encoding.UTF8);
            WriteText(path, text);
        }

        public static string R(double value)
        {
            return value.ToString("R", Culture);
        }

        public static void WriteCsv(string path, IList<string> header, IEnumerable<IList<string>> rows)
        {
            StringBuilder sb = new StringBuilder();
            sb.Append(string.Join(",", header)).Append('\n');
            foreach (IList<string> row in rows)
                sb.Append(string.Join(",", row)).Append('\n');
            WriteText(path, sb.ToString());
        }

        public static string Csv(string value)
        {
            return value.IndexOfAny(new[] { ',', '"', '\n' }) >= 0 ? "\"" + value.Replace("\"", "\"\"") + "\"" : value;
        }
    }
}
