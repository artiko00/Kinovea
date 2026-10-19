using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using Kinovea.Video;

namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// Replays a script of operations on the original frame containers (Cache, PreBuffer2) with null images
    /// and a no-op disposer, and records the state after each step.
    /// inputs: { "container": "cache"|"prebuffer", "capacity": 32, "retention": 8, "tolerance": 0.5,
    ///           "farAhead": 50, "tsPerFrame": 1, "ops": ["add 0", "acquireClosest 3", ...] }
    /// steps.csv: op, result, current (-1 when none), snapshot ("version|[a;b] [c;d]"), frames (timestamps).
    /// </summary>
    public partial class VideoCacheArea : IFixtureArea
    {
        public string Name { get { return "videocache"; } }

        private Cache cache;
        private PreBuffer2 prebuffer;
        private long tsPerFrame;

        public void Generate(FixtureCase fixtureCase, string outDir)
        {
            tsPerFrame = fixtureCase.Input("tsPerFrame", 1L);
            Setup(fixtureCase);

            List<IList<string>> rows = new List<IList<string>>();
            foreach (string op in fixtureCase.Input("ops", new List<string>()))
            {
                string result = Run(op.Split(' '));
                rows.Add(new[] { op, result, Current(), Snapshot(), Frames() });
            }

            Oracle.WriteCsv(Path.Combine(outDir, "steps.csv"), new[] { "op", "result", "current", "snapshot", "frames" }, rows);
            if (prebuffer != null)
                prebuffer.Shutdown();
        }

        private void Setup(FixtureCase fixtureCase)
        {
            VideoFrameDisposer disposer = frame => { };
            cache = null;
            prebuffer = null;
            if (fixtureCase.Input("container", "cache") == "cache")
            {
                cache = new Cache(disposer);
                cache.Tolerance = fixtureCase.Input("tolerance", 0.0);
                return;
            }

            prebuffer = new PreBuffer2(disposer);
            prebuffer.Tolerance = fixtureCase.Input("tolerance", 0.0);
            prebuffer.FarAheadThreshold = fixtureCase.Input("farAhead", 0.0);
            prebuffer.RetentionWindow = fixtureCase.Input("retention", 8);

            // The capacity has no setter (always 32 in the player); small capacities keep the scripts short.
            FieldInfo capacity = typeof(PreBuffer2).GetField("capacity", BindingFlags.NonPublic | BindingFlags.Instance);
            capacity.SetValue(prebuffer, fixtureCase.Input("capacity", 32));
        }

        private IVideoFramesContainer Container
        {
            get { return cache != null ? (IVideoFramesContainer)cache : prebuffer; }
        }

        private string Current()
        {
            VideoFrame current = Container.CurrentFrame;
            return current == null ? "-1" : current.Timestamp.ToString(Oracle.Culture);
        }

        private string Snapshot()
        {
            CacheSnapshot snapshot = Container.Snapshot;
            return snapshot.Version.ToString(Oracle.Culture) + "|" + snapshot.ToString().TrimEnd();
        }

        private string Frames()
        {
            // The containers keep their frames private; the dense cache exposes them, the prebuffer through reflection.
            IEnumerable<VideoFrame> frames = cache != null
                ? cache.Frames
                : ((SortedList<long, VideoFrame>)typeof(PreBuffer2).GetField("frames", BindingFlags.NonPublic | BindingFlags.Instance).GetValue(prebuffer)).Values;
            return string.Join(" ", frames.Select(f => f.Timestamp.ToString(Oracle.Culture)));
        }
    }
}
