using System;
using System.Threading;
using Kinovea.Video;

namespace Kinovea.FixtureGen.Areas
{
    public partial class VideoCacheArea
    {
        /// <summary>
        /// Runs one operation and returns its result. Operations a container does not have, or that throw,
        /// return "exception:&lt;type&gt;".
        /// </summary>
        private string Run(string[] op)
        {
            try
            {
                return Dispatch(op);
            }
            catch (Exception e)
            {
                return "exception:" + e.GetType().Name;
            }
        }

        private string Dispatch(string[] op)
        {
            switch (op[0])
            {
                case "add":
                    return Add(MakeFrame(op));
                case "forceAdd":
                    return Container.ForceAdd(MakeFrame(op)).ToString();
                case "acquireClosest":
                    Container.AcquireClosest(Long(op, 1));
                    return "";
                case "acquireNext":
                    return Container.AcquireNext().ToString();
                case "clear":
                    Container.Clear();
                    return "";
                case "shutdown":
                    Container.Shutdown();
                    return "";
                case "trim":
                    cache.Trim(Long(op, 1), Long(op, 2));
                    return "";
                case "workingZone":
                    return cache.WorkingZone.Start.ToString(Oracle.Culture) + ";" + cache.WorkingZone.End.ToString(Oracle.Culture);
                case "tryAcquire":
                    return TryAcquire((PlayerAction)Enum.Parse(typeof(PlayerAction), op[1]), Long(op, 2));
                case "relate":
                    return prebuffer.RelateTimestamp(Long(op, 1)).ToString();
                case "contiguous":
                    return prebuffer.IsContiguous().ToString();
                case "purge":
                    prebuffer.Purge();
                    return "";
                case "interrupt":
                    prebuffer.InterruptAdd();
                    return "";
                case "resetInterrupt":
                    prebuffer.ResetInterruptAdd();
                    return "";
                default:
                    throw new ArgumentException("unknown op " + op[0]);
            }
        }

        /// <summary>
        /// A prebuffer add blocks while the buffer is full. The replay is single-threaded, so the add runs on a
        /// worker: if it has not returned after a while it is reported "Blocked", then released with
        /// InterruptAdd (leaving the frame out) and the interrupt flag is reset to its previous value (false).
        /// </summary>
        private string Add(VideoFrame frame)
        {
            if (cache != null)
                return cache.Add(frame).ToString();

            CacheAddResult result = CacheAddResult.Added;
            Thread worker = new Thread(() => result = prebuffer.Add(frame));
            worker.Start();
            if (worker.Join(1000))
                return result.ToString();

            prebuffer.InterruptAdd();
            worker.Join();
            prebuffer.ResetInterruptAdd();
            return "Blocked";
        }

        private string TryAcquire(PlayerAction action, long reference)
        {
            TryAcquireResult r = prebuffer.TryAcquire(new PlayerState(0, action, false, reference));
            return r.TargetAcquired + ";" + r.AcquiredTimestamp.ToString(Oracle.Culture) + ";" + r.IsContiguous;
        }

        /// <summary>
        /// "add t [previous]": the previous timestamp defaults to t - tsPerFrame.
        /// </summary>
        private VideoFrame MakeFrame(string[] op)
        {
            long t = Long(op, 1);
            long previous = op.Length > 2 ? Long(op, 2) : t - tsPerFrame;
            return new VideoFrame(null, t, previous);
        }

        private static long Long(string[] op, int index)
        {
            return long.Parse(op[index], Oracle.Culture);
        }
    }
}
