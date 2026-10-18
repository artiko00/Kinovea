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

            Table2<LengthUnit, SpeedUnit>(outDir, "velocity.csv", "length_unit", "speed_unit", magnitudes,
                (m, lu, su) => UnitHelper.ConvertVelocity((float)m, lu, su));
            Table2<LengthUnit, AccelerationUnit>(outDir, "acceleration.csv", "length_unit", "acceleration_unit", magnitudes,
                (m, lu, au) => UnitHelper.ConvertAcceleration((float)m, lu, au));
            Table2<SpeedUnit, LengthUnit>(outDir, "length_from_speed.csv", "speed_unit", "length_unit", magnitudes,
                (m, su, lu) => UnitHelper.ConvertForLengthUnit((float)m, su, lu));
            Table1<AngularVelocityUnit>(outDir, "angular_velocity.csv", magnitudes, (m, u) => UnitHelper.ConvertAngularVelocity(m, u));
            Table1<AngularAccelerationUnit>(outDir, "angular_acceleration.csv", magnitudes, (m, u) => UnitHelper.ConvertAngularAcceleration((float)m, u));
            Table1<CadenceUnit>(outDir, "frequency.csv", magnitudes, (m, u) => UnitHelper.ConvertFrequency((float)m, u));

            List<IList<string>> rows = new List<IList<string>>();
            foreach (List<string> ps in pixelSizes)
            {
                float[] v = new float[4];
                for (int i = 0; i < 4; i++)
                    v[i] = float.Parse(ps[i], CultureInfo.InvariantCulture);
                LengthUnit lu = (LengthUnit)Enum.Parse(typeof(LengthUnit), ps[4]);
                string text = UnitHelper.GetPixelSize(v[0], v[1], v[2], v[3], lu);
                rows.Add(new[] { Oracle.R(v[0]), Oracle.R(v[1]), Oracle.R(v[2]), Oracle.R(v[3]), lu.ToString(), Oracle.Csv(text) });
            }
            Oracle.WriteCsv(Path.Combine(outDir, "pixel_size.csv"), new[] { "world1", "world2", "pixel1", "pixel2", "unit", "output" }, rows);
        }

        /// <summary>
        /// Conversion table over every magnitude and every value of one unit enum.
        /// </summary>
        private static void Table1<T>(string outDir, string file, List<double> magnitudes, Func<double, T, double> convert)
        {
            List<IList<string>> rows = new List<IList<string>>();
            foreach (double m in magnitudes)
                foreach (T u in Enum.GetValues(typeof(T)))
                    rows.Add(new[] { Oracle.R(m), u.ToString(), Oracle.R(convert(m, u)) });
            Oracle.WriteCsv(Path.Combine(outDir, file), new[] { "input", "unit", "output" }, rows);
        }

        /// <summary>
        /// Conversion table over every magnitude and the cross product of two unit enums.
        /// </summary>
        private static void Table2<TA, TB>(string outDir, string file, string colA, string colB, List<double> magnitudes, Func<double, TA, TB, double> convert)
        {
            List<IList<string>> rows = new List<IList<string>>();
            foreach (double m in magnitudes)
                foreach (TA a in Enum.GetValues(typeof(TA)))
                    foreach (TB b in Enum.GetValues(typeof(TB)))
                        rows.Add(new[] { Oracle.R(m), a.ToString(), b.ToString(), Oracle.R(convert(m, a, b)) });
            Oracle.WriteCsv(Path.Combine(outDir, file), new[] { "input", colA, colB, "output" }, rows);
        }

        private static void Abbreviations<T>(List<IList<string>> rows, string kind, Func<T, string> abbreviation)
        {
            foreach (T u in Enum.GetValues(typeof(T)))
                rows.Add(new[] { kind, u.ToString(), abbreviation(u) });
        }

        private static void WriteAbbreviations(string path)
        {
            List<IList<string>> rows = new List<IList<string>>();
            Abbreviations<LengthUnit>(rows, "Length", UnitHelper.LengthAbbreviation);
            Abbreviations<SpeedUnit>(rows, "Speed", UnitHelper.SpeedAbbreviation);
            Abbreviations<AccelerationUnit>(rows, "Acceleration", UnitHelper.AccelerationAbbreviation);
            Abbreviations<AngleUnit>(rows, "Angle", UnitHelper.AngleAbbreviation);
            Abbreviations<AngularVelocityUnit>(rows, "AngularVelocity", UnitHelper.AngularVelocityAbbreviation);
            Abbreviations<AngularAccelerationUnit>(rows, "AngularAcceleration", UnitHelper.AngularAccelerationAbbreviation);
            Abbreviations<CadenceUnit>(rows, "Cadence", UnitHelper.FrequencyAbbreviation);
            foreach (TimecodeFormat u in Enum.GetValues(typeof(TimecodeFormat)))
            {
                if (u != TimecodeFormat.Unknown && u != TimecodeFormat.NumberOfTimeCodeFormats)
                    rows.Add(new[] { "Time", u.ToString(), UnitHelper.TimeAbbreviation(u) });
            }
            Oracle.WriteCsv(path, new[] { "kind", "unit", "abbreviation" }, rows);
        }
    }
}
