using System;
using System.IO;
using Kinovea.ScreenManager;

namespace Kinovea.FixtureGen.Areas
{
    public partial class KvaArea
    {
        /// <summary>
        /// Seeded random KVA files (`fuzz/NN.kva`) and their round trip through the original
        /// (`fuzz/NN.roundtrip.kva`). The generator is `KvaFuzz`, a deterministic rewrite of the
        /// legacy `Kinovea.Tests` fuzzer for the 2.1 element shapes.
        /// </summary>
        private static void Fuzz(FixtureCase fixtureCase, string outDir)
        {
            int count = fixtureCase.Input("count", 20);
            Random random = new Random(fixtureCase.Seed);
            string dir = Path.Combine(outDir, "fuzz");
            Directory.CreateDirectory(dir);
            for (int i = 1; i <= count; i++)
            {
                string input = Path.Combine(dir, i.ToString("D2", Oracle.Culture) + ".kva");
                Oracle.WriteText(input, new KvaFuzz(random).Create());
                Metadata metadata = Load(input, null, fixtureCase.Path);
                Save(metadata, Path.Combine(dir, i.ToString("D2", Oracle.Culture) + ".roundtrip.kva"));
            }
        }
    }
}
