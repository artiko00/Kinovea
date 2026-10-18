using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using Kinovea.ScreenManager;
using Kinovea.Services;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Tables of unit abbreviations and conversions from UnitHelper.
    /// inputs: { "magnitudes": [..], "pixelSizes": [[world1, world2, pixel1, pixel2, "LengthUnit"], ..] }
    /// </summary>
    public class UnitsArea : IFixtureArea
    {
        public string Name { get { return "units"; } }

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            List<double> magnitudes = fixtureCase.Input("magnitudes", new List<double> { 0, 1, 10 });
            List<List<string>> pixelSizes = fixtureCase.Input("pixelSizes", new List<List<string>>());

            WriteAbbreviations(Path.Combine(outDir, "abbreviations.csv"));

            List<IList<string>> rows = new List<IList<string>>();
            foreach (double m in magnitudes)
                foreach (LengthUnit lu in Enum.GetValues(typeof(LengthUnit)))
                    foreach (SpeedUnit su in Enum.GetValues(typeof(SpeedUnit)))
                        rows.Add(new[] { Oracle.R(m), lu.ToString(), su.ToString(), Oracle.R(UnitHelper.ConvertVelocity((float)m, lu, su)) });
            Oracle.WriteCsv(Path.Combine(outDir, "velocity.csv"), new[] { "input", "length_unit", "speed_unit", "output" }, rows);

            rows = new List<IList<string>>();
            foreach (double m in magnitudes)
                foreach (LengthUnit lu in Enum.GetValues(typeof(LengthUnit)))
                    foreach (AccelerationUnit au in Enum.GetValues(typeof(AccelerationUnit)))
                        rows.Add(new[] { Oracle.R(m), lu.ToString(), au.ToString(), Oracle.R(UnitHelper.ConvertAcceleration((float)m, lu, au)) });
            Oracle.WriteCsv(Path.Combine(outDir, "acceleration.csv"), new[] { "input", "length_unit", "acceleration_unit", "output" }, rows);

            rows = new List<IList<string>>();
            foreach (double m in magnitudes)
                foreach (SpeedUnit su in Enum.GetValues(typeof(SpeedUnit)))
                    foreach (LengthUnit lu in Enum.GetValues(typeof(LengthUnit)))
                        rows.Add(new[] { Oracle.R(m), su.ToString(), lu.ToString(), Oracle.R(UnitHelper.ConvertForLengthUnit((float)m, su, lu)) });
            Oracle.WriteCsv(Path.Combine(outDir, "length_from_speed.csv"), new[] { "input", "speed_unit", "length_unit", "output" }, rows);

            rows = new List<IList<string>>();
            foreach (double m in magnitudes)
                foreach (AngularVelocityUnit u in Enum.GetValues(typeof(AngularVelocityUnit)))
                    rows.Add(new[] { Oracle.R(m), u.ToString(), Oracle.R(UnitHelper.ConvertAngularVelocity(m, u)) });
            Oracle.WriteCsv(Path.Combine(outDir, "angular_velocity.csv"), new[] { "input", "unit", "output" }, rows);

            rows = new List<IList<string>>();
            foreach (double m in magnitudes)
                foreach (AngularAccelerationUnit u in Enum.GetValues(typeof(AngularAccelerationUnit)))
                    rows.Add(new[] { Oracle.R(m), u.ToString(), Oracle.R(UnitHelper.ConvertAngularAcceleration((float)m, u)) });
            Oracle.WriteCsv(Path.Combine(outDir, "angular_acceleration.csv"), new[] { "input", "unit", "output" }, rows);

            rows = new List<IList<string>>();
            foreach (double m in magnitudes)
                foreach (CadenceUnit u in Enum.GetValues(typeof(CadenceUnit)))
                    rows.Add(new[] { Oracle.R(m), u.ToString(), Oracle.R(UnitHelper.ConvertFrequency((float)m, u)) });
            Oracle.WriteCsv(Path.Combine(outDir, "frequency.csv"), new[] { "input", "unit", "output" }, rows);

            rows = new List<IList<string>>();
            foreach (List<string> ps in pixelSizes)
            {
                float w1 = float.Parse(ps[0], CultureInfo.InvariantCulture);
                float w2 = float.Parse(ps[1], CultureInfo.InvariantCulture);
                float p1 = float.Parse(ps[2], CultureInfo.InvariantCulture);
                float p2 = float.Parse(ps[3], CultureInfo.InvariantCulture);
                LengthUnit lu = (LengthUnit)Enum.Parse(typeof(LengthUnit), ps[4]);
                rows.Add(new[] { Oracle.R(w1), Oracle.R(w2), Oracle.R(p1), Oracle.R(p2), lu.ToString(), Oracle.Csv(UnitHelper.GetPixelSize(w1, w2, p1, p2, lu)) });
            }
            Oracle.WriteCsv(Path.Combine(outDir, "pixel_size.csv"), new[] { "world1", "world2", "pixel1", "pixel2", "unit", "output" }, rows);
        }

        private static void WriteAbbreviations(string path)
        {
            List<IList<string>> rows = new List<IList<string>>();
            foreach (LengthUnit u in Enum.GetValues(typeof(LengthUnit))) rows.Add(new[] { "Length", u.ToString(), UnitHelper.LengthAbbreviation(u) });
            foreach (SpeedUnit u in Enum.GetValues(typeof(SpeedUnit))) rows.Add(new[] { "Speed", u.ToString(), UnitHelper.SpeedAbbreviation(u) });
            foreach (AccelerationUnit u in Enum.GetValues(typeof(AccelerationUnit))) rows.Add(new[] { "Acceleration", u.ToString(), UnitHelper.AccelerationAbbreviation(u) });
            foreach (AngleUnit u in Enum.GetValues(typeof(AngleUnit))) rows.Add(new[] { "Angle", u.ToString(), UnitHelper.AngleAbbreviation(u) });
            foreach (AngularVelocityUnit u in Enum.GetValues(typeof(AngularVelocityUnit))) rows.Add(new[] { "AngularVelocity", u.ToString(), UnitHelper.AngularVelocityAbbreviation(u) });
            foreach (AngularAccelerationUnit u in Enum.GetValues(typeof(AngularAccelerationUnit))) rows.Add(new[] { "AngularAcceleration", u.ToString(), UnitHelper.AngularAccelerationAbbreviation(u) });
            foreach (CadenceUnit u in Enum.GetValues(typeof(CadenceUnit))) rows.Add(new[] { "Cadence", u.ToString(), UnitHelper.FrequencyAbbreviation(u) });
            foreach (TimecodeFormat u in Enum.GetValues(typeof(TimecodeFormat)))
            {
                if (u == TimecodeFormat.Unknown || u == TimecodeFormat.NumberOfTimeCodeFormats)
                    continue;
                rows.Add(new[] { "Time", u.ToString(), UnitHelper.TimeAbbreviation(u) });
            }
            Oracle.WriteCsv(path, new[] { "kind", "unit", "abbreviation" }, rows);
        }
    }
}
