using System;

namespace Kinovea.FixtureGen
{
    public partial class KvaFuzz
    {
        private static readonly int[] sizes = { 1, 2, 3, 4, 6, 8, 10, 12, 14, 18, 24, 30, 36 };
        private static readonly string[] fonts = { "8", "9", "10", "11", "12", "14", "16", "18", "20", "24", "28", "32", "36" };

        private void Drawing()
        {
            switch (random.Next(0, 9))
            {
                case 0: Open("Label"); Leaf("Text", Text(random.Next(1, 20))); Leaf("Position", Point()); Leaf("ArrowEnd", Point()); Style(Color("back color"), Font()); Fading(); break;
                case 1: Open("Angle"); Leaf("PointO", Point()); Leaf("PointA", Point()); Leaf("PointB", Point()); Leaf("Signed", Bool()); Leaf("CCW", Bool()); Leaf("Supplementary", Bool()); Style(Color("line color"), Font()); Fading(); break;
                case 2: Open("CrossMark"); Leaf("CenterPoint", Point()); Leaf("ShowAsDot", Bool()); Style(Color("back color")); Fading(); break;
                case 3: Open("Line"); Leaf("Start", Point()); Leaf("End", Point()); Style(Color("color"), Int("LineSize", "line size", sizes), Enum("Arrows", "arrows", "None", "StartArrow", "EndArrow", "DoubleArrow")); Fading(); break;
                case 4: Open("Circle"); Leaf("Origin", Point()); Leaf("Radius", random.Next(1, 200).ToString()); Leaf("Filled", Bool()); Style(Color("color"), Int("PenSize", "pen size", sizes)); Fading(); break;
                case 5: Open("Pencil"); PointList(random.Next(1, 40), false); Style(Color("color"), Int("PenSize", "pen size", sizes)); Fading(); break;
                case 6: Open("Plane"); Leaf("PointUpperLeft", Corner(0, 0)); Leaf("PointUpperRight", Corner(1, 0)); Leaf("PointLowerRight", Corner(1, 1)); Leaf("PointLowerLeft", Corner(0, 1)); Leaf("ShowGrid", Bool()); Style(Color("color")); Fading(); break;
                case 7: Open("Rectangle"); Leaf("PointUpperLeft", Corner(0, 0)); Leaf("PointUpperRight", Corner(1, 0)); Leaf("PointLowerRight", Corner(1, 1)); Leaf("PointLowerLeft", Corner(0, 1)); Leaf("Filled", Bool()); Style(Color("color"), Int("PenSize", "pen size", sizes)); Fading(); break;
                default: Open("Polyline"); PointList(random.Next(2, 12), true); Style(Color("color"), Int("LineSize", "line size", sizes)); Fading(); break;
            }
            sb.Append("</").Append(open).Append(">\n");
        }

        private string open;

        private void Open(string name)
        {
            open = name;
            sb.Append('<').Append(name).Append(">\n");
        }

        private string Corner(int right, int bottom)
        {
            double x = (right == 0 ? 0 : imageSize.Width / 2.0) + random.NextDouble() * imageSize.Width / 2.0;
            double y = (bottom == 0 ? 0 : imageSize.Height / 2.0) + random.NextDouble() * imageSize.Height / 2.0;
            return F(x) + ";" + F(y);
        }

        private void PointList(int count, bool keyed)
        {
            sb.Append("<PointList Count=\"").Append(count).Append("\">\n");
            for (int i = 0; i < count; i++)
                sb.Append(keyed ? "<Point key=\"" + i + "\">" : "<Point>").Append(Point()).Append("</Point>\n");
            sb.Append("</PointList>\n");
        }

        private void Style(params string[] elements)
        {
            sb.Append("<DrawingStyle>\n");
            foreach (string e in elements)
                sb.Append(e);
            sb.Append("</DrawingStyle>\n");
        }

        private string Color(string key)
        {
            return "<Color Key=\"" + key + "\"><Value>" + random.Next(256) + ";" + random.Next(256) + ";" + random.Next(256) + ";" + random.Next(256) + "</Value></Color>\n";
        }

        private string Font()
        {
            return "<FontSize Key=\"font size\"><Value>" + Pick(fonts) + "</Value></FontSize>\n";
        }

        private string Int(string element, string key, int[] options)
        {
            return "<" + element + " Key=\"" + key + "\"><Value>" + Pick(options) + "</Value></" + element + ">\n";
        }

        private string Enum(string element, string key, params string[] options)
        {
            return "<" + element + " Key=\"" + key + "\"><Value>" + Pick(options) + "</Value></" + element + ">\n";
        }

        private void Fading()
        {
            sb.Append("<InfosFading>\n");
            Leaf("Enabled", Bool());
            Leaf("UseDefault", Bool());
            Leaf("AlwaysVisible", Bool());
            Leaf("OpaqueFrames", random.Next(1, 10).ToString());
            Leaf("Frames", random.Next(1, 100).ToString());
            sb.Append("</InfosFading>\n");
        }

        private void Chrono()
        {
            long visible = Time(), start = visible + random.Next(0, 1000), stop = start + random.Next(0, 5000), invisible = stop + random.Next(0, 5000);
            Open("Chrono"); Leaf("Position", Point());
            sb.Append("<Values>\n"); Leaf("Visible", visible.ToString()); Leaf("Invisible", invisible.ToString()); Leaf("StartCounting", start.ToString()); Leaf("StopCounting", stop.ToString()); Leaf("ClockOrigin", "-1"); sb.Append("</Values>\n");
            sb.Append("<Label>\n"); Leaf("Text", Text(6)); Leaf("Show", Bool()); sb.Append("</Label>\n");
            Style(Color("color"), Font());
            sb.Append("</Chrono>\n");
        }

        private void Track()
        {
            long begin = Time();
            int count = random.Next(2, 30);
            Open("Track"); Leaf("TimePosition", begin.ToString()); Leaf("Visible", begin.ToString()); Leaf("Invisible", "-1"); Leaf("ExtraData", Pick(new[] { "None", "Name", "Speed", "TotalDistance" }));
            sb.Append("<TrackPointList Count=\"").Append(count).Append("\">\n");
            double x = random.NextDouble() * imageSize.Width, y = random.NextDouble() * imageSize.Height;
            for (int i = 0; i < count; i++)
            {
                x += random.NextDouble() * 20 - 10; y += random.NextDouble() * 20 - 10;
                sb.Append("<TrackPoint>").Append(F(x)).Append(';').Append(F(y)).Append(';').Append(begin + i).Append("</TrackPoint>\n");
            }
            sb.Append("</TrackPointList>\n");
            Style(Color("color"), Int("LineSize", "line size", sizes), Enum("TrackShape", "track shape", "Solid;false", "Dash;false", "Solid;true", "Dash;true"));
            sb.Append("<MainLabel Text=\"").Append(Text(6)).Append("\">\n"); Leaf("SpacePosition", Point()); Leaf("TimePosition", "0"); sb.Append("</MainLabel>\n");
            sb.Append("</Track>\n");
        }

        private void Spotlight()
        {
            Open("Spotlight"); Leaf("Time", Time().ToString()); Leaf("Center", Point()); Leaf("Radius", random.Next(5, 100).ToString()); sb.Append("</Spotlight>\n");
        }

        private void Numbers(int count)
        {
            if (count == 0)
                return;
            sb.Append("<NumberSequence>\n");
            Style(Color("back color"), Font());
            for (int i = 0; i < count; i++)
            {
                Open("AutoNumber"); Leaf("Time", Time().ToString()); Leaf("Location", Point()); Leaf("Value", (i + 1).ToString()); sb.Append("</AutoNumber>\n");
            }
            sb.Append("</NumberSequence>\n");
        }
    }
}
