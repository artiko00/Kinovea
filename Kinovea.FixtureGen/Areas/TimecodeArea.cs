using System;
using System.Collections.Generic;
using System.IO;
using Kinovea.Services;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Formats a list of times with every TimecodeFormat and with MillisecondsToTimecode.
    /// inputs: { "milliseconds": [..], "framerates": [..], "precisions": [..], "durationSeconds": n }
    /// </summary>
    public class TimecodeArea : IFixtureArea
    {
        public string Name { get { return "timecode"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            List<long> milliseconds = fixtureCase.Input("milliseconds", new List<long>());
            List<double> framerates = fixtureCase.Input("framerates", new List<double> { 25 });
            List<int> precisions = fixtureCase.Input("precisions", new List<int> { 2, 3 });
            double durationSeconds = fixtureCase.Input("durationSeconds", 10.0);

            List<IList<string>> rows = new List<IList<string>>();
            foreach (double fps in framerates)
            {
                double totalFrames = Math.Round(durationSeconds * fps);
                foreach (long ms in milliseconds)
                {
                    int frames = (int)Math.Round(ms * fps / 1000.0);
                    foreach (TimecodeFormat format in Enum.GetValues(typeof(TimecodeFormat)))
                    {
                        if (format == TimecodeFormat.Unknown || format == TimecodeFormat.NumberOfTimeCodeFormats)
                            continue;

                        foreach (bool symbol in new[] { false, true })
                        {
                            string output = TimeHelper.GetTimestring(fps, frames, ms, frames, totalFrames, totalFrames, format, symbol);
                            rows.Add(new[] { Oracle.R(fps), ms.ToString(Oracle.Culture), frames.ToString(Oracle.Culture), format.ToString(), symbol ? "1" : "0", Oracle.Csv(output) });
                        }
                    }
                }
            }
            Oracle.WriteCsv(Path.Combine(outDir, "timecodes.csv"), new[] { "framerate", "milliseconds", "frames", "format", "symbol", "output" }, rows);

            List<IList<string>> ms2tc = new List<IList<string>>();
            foreach (long ms in milliseconds)
                foreach (int precision in precisions)
                    foreach (long signed in new[] { ms, -ms })
                        ms2tc.Add(new[] { signed.ToString(Oracle.Culture), precision.ToString(Oracle.Culture), Oracle.Csv(TimeHelper.MillisecondsToTimecode(signed, precision)) });
            Oracle.WriteCsv(Path.Combine(outDir, "ms_to_timecode.csv"), new[] { "milliseconds", "precision", "output" }, ms2tc);
        }
    }
}
