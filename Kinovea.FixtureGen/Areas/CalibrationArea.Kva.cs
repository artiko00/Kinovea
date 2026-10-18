using System;
using System.IO;
using System.Xml;
using Kinovea.ScreenManager;
using Newtonsoft.Json.Linq;

namespace Kinovea.FixtureGen.Areas
{
    public partial class CalibrationArea
    {
        /// <summary>
        /// The exact `Calibration` fragment written by CalibrationHelper.WriteXml for a line, a
        /// plane with origin and offset, and the same plane with lens distortion.
        /// </summary>
        private void Kva(FixtureCase c, string outDir)
        {
            Guid id = Guid.Parse(c.Input("drawingId", Guid.Empty.ToString()));
            JToken line = c.Inputs["line"];
            JToken plane = c.Inputs["plane"];
            JObject planeDistortion = (JObject)plane.DeepClone();
            planeDistortion["distortion"] = c.Inputs["distortion"];

            WriteFragment(Path.Combine(outDir, "line.xml"), Build(line, 25, id));
            WriteFragment(Path.Combine(outDir, "plane.xml"), Build(plane, 25, id));
            WriteFragment(Path.Combine(outDir, "plane-distortion.xml"), Build(planeDistortion, 25, id));
        }

        private static void WriteFragment(string path, CalibrationHelper h)
        {
            XmlWriterSettings settings = new XmlWriterSettings();
            settings.Indent = true;
            settings.CloseOutput = true;
            using (XmlWriter w = XmlWriter.Create(path, settings))
            {
                w.WriteStartElement("Calibration");
                h.WriteXml(w);
                w.WriteEndElement();
            }
            Oracle.NormalizeTextFile(path);
        }
    }
}
