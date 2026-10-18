using System.Collections.Generic;
using System.IO;
using Kinovea.ScreenManager;
using Kinovea.Services;

namespace Kinovea.FixtureGen.Areas
{
    public partial class KinematicsArea
    {
        private static readonly Kinematics[] linearComponents =
        {
            Kinematics.XRaw, Kinematics.YRaw, Kinematics.X, Kinematics.Y,
            Kinematics.LinearDistance, Kinematics.LinearHorizontalDisplacement, Kinematics.LinearVerticalDisplacement,
            Kinematics.LinearSpeed, Kinematics.LinearHorizontalVelocity, Kinematics.LinearVerticalVelocity,
            Kinematics.LinearAcceleration, Kinematics.LinearHorizontalAcceleration, Kinematics.LinearVerticalAcceleration,
        };

        private static readonly Kinematics[] angularComponents =
        {
            Kinematics.AngularPosition, Kinematics.AngularDisplacement, Kinematics.TotalAngularDisplacement,
            Kinematics.AngularVelocity, Kinematics.TangentialVelocity,
            Kinematics.AngularAcceleration, Kinematics.TangentialAcceleration, Kinematics.CentripetalAcceleration,
            Kinematics.ResultantLinearAcceleration,
        };

        private static FilteredTrajectory Trajectory(List<TimedPoint> samples, CalibrationHelper calibration)
        {
            FilteredTrajectory t = new FilteredTrajectory();
            t.Initialize(samples, calibration);
            return t;
        }

        private static void WriteSeries(string path, TimeSeriesCollection tsc, Kinematics[] components)
        {
            List<string> header = new List<string> { "time" };
            foreach (Kinematics k in components)
                header.Add(k.ToString());

            List<IList<string>> rows = new List<IList<string>>();
            for (int i = 0; i < tsc.Length; i++)
            {
                List<string> row = new List<string> { tsc.Times[i].ToString(Oracle.Culture) };
                foreach (Kinematics k in components)
                    row.Add(Oracle.R(tsc[k][i]));
                rows.Add(row);
            }
            Oracle.WriteCsv(path, header, rows);
        }
    }
}
