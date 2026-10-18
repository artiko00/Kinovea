using System.IO;
using Kinovea.Services;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Writes Preferences.xml as the original PreferencesManager saves it.
    /// inputs: { "input": "relative/path/to/Preferences.xml" } (optional; absent means defaults).
    /// The settings directory is the portable AppData folder next to the generator.
    /// </summary>
    public class PreferencesArea : IFixtureArea
    {
        public string Name { get { return "preferences"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            string input = fixtureCase.Input<string>("input", null);
            string prefsFile = Software.PreferencesFile;

            if (File.Exists(prefsFile))
                File.Delete(prefsFile);

            if (!string.IsNullOrEmpty(input))
            {
                string inputPath = Path.Combine(Path.GetDirectoryName(fixtureCase.Path), input);
                File.Copy(inputPath, prefsFile);
            }

            // A fresh manager imports the file lazily on first read, then exports it unchanged.
            PreferencesManager.Initialize();
            bool touch = PreferencesManager.GeneralPreferences.EnableDebugLog;
            PreferencesManager.Save();

            Directory.CreateDirectory(outDir);
            string output = Path.Combine(outDir, "Preferences.xml");
            File.Copy(prefsFile, output, true);
            Oracle.NormalizeTextFile(output);

            File.Delete(prefsFile);
            PreferencesManager.Initialize();
        }
    }
}
