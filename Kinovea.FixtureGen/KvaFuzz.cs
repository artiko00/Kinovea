using System;
using System.Drawing;
using System.Text;

namespace Kinovea.FixtureGen
{
    /// <summary>
    /// Deterministic random KVA 2.1 documents: a few keyframes with random attached drawings,
    /// chronos, a track, spotlights and numbers. Everything derives from the injected Random.
    /// </summary>
    public partial class KvaFuzz
    {
        private readonly Random random;
        private readonly StringBuilder sb = new StringBuilder();
        private Size imageSize;
        private const long Duration = 100000;

        public KvaFuzz(Random random)
        {
            this.random = random;
        }

        public string Create()
        {
            imageSize = new Size(random.Next(320, 1921), random.Next(240, 1081));
            sb.Append("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<KinoveaVideoAnalysis>\n");
            Leaf("FormatVersion", "2.1");
            Leaf("Producer", "KvaFuzz");
            Leaf("OriginalFilename", Text(8));
            Leaf("GlobalTitle", Text(12));
            Leaf("ImageSize", imageSize.Width + ";" + imageSize.Height);
            Leaf("AverageTimeStampsPerFrame", "1");
            Leaf("CaptureFramerate", F(1 + random.NextDouble() * 600));
            Leaf("FirstTimeStamp", "0");
            Leaf("SelectionStart", "0");
            Keyframes(random.Next(0, 5));
            Section("Chronos", random.Next(0, 3), Chrono);
            Section("Tracks", random.Next(0, 2), Track);
            Section("Spotlights", random.Next(0, 3), Spotlight);
            Numbers(random.Next(0, 3));
            sb.Append("</KinoveaVideoAnalysis>");
            return sb.ToString();
        }

        private void Keyframes(int count)
        {
            if (count == 0)
                return;
            long[] times = new long[count];
            for (int i = 0; i < count; i++)
                times[i] = random.Next((int)Duration);
            Array.Sort(times);
            sb.Append("<Keyframes>\n");
            foreach (long t in times)
            {
                sb.Append("<Keyframe>\n");
                Leaf("Timestamp", t.ToString(Oracle.Culture));
                Leaf("Name", Text(10));
                int drawings = random.Next(0, 6);
                if (drawings > 0)
                {
                    sb.Append("<Drawings>\n");
                    for (int i = 0; i < drawings; i++)
                        Drawing();
                    sb.Append("</Drawings>\n");
                }
                sb.Append("</Keyframe>\n");
            }
            sb.Append("</Keyframes>\n");
        }

        private void Section(string name, int count, Action item)
        {
            if (count == 0)
                return;
            sb.Append('<').Append(name).Append(">\n");
            for (int i = 0; i < count; i++)
                item();
            sb.Append("</").Append(name).Append(">\n");
        }

        // Primitive writers.
        private void Leaf(string name, string value)
        {
            sb.Append('<').Append(name).Append('>').Append(value).Append("</").Append(name).Append(">\n");
        }

        private string Text(int length)
        {
            const string valid = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
            StringBuilder s = new StringBuilder(length);
            for (int i = 0; i < length; i++)
                s.Append(valid[random.Next(valid.Length)]);
            return s.ToString();
        }

        private string F(double value)
        {
            return ((float)value).ToString(Oracle.Culture);
        }

        private string Point()
        {
            return F(random.NextDouble() * imageSize.Width) + ";" + F(random.NextDouble() * imageSize.Height);
        }

        private string Bool()
        {
            return random.NextDouble() < 0.5 ? "true" : "false";
        }

        private T Pick<T>(T[] options)
        {
            return options[random.Next(options.Length)];
        }

        private long Time()
        {
            return random.Next((int)Duration);
        }
    }
}
