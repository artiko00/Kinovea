using System.Collections.Generic;
using System.IO;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen
{
    /// <summary>
    /// One input case: tests/cases/&lt;area&gt;/&lt;name&gt;.json.
    /// </summary>
    public class FixtureCase
    {
        public string Area { get; private set; }
        public int Seed { get; private set; }
        public JObject Inputs { get; private set; }
        public string Name { get; private set; }
        public string Path { get; private set; }
        public byte[] Bytes { get; private set; }

        public static List<FixtureCase> LoadAll(string casesDir)
        {
            List<FixtureCase> cases = new List<FixtureCase>();
            if (!Directory.Exists(casesDir))
                return cases;

            string[] files = Directory.GetFiles(casesDir, "*.json", SearchOption.AllDirectories);
            System.Array.Sort(files, System.StringComparer.Ordinal);
            foreach (string file in files)
                cases.Add(Load(file));

            return cases;
        }

        private static FixtureCase Load(string file)
        {
            byte[] bytes = File.ReadAllBytes(file);
            JObject root = JObject.Parse(System.Text.Encoding.UTF8.GetString(bytes));
            return new FixtureCase
            {
                Area = (string)root["area"] ?? "",
                Seed = root["seed"] == null ? 0 : (int)root["seed"],
                Inputs = (JObject)root["inputs"] ?? new JObject(),
                Name = System.IO.Path.GetFileNameWithoutExtension(file),
                Path = file,
                Bytes = bytes,
            };
        }

        public T Input<T>(string key, T fallback)
        {
            JToken token = Inputs[key];
            return token == null ? fallback : token.ToObject<T>();
        }
    }
}
