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
        /// worker; it is reported "Blocked" once the worker sits in Monitor.Wait (nothing else can make it wait
        /// here: no other thread holds the lock). It is then released with InterruptAdd, which leaves the frame
        /// out, and ResetInterruptAdd sets the flag back to false (it was false: an interrupted add returns at once).
        /// </summary>
        private string Add(VideoFrame frame)
        {
            if (cache != null)
                return cache.Add(frame).ToString();

            CacheAddResult result = CacheAddResult.Added;
            Thread worker = new Thread(() => result = prebuffer.Add(frame));
            worker.Start();
            while (!worker.Join(10))
            {
                if ((worker.ThreadState & ThreadState.WaitSleepJoin) == 0)
                    continue;

                prebuffer.InterruptAdd();
                worker.Join();
                prebuffer.ResetInterruptAdd();
                return "Blocked";
            }

            return result.ToString();
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
